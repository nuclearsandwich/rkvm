mod caps;

pub use caps::{AbsCaps, KeyCaps, RelCaps};

use crate::abs::{AbsAxis, AbsEvent, ToolType};
use crate::event::Event;
use crate::glue::{self, libevdev};
use crate::key::{Key, KeyEvent};
use crate::rel::{RelAxis, RelEvent};
use crate::sync::SyncEvent;
use crate::writer::Writer;

use std::collections::VecDeque;
use std::ffi::CStr;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind};
use std::mem::MaybeUninit;
use std::os::unix::prelude::{AsRawFd, OpenOptionsExt};
use std::path::Path;
use std::ptr::NonNull;
use thiserror::Error;
use tokio::io::unix::AsyncFd;
use tokio::task;

pub struct Interceptor {
    file: AsyncFd<File>,
    evdev: NonNull<libevdev>,
    writer: Writer,
    // The state of `read` is stored here to make it cancel safe.
    events: VecDeque<Event>,
    writing: Option<(u16, u16, i32)>,
    dropped: bool,
}

impl Interceptor {
    pub async fn read(&mut self) -> Result<Event, Error> {
        if let Some((r#type, code, value)) = self.writing {
            log::trace!("Resuming interrupted write");

            self.writer.write_raw(r#type, code, value).await?;
            self.writing = None;
        }

        while !matches!(self.events.back(), Some(Event::Sync(SyncEvent::All))) {
            let (r#type, code, value) = self.read_raw().await?;
            let event = match r#type as _ {
                glue::EV_REL if !self.dropped => {
                    RelAxis::from_raw(code).map(|axis| Event::Rel(RelEvent { axis, value }))
                }
                glue::EV_ABS if !self.dropped => match code as _ {
                    glue::ABS_MT_TOOL_TYPE => {
                        ToolType::from_raw(value).map(|value| AbsEvent::MtToolType { value })
                    }
                    _ => AbsAxis::from_raw(code).map(|axis| AbsEvent::Axis { axis, value }),
                }
                .map(Event::Abs),
                glue::EV_KEY if !self.dropped && (value == 0 || value == 1) => Key::from_raw(code)
                    .map(|key| {
                        Event::Key(KeyEvent {
                            key,
                            down: value == 1,
                        })
                    }),
                glue::EV_SYN => match code as _ {
                    glue::SYN_REPORT => {
                        if self.dropped {
                            self.dropped = false;
                            continue;
                        }

                        Some(Event::Sync(SyncEvent::All))
                    }
                    glue::SYN_DROPPED => {
                        log::warn!(
                            "Dropped {} event{}",
                            self.events.len(),
                            if self.events.len() == 1 { "" } else { "s" }
                        );

                        self.events.clear();
                        self.dropped = true;
                        continue;
                    }
                    glue::SYN_MT_REPORT if !self.dropped => Some(Event::Sync(SyncEvent::Mt)),
                    _ => continue,
                },
                _ => None,
            };

            if let Some(event) = event {
                self.events.push_back(event);
                continue;
            }

            self.writing = Some((r#type, code, value));
            self.writer.write_raw(r#type, code, value).await?;
            self.writing = None;
        }

        Ok(self.events.pop_front().unwrap())
    }

    pub async fn write(&mut self, event: &Event) -> Result<(), Error> {
        self.writer.write(event).await
    }

    pub fn name(&self) -> &CStr {
        let name = unsafe { glue::libevdev_get_name(self.evdev.as_ptr()) };
        let name = unsafe { CStr::from_ptr(name) };

        name
    }

    pub fn vendor(&self) -> u16 {
        unsafe { glue::libevdev_get_id_vendor(self.evdev.as_ptr()) as _ }
    }

    pub fn product(&self) -> u16 {
        unsafe { glue::libevdev_get_id_product(self.evdev.as_ptr()) as _ }
    }

    pub fn version(&self) -> u16 {
        unsafe { glue::libevdev_get_id_version(self.evdev.as_ptr()) as _ }
    }

    pub fn rel(&self) -> RelCaps {
        RelCaps::new(self)
    }

    pub fn abs(&self) -> AbsCaps {
        AbsCaps::new(self)
    }

    pub fn key(&self) -> KeyCaps {
        KeyCaps::new(self)
    }

    async fn read_raw(&mut self) -> Result<(u16, u16, i32), Error> {
        loop {
            let result = self.file.readable().await?.try_io(|_| {
                let mut event = MaybeUninit::uninit();
                let ret = unsafe {
                    glue::libevdev_next_event(
                        self.evdev.as_ptr(),
                        glue::libevdev_read_flag_LIBEVDEV_READ_FLAG_NORMAL,
                        event.as_mut_ptr(),
                    )
                };

                if ret < 0 {
                    // ENODEV means that the device got disconnected.
                    // However, ErrorKind doesn't have support for it yet,
                    // so translate to BrokenPipe here to not introduce
                    // platform specific code to rkvm-server.
                    let err = if ret == -libc::ENODEV {
                        Error::new(ErrorKind::BrokenPipe, "Device disconnected")
                    } else {
                        Error::from_raw_os_error(-ret)
                    };

                    return Err(err);
                }

                let event = unsafe { event.assume_init() };
                Ok((event.type_, event.code, event.value))
            });

            match result {
                Ok(result) => return result,
                Err(_) => continue, // This means it would block.
            }
        }
    }

    pub(crate) async fn open(path: &Path) -> Result<Self, OpenError> {
        let path = path.to_owned();
        task::spawn_blocking(move || Self::open_sync(&path))
            .await
            .map_err(|err| OpenError::Io(err.into()))?
    }

    fn open_sync(path: &Path) -> Result<Self, OpenError> {
        let file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(path)
            .and_then(AsyncFd::new)?;

        let mut evdev = MaybeUninit::uninit();

        let ret = unsafe { glue::libevdev_new_from_fd(file.as_raw_fd(), evdev.as_mut_ptr()) };
        if ret < 0 {
            return Err(Error::from_raw_os_error(-ret).into());
        }

        let evdev = unsafe { evdev.assume_init() };
        let evdev = NonNull::new(evdev).unwrap();

        // "Upon binding to a device or resuming from suspend, a driver must report
        // the current switch state. This ensures that the device, kernel, and userspace
        // state is in sync."
        // We have no way of knowing that.
        let sw = unsafe { glue::libevdev_has_event_type(evdev.as_ptr(), glue::EV_SW) };

        // Check if we're not opening our own virtual device.
        let bus_type = unsafe { glue::libevdev_get_id_bustype(evdev.as_ptr()) };

        if bus_type == glue::BUS_VIRTUAL as _ || sw == 1 {
            unsafe {
                glue::libevdev_free(evdev.as_ptr());
            }

            return Err(OpenError::NotAppliable);
        }

        unsafe {
            glue::libevdev_set_id_bustype(evdev.as_ptr(), glue::BUS_VIRTUAL as _);
        }

        let ret =
            unsafe { glue::libevdev_grab(evdev.as_ptr(), glue::libevdev_grab_mode_LIBEVDEV_GRAB) };

        if ret < 0 {
            unsafe {
                glue::libevdev_free(evdev.as_ptr());
            }

            return Err(Error::from_raw_os_error(-ret).into());
        }

        let writer = unsafe { Writer::from_evdev(evdev.as_ptr()) };
        let writer = match writer {
            Ok(writer) => writer,
            Err(err) => {
                unsafe {
                    glue::libevdev_free(evdev.as_ptr());
                }

                return Err(err.into());
            }
        };

        Ok(Self {
            file,
            evdev,
            writer,
            events: VecDeque::new(),
            dropped: false,
            writing: None,
        })
    }
}

impl Drop for Interceptor {
    fn drop(&mut self) {
        unsafe {
            glue::libevdev_free(self.evdev.as_ptr());
        }
    }
}

unsafe impl Send for Interceptor {}

#[derive(Error, Debug)]
pub(crate) enum OpenError {
    #[error("Not appliable")]
    NotAppliable,
    #[error(transparent)]
    Io(#[from] Error),
}
