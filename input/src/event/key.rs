use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Serialize, Deserialize, Hash)]
pub enum Key {
    A,
    Ab,
    AddressBook,
    Again,
    AlsToggle,
    AltErase,
    Angle,
    Apostrophe,
    Appselect,
    Archive,
    AspectRatio,
    Assistant,
    AttendantOff,
    AttendantOn,
    AttendantToggle,
    Audio,
    AudioDesc,
    Aux,
    B,
    Back,
    Backslash,
    Backspace,
    BassBoost,
    Battery,
    Blue,
    Bluetooth,
    Bookmarks,
    Break,
    BrightnessAuto,
    BrightnessCycle,
    BrightnessMax,
    BrightnessMin,
    BrightnessToggle,
    BrightnessZero,
    BrightnessDown,
    BrightnessUp,
    BrlDot1,
    BrlDot10,
    BrlDot2,
    BrlDot3,
    BrlDot4,
    BrlDot5,
    BrlDot6,
    BrlDot7,
    BrlDot8,
    BrlDot9,
    ButtonConfig,
    C,
    Calc,
    Calendar,
    Camera,
    CameraDown,
    CameraFocus,
    CameraLeft,
    CameraRight,
    CameraUp,
    CameraZoomIn,
    CameraZoomOut,
    Cancel,
    CapsLock,
    Cd,
    Channel,
    ChannelDown,
    ChannelUp,
    Chat,
    Clear,
    Close,
    CloseCd,
    Coffee,
    Comma,
    Compose,
    Computer,
    Config,
    Connect,
    ContextMenu,
    Controlpanel,
    Copy,
    Cut,
    CycleWindows,
    D,
    Dashboard,
    Data,
    Database,
    DelEol,
    DelEos,
    DelLine,
    Delete,
    DeleteFile,
    Digits,
    Direction,
    Directory,
    DisplayOff,
    DisplayToggle,
    Documents,
    Dollar,
    Dot,
    Down,
    Dvd,
    E,
    Edit,
    Editor,
    EjectCd,
    EjectCloseCd,
    Email,
    End,
    Enter,
    Epg,
    Equal,
    Esc,
    Euro,
    Exit,
    F,
    F1,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F2,
    F20,
    F21,
    F22,
    F23,
    F24,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    FastForward,
    FastReverse,
    Favorites,
    File,
    Finance,
    Find,
    First,
    Fn,
    Fn1,
    Fn2,
    FnB,
    FnD,
    FnE,
    FnEsc,
    FnF,
    FnF1,
    FnF10,
    FnF11,
    FnF12,
    FnF2,
    FnF3,
    FnF4,
    FnF5,
    FnF6,
    FnF7,
    FnF8,
    FnF9,
    FnS,
    Forward,
    ForwardMail,
    Frameback,
    FrameForward,
    Front,
    FullScreen,
    G,
    Games,
    Goto,
    GraphicsEditor,
    Grave,
    Green,
    H,
    Hangeul,
    Hanja,
    Help,
    Henkan,
    Hiragana,
    Home,
    Homepage,
    Hp,
    I,
    Images,
    Info,
    InsLine,
    Insert,
    Iso,
    J,
    Journal,
    K,
    Katakana,
    KatakanaHiragana,
    KbdLayoutNext,
    KbdLcdMenu1,
    KbdLcdMenu2,
    KbdLcdMenu3,
    KbdLcdMenu4,
    KbdLcdMenu5,
    KbdIllumDown,
    KbdIllumToggle,
    KbdIllumUp,
    KbdInputAssistAccept,
    KbdInputAssistCancel,
    KbdInputAssistNext,
    KbdInputAssistNextgroup,
    KbdInputAssistPrev,
    KbdInputAssistPrevgroup,
    Keyboard,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpAsterisk,
    KpComma,
    KpDott,
    KpEnter,
    KpEqual,
    KpJpComma,
    KpLeftParen,
    KpMinus,
    KpPlus,
    KpPlusMinus,
    KpRightParen,
    KpSlash,
    L,
    Language,
    Last,
    Left,
    LeftDown,
    LeftUp,
    LeftAlt,
    LeftBrace,
    LeftCtrl,
    LeftMeta,
    LeftShift,
    LightsToggle,
    LineFeed,
    List,
    LogOff,
    M,
    Macro,
    Macro1,
    Macro10,
    Macro11,
    Macro12,
    Macro13,
    Macro14,
    Macro15,
    Macro16,
    Macro17,
    Macro18,
    Macro19,
    Macro2,
    Macro20,
    Macro21,
    Macro22,
    Macro23,
    Macro24,
    Macro25,
    Macro26,
    Macro27,
    Macro28,
    Macro29,
    Macro3,
    Macro30,
    Macro4,
    Macro5,
    Macro6,
    Macro7,
    Macro8,
    Macro9,
    MacroPreset1,
    MacroPreset2,
    MacroPreset3,
    MacroPresetCycle,
    MacroRecordStart,
    MacroRecordStop,
    Mail,
    Media,
    MediaRepeat,
    MediaTopMenu,
    Memo,
    Menu,
    Messenger,
    Mhp,
    MicMute,
    Minus,
    Mode,
    Move,
    Mp3,
    MsDos,
    Muhenkan,
    Mute,
    N,
    N0,
    N1,
    N102nd,
    N10ChannelsDown,
    N10ChannelsUp,
    N2,
    N3,
    N3dMode,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    New,
    News,
    Next,
    NextFavorite,
    Nextsong,
    Numeric0,
    Numeric1,
    Numeric11,
    Numeric12,
    Numeric2,
    Numeric3,
    Numeric4,
    Numeric5,
    Numeric6,
    Numeric7,
    Numeric8,
    Numeric9,
    NumericA,
    NumericB,
    NumericC,
    NumericD,
    NumericPound,
    NumericStar,
    NumLock,
    O,
    Ok,
    OnscreenKeyboard,
    Open,
    Option,
    P,
    Pagedown,
    Pageup,
    Paste,
    Pause,
    PauseRecord,
    PauseCd,
    Pc,
    Phone,
    Play,
    PlayCd,
    Player,
    PlayPause,
    Power,
    Power2,
    Presentation,
    Previous,
    PreviousSong,
    Print,
    PrivacyScreenToggle,
    Prog1,
    Prog2,
    Prog3,
    Prog4,
    Program,
    Props,
    Pvr,
    Q,
    Question,
    R,
    Radio,
    Record,
    Red,
    Redo,
    Refresh,
    Reply,
    Reserved,
    Restart,
    Rewind,
    RfKill,
    Right,
    RightDown,
    RightUp,
    RightAlt,
    RightBrace,
    RightCtrl,
    RightMeta,
    RightShift,
    Ro,
    RootMenu,
    RotateDisplay,
    RotateLockToggle,
    S,
    Sat,
    Sat2,
    Save,
    Scale,
    Screen,
    Screenlock,
    Screensaver,
    ScrollDown,
    ScrollLock,
    ScrollUp,
    Search,
    Select,
    SelectiveScreenshot,
    Semicolon,
    Send,
    SendFile,
    Setup,
    Shop,
    Shuffle,
    Slash,
    Sleep,
    Slow,
    SlowReverse,
    Sound,
    Space,
    Spellcheck,
    Sport,
    Spreadsheet,
    Stop,
    StopRecord,
    StopCd,
    Subtitle,
    Suspend,
    SwitchVideoMode,
    SysRq,
    T,
    Tab,
    Tape,
    TaskManager,
    Teen,
    Text,
    Time,
    Title,
    TouchpadOff,
    TouchpadOn,
    TouchpadToggle,
    Tuner,
    Tv,
    Tv2,
    Twen,
    U,
    Undo,
    Unknown,
    Unmute,
    Up,
    Uwb,
    V,
    Vcr,
    Vcr2,
    Vendor,
    Video,
    VideoNext,
    VideoPrev,
    VideoPhone,
    Vod,
    VoiceCommand,
    VoiceMail,
    VolumeDown,
    VolumeUp,
    W,
    WakeUp,
    Wimax,
    Wlan,
    WordProcessor,
    WpsButton,
    Wwan,
    Www,
    X,
    Xfer,
    Y,
    Yellow,
    Yen,
    Z,
    ZenkakuHankaku,
    Zoom,
    ZoomIn,
    ZoomOut,
    ZoomReset,
}

impl Key {
    pub(crate) fn to_raw(&self) -> u16 {
        use Key::*;

        match *self {
            A => 0x001E,
            Ab => 0x0196,
            AddressBook => 0x01AD,
            Again => 0x0081,
            AlsToggle => 0x0230,
            AltErase => 0x00DE,
            Angle => 0x0173,
            Apostrophe => 0x0028,
            Appselect => 0x0244,
            Archive => 0x0169,
            AspectRatio => 0x0177,
            Assistant => 0x0247,
            AttendantOff => 0x021C,
            AttendantOn => 0x021B,
            AttendantToggle => 0x021D,
            Audio => 0x0188,
            AudioDesc => 0x026E,
            Aux => 0x0186,
            B => 0x0030,
            Back => 0x009E,
            Backslash => 0x002B,
            Backspace => 0x000E,
            BassBoost => 0x00D1,
            Battery => 0x00EC,
            Blue => 0x0191,
            Bluetooth => 0x00ED,
            Bookmarks => 0x009C,
            Break => 0x019B,
            BrightnessAuto => 0x00F4,
            BrightnessCycle => 0x00F3,
            BrightnessMax => 0x0251,
            BrightnessMin => 0x0250,
            BrightnessToggle => 0x01AF,
            BrightnessZero => 0x00F4,
            BrightnessDown => 0x00E0,
            BrightnessUp => 0x00E1,
            BrlDot1 => 0x01F1,
            BrlDot10 => 0x01FA,
            BrlDot2 => 0x01F2,
            BrlDot3 => 0x01F3,
            BrlDot4 => 0x01F4,
            BrlDot5 => 0x01F5,
            BrlDot6 => 0x01F6,
            BrlDot7 => 0x01F7,
            BrlDot8 => 0x01F8,
            BrlDot9 => 0x01F9,
            ButtonConfig => 0x0240,
            C => 0x002E,
            Calc => 0x008C,
            Calendar => 0x018D,
            Camera => 0x00D4,
            CameraDown => 0x0218,
            CameraFocus => 0x0210,
            CameraLeft => 0x0219,
            CameraRight => 0x021A,
            CameraUp => 0x0217,
            CameraZoomIn => 0x0215,
            CameraZoomOut => 0x0216,
            Cancel => 0x00DF,
            CapsLock => 0x003A,
            Cd => 0x017F,
            Channel => 0x016B,
            ChannelDown => 0x0193,
            ChannelUp => 0x0192,
            Chat => 0x00D8,
            Clear => 0x0163,
            Close => 0x00CE,
            CloseCd => 0x00A0,
            Coffee => 0x0098,
            Comma => 0x0033,
            Compose => 0x007F,
            Computer => 0x009D,
            Config => 0x00AB,
            Connect => 0x00DA,
            ContextMenu => 0x01B6,
            Controlpanel => 0x0243,
            Copy => 0x0085,
            Cut => 0x0089,
            CycleWindows => 0x009A,
            D => 0x0020,
            Dashboard => 0x00CC,
            Data => 0x0277,
            Database => 0x01AA,
            DelEol => 0x01C0,
            DelEos => 0x01C1,
            DelLine => 0x01C3,
            Delete => 0x006F,
            DeleteFile => 0x0092,
            Digits => 0x019D,
            Direction => 0x0099,
            Directory => 0x018A,
            DisplayOff => 0x00F5,
            DisplayToggle => 0x01AF,
            Documents => 0x00EB,
            Dollar => 0x01B2,
            Dot => 0x0034,
            Down => 0x006C,
            Dvd => 0x0185,
            E => 0x0012,
            Edit => 0x00B0,
            Editor => 0x01A6,
            EjectCd => 0x00A1,
            EjectCloseCd => 0x00A2,
            Email => 0x00D7,
            End => 0x006B,
            Enter => 0x001C,
            Epg => 0x016D,
            Equal => 0x000D,
            Esc => 0x0001,
            Euro => 0x01B3,
            Exit => 0x00AE,
            F => 0x0021,
            F1 => 0x003B,
            F10 => 0x0044,
            F11 => 0x0057,
            F12 => 0x0058,
            F13 => 0x00B7,
            F14 => 0x00B8,
            F15 => 0x00B9,
            F16 => 0x00BA,
            F17 => 0x00BB,
            F18 => 0x00BC,
            F19 => 0x00BD,
            F2 => 0x003C,
            F20 => 0x00BE,
            F21 => 0x00BF,
            F22 => 0x00C0,
            F23 => 0x00C1,
            F24 => 0x00C2,
            F3 => 0x003D,
            F4 => 0x003E,
            F5 => 0x003F,
            F6 => 0x0040,
            F7 => 0x0041,
            F8 => 0x0042,
            F9 => 0x0043,
            FastForward => 0x00D0,
            FastReverse => 0x0275,
            Favorites => 0x016C,
            File => 0x0090,
            Finance => 0x00DB,
            Find => 0x0088,
            First => 0x0194,
            Fn => 0x01D0,
            Fn1 => 0x01DE,
            Fn2 => 0x01DF,
            FnB => 0x01E4,
            FnD => 0x01E0,
            FnE => 0x01E1,
            FnEsc => 0x01D1,
            FnF => 0x01E2,
            FnF1 => 0x01D2,
            FnF10 => 0x01DB,
            FnF11 => 0x01DC,
            FnF12 => 0x01DD,
            FnF2 => 0x01D3,
            FnF3 => 0x01D4,
            FnF4 => 0x01D5,
            FnF5 => 0x01D6,
            FnF6 => 0x01D7,
            FnF7 => 0x01D8,
            FnF8 => 0x01D9,
            FnF9 => 0x01DA,
            FnS => 0x01E3,
            Forward => 0x009F,
            ForwardMail => 0x00E9,
            Frameback => 0x01B4,
            FrameForward => 0x01B5,
            Front => 0x0084,
            FullScreen => 0x0174,
            G => 0x0022,
            Games => 0x01A1,
            Goto => 0x0162,
            GraphicsEditor => 0x01A8,
            Grave => 0x0029,
            Green => 0x018F,
            H => 0x0023,
            Hangeul => 0x007A,
            Hanja => 0x007B,
            Help => 0x008A,
            Henkan => 0x005C,
            Hiragana => 0x005B,
            Home => 0x0066,
            Homepage => 0x00AC,
            Hp => 0x00D3,
            I => 0x0017,
            Images => 0x01BA,
            Info => 0x0166,
            InsLine => 0x01C2,
            Insert => 0x006E,
            Iso => 0x00AA,
            J => 0x0024,
            Journal => 0x0242,
            K => 0x0025,
            Katakana => 0x005A,
            KatakanaHiragana => 0x005D,
            KbdLayoutNext => 0x0248,
            KbdLcdMenu1 => 0x02B8,
            KbdLcdMenu2 => 0x02B9,
            KbdLcdMenu3 => 0x02BA,
            KbdLcdMenu4 => 0x02BB,
            KbdLcdMenu5 => 0x02BC,
            KbdIllumDown => 0x00E5,
            KbdIllumToggle => 0x00E4,
            KbdIllumUp => 0x00E6,
            KbdInputAssistAccept => 0x0264,
            KbdInputAssistCancel => 0x0265,
            KbdInputAssistNext => 0x0261,
            KbdInputAssistNextgroup => 0x0263,
            KbdInputAssistPrev => 0x0260,
            KbdInputAssistPrevgroup => 0x0262,
            Keyboard => 0x0176,
            Kp0 => 0x0052,
            Kp1 => 0x004F,
            Kp2 => 0x0050,
            Kp3 => 0x0051,
            Kp4 => 0x004B,
            Kp5 => 0x004C,
            Kp6 => 0x004D,
            Kp7 => 0x0047,
            Kp8 => 0x0048,
            Kp9 => 0x0049,
            KpAsterisk => 0x0037,
            KpComma => 0x0079,
            KpDott => 0x0053,
            KpEnter => 0x0060,
            KpEqual => 0x0075,
            KpJpComma => 0x005F,
            KpLeftParen => 0x00B3,
            KpMinus => 0x004A,
            KpPlus => 0x004E,
            KpPlusMinus => 0x0076,
            KpRightParen => 0x00B4,
            KpSlash => 0x0062,
            L => 0x0026,
            Language => 0x0170,
            Last => 0x0195,
            Left => 0x0069,
            LeftDown => 0x0269,
            LeftUp => 0x0268,
            LeftAlt => 0x0038,
            LeftBrace => 0x001A,
            LeftCtrl => 0x001D,
            LeftMeta => 0x007D,
            LeftShift => 0x002A,
            LightsToggle => 0x021E,
            LineFeed => 0x0065,
            List => 0x018B,
            LogOff => 0x01B1,
            M => 0x0032,
            Macro => 0x0070,
            Macro1 => 0x0290,
            Macro10 => 0x0299,
            Macro11 => 0x029A,
            Macro12 => 0x029B,
            Macro13 => 0x029C,
            Macro14 => 0x029D,
            Macro15 => 0x029E,
            Macro16 => 0x029F,
            Macro17 => 0x02A0,
            Macro18 => 0x02A1,
            Macro19 => 0x02A2,
            Macro2 => 0x0291,
            Macro20 => 0x02A3,
            Macro21 => 0x02A4,
            Macro22 => 0x02A5,
            Macro23 => 0x02A6,
            Macro24 => 0x02A7,
            Macro25 => 0x02A8,
            Macro26 => 0x02A9,
            Macro27 => 0x02AA,
            Macro28 => 0x02AB,
            Macro29 => 0x02AC,
            Macro3 => 0x0292,
            Macro30 => 0x02AD,
            Macro4 => 0x0293,
            Macro5 => 0x0294,
            Macro6 => 0x0295,
            Macro7 => 0x0296,
            Macro8 => 0x0297,
            Macro9 => 0x0298,
            MacroPreset1 => 0x02B3,
            MacroPreset2 => 0x02B4,
            MacroPreset3 => 0x02B5,
            MacroPresetCycle => 0x02B2,
            MacroRecordStart => 0x02B0,
            MacroRecordStop => 0x02B1,
            Mail => 0x009B,
            Media => 0x00E2,
            MediaRepeat => 0x01B7,
            MediaTopMenu => 0x026B,
            Memo => 0x018C,
            Menu => 0x008B,
            Messenger => 0x01AE,
            Mhp => 0x016F,
            MicMute => 0x00F8,
            Minus => 0x000C,
            Mode => 0x0175,
            Move => 0x00AF,
            Mp3 => 0x0187,
            MsDos => 0x0097,
            Muhenkan => 0x005E,
            Mute => 0x0071,
            N => 0x0031,
            N0 => 0x000B,
            N1 => 0x0002,
            N102nd => 0x0056,
            N10ChannelsDown => 0x01B9,
            N10ChannelsUp => 0x01B8,
            N2 => 0x0003,
            N3 => 0x0004,
            N3dMode => 0x026F,
            N4 => 0x0005,
            N5 => 0x0006,
            N6 => 0x0007,
            N7 => 0x0008,
            N8 => 0x0009,
            N9 => 0x000A,
            New => 0x00B5,
            News => 0x01AB,
            Next => 0x0197,
            NextFavorite => 0x0270,
            Nextsong => 0x00A3,
            Numeric0 => 0x0200,
            Numeric1 => 0x0201,
            Numeric11 => 0x026C,
            Numeric12 => 0x026D,
            Numeric2 => 0x0202,
            Numeric3 => 0x0203,
            Numeric4 => 0x0204,
            Numeric5 => 0x0205,
            Numeric6 => 0x0206,
            Numeric7 => 0x0207,
            Numeric8 => 0x0208,
            Numeric9 => 0x0209,
            NumericA => 0x020C,
            NumericB => 0x020D,
            NumericC => 0x020E,
            NumericD => 0x020F,
            NumericPound => 0x020B,
            NumericStar => 0x020A,
            NumLock => 0x0045,
            O => 0x0018,
            Ok => 0x0160,
            OnscreenKeyboard => 0x0278,
            Open => 0x0086,
            Option => 0x0165,
            P => 0x0019,
            Pagedown => 0x006D,
            Pageup => 0x0068,
            Paste => 0x0087,
            Pause => 0x0077,
            PauseRecord => 0x0272,
            PauseCd => 0x00C9,
            Pc => 0x0178,
            Phone => 0x00A9,
            Play => 0x00CF,
            PlayCd => 0x00C8,
            Player => 0x0183,
            PlayPause => 0x00A4,
            Power => 0x0074,
            Power2 => 0x0164,
            Presentation => 0x01A9,
            Previous => 0x019C,
            PreviousSong => 0x00A5,
            Print => 0x00D2,
            PrivacyScreenToggle => 0x0279,
            Prog1 => 0x0094,
            Prog2 => 0x0095,
            Prog3 => 0x00CA,
            Prog4 => 0x00CB,
            Program => 0x016A,
            Props => 0x0082,
            Pvr => 0x016E,
            Q => 0x0010,
            Question => 0x00D6,
            R => 0x0013,
            Radio => 0x0181,
            Record => 0x00A7,
            Red => 0x018E,
            Redo => 0x00B6,
            Refresh => 0x00AD,
            Reply => 0x00E8,
            Reserved => 0x0000,
            Restart => 0x0198,
            Rewind => 0x00A8,
            RfKill => 0x00F7,
            Right => 0x006A,
            RightDown => 0x0267,
            RightUp => 0x0266,
            RightAlt => 0x0064,
            RightBrace => 0x001B,
            RightCtrl => 0x0061,
            RightMeta => 0x007E,
            RightShift => 0x0036,
            Ro => 0x0059,
            RootMenu => 0x026A,
            RotateDisplay => 0x0099,
            RotateLockToggle => 0x0231,
            S => 0x001F,
            Sat => 0x017D,
            Sat2 => 0x017E,
            Save => 0x00EA,
            Scale => 0x0078,
            Screen => 0x0177,
            Screenlock => 0x0098,
            Screensaver => 0x0245,
            ScrollDown => 0x00B2,
            ScrollLock => 0x0046,
            ScrollUp => 0x00B1,
            Search => 0x00D9,
            Select => 0x0161,
            SelectiveScreenshot => 0x027A,
            Semicolon => 0x0027,
            Send => 0x00E7,
            SendFile => 0x0091,
            Setup => 0x008D,
            Shop => 0x00DD,
            Shuffle => 0x019A,
            Slash => 0x0035,
            Sleep => 0x008E,
            Slow => 0x0199,
            SlowReverse => 0x0276,
            Sound => 0x00D5,
            Space => 0x0039,
            Spellcheck => 0x01B0,
            Sport => 0x00DC,
            Spreadsheet => 0x01A7,
            Stop => 0x0080,
            StopRecord => 0x0271,
            StopCd => 0x00A6,
            Subtitle => 0x0172,
            Suspend => 0x00CD,
            SwitchVideoMode => 0x00E3,
            SysRq => 0x0063,
            T => 0x0014,
            Tab => 0x000F,
            Tape => 0x0180,
            TaskManager => 0x0241,
            Teen => 0x019E,
            Text => 0x0184,
            Time => 0x0167,
            Title => 0x0171,
            TouchpadOff => 0x0214,
            TouchpadOn => 0x0213,
            TouchpadToggle => 0x0212,
            Tuner => 0x0182,
            Tv => 0x0179,
            Tv2 => 0x017A,
            Twen => 0x019F,
            U => 0x0016,
            Undo => 0x0083,
            Unknown => 0x00F0,
            Unmute => 0x0274,
            Up => 0x0067,
            Uwb => 0x00EF,
            V => 0x002F,
            Vcr => 0x017B,
            Vcr2 => 0x017C,
            Vendor => 0x0168,
            Video => 0x0189,
            VideoNext => 0x00F1,
            VideoPrev => 0x00F2,
            VideoPhone => 0x01A0,
            Vod => 0x0273,
            VoiceCommand => 0x0246,
            VoiceMail => 0x01AC,
            VolumeDown => 0x0072,
            VolumeUp => 0x0073,
            W => 0x0011,
            WakeUp => 0x008F,
            Wimax => 0x00F6,
            Wlan => 0x00EE,
            WordProcessor => 0x01A5,
            WpsButton => 0x0211,
            Wwan => 0x00F6,
            Www => 0x0096,
            X => 0x002D,
            Xfer => 0x0093,
            Y => 0x0015,
            Yellow => 0x0190,
            Yen => 0x007C,
            Z => 0x002C,
            ZenkakuHankaku => 0x0055,
            Zoom => 0x0174,
            ZoomIn => 0x01A2,
            ZoomOut => 0x01A3,
            ZoomReset => 0x01A4,
        }
    }

    pub(crate) fn from_raw(code: u16) -> Option<Self> {
        use Key::*;

        // This is generated from linux headers, some patterns are unreachable, and we don't care.
        #[allow(unreachable_patterns)]
        let key = match code {
            0x001E => A,
            0x0196 => Ab,
            0x01AD => AddressBook,
            0x0081 => Again,
            0x0230 => AlsToggle,
            0x00DE => AltErase,
            0x0173 => Angle,
            0x0028 => Apostrophe,
            0x0244 => Appselect,
            0x0169 => Archive,
            0x0177 => AspectRatio,
            0x0247 => Assistant,
            0x021C => AttendantOff,
            0x021B => AttendantOn,
            0x021D => AttendantToggle,
            0x0188 => Audio,
            0x026E => AudioDesc,
            0x0186 => Aux,
            0x0030 => B,
            0x009E => Back,
            0x002B => Backslash,
            0x000E => Backspace,
            0x00D1 => BassBoost,
            0x00EC => Battery,
            0x0191 => Blue,
            0x00ED => Bluetooth,
            0x009C => Bookmarks,
            0x019B => Break,
            0x00F4 => BrightnessAuto,
            0x00F3 => BrightnessCycle,
            0x0251 => BrightnessMax,
            0x0250 => BrightnessMin,
            0x00E0 => BrightnessDown,
            0x00E1 => BrightnessUp,
            0x01F1 => BrlDot1,
            0x01FA => BrlDot10,
            0x01F2 => BrlDot2,
            0x01F3 => BrlDot3,
            0x01F4 => BrlDot4,
            0x01F5 => BrlDot5,
            0x01F6 => BrlDot6,
            0x01F7 => BrlDot7,
            0x01F8 => BrlDot8,
            0x01F9 => BrlDot9,
            0x0240 => ButtonConfig,
            0x002E => C,
            0x008C => Calc,
            0x018D => Calendar,
            0x00D4 => Camera,
            0x0218 => CameraDown,
            0x0210 => CameraFocus,
            0x0219 => CameraLeft,
            0x021A => CameraRight,
            0x0217 => CameraUp,
            0x0215 => CameraZoomIn,
            0x0216 => CameraZoomOut,
            0x00DF => Cancel,
            0x003A => CapsLock,
            0x017F => Cd,
            0x016B => Channel,
            0x0193 => ChannelDown,
            0x0192 => ChannelUp,
            0x00D8 => Chat,
            0x0163 => Clear,
            0x00CE => Close,
            0x00A0 => CloseCd,
            0x0098 => Coffee,
            0x0033 => Comma,
            0x007F => Compose,
            0x009D => Computer,
            0x00AB => Config,
            0x00DA => Connect,
            0x01B6 => ContextMenu,
            0x0243 => Controlpanel,
            0x0085 => Copy,
            0x0089 => Cut,
            0x009A => CycleWindows,
            0x0020 => D,
            0x00CC => Dashboard,
            0x0277 => Data,
            0x01AA => Database,
            0x01C0 => DelEol,
            0x01C1 => DelEos,
            0x01C3 => DelLine,
            0x006F => Delete,
            0x0092 => DeleteFile,
            0x019D => Digits,
            0x018A => Directory,
            0x00F5 => DisplayOff,
            0x01AF => DisplayToggle,
            0x00EB => Documents,
            0x01B2 => Dollar,
            0x0034 => Dot,
            0x006C => Down,
            0x0185 => Dvd,
            0x0012 => E,
            0x00B0 => Edit,
            0x01A6 => Editor,
            0x00A1 => EjectCd,
            0x00A2 => EjectCloseCd,
            0x00D7 => Email,
            0x006B => End,
            0x001C => Enter,
            0x016D => Epg,
            0x000D => Equal,
            0x0001 => Esc,
            0x01B3 => Euro,
            0x00AE => Exit,
            0x0021 => F,
            0x003B => F1,
            0x0044 => F10,
            0x0057 => F11,
            0x0058 => F12,
            0x00B7 => F13,
            0x00B8 => F14,
            0x00B9 => F15,
            0x00BA => F16,
            0x00BB => F17,
            0x00BC => F18,
            0x00BD => F19,
            0x003C => F2,
            0x00BE => F20,
            0x00BF => F21,
            0x00C0 => F22,
            0x00C1 => F23,
            0x00C2 => F24,
            0x003D => F3,
            0x003E => F4,
            0x003F => F5,
            0x0040 => F6,
            0x0041 => F7,
            0x0042 => F8,
            0x0043 => F9,
            0x00D0 => FastForward,
            0x0275 => FastReverse,
            0x016C => Favorites,
            0x0090 => File,
            0x00DB => Finance,
            0x0088 => Find,
            0x0194 => First,
            0x01D0 => Fn,
            0x01DE => Fn1,
            0x01DF => Fn2,
            0x01E4 => FnB,
            0x01E0 => FnD,
            0x01E1 => FnE,
            0x01D1 => FnEsc,
            0x01E2 => FnF,
            0x01D2 => FnF1,
            0x01DB => FnF10,
            0x01DC => FnF11,
            0x01DD => FnF12,
            0x01D3 => FnF2,
            0x01D4 => FnF3,
            0x01D5 => FnF4,
            0x01D6 => FnF5,
            0x01D7 => FnF6,
            0x01D8 => FnF7,
            0x01D9 => FnF8,
            0x01DA => FnF9,
            0x01E3 => FnS,
            0x009F => Forward,
            0x00E9 => ForwardMail,
            0x01B4 => Frameback,
            0x01B5 => FrameForward,
            0x0084 => Front,
            0x0174 => FullScreen,
            0x0022 => G,
            0x01A1 => Games,
            0x0162 => Goto,
            0x01A8 => GraphicsEditor,
            0x0029 => Grave,
            0x018F => Green,
            0x0023 => H,
            0x007A => Hangeul,
            0x007B => Hanja,
            0x008A => Help,
            0x005C => Henkan,
            0x005B => Hiragana,
            0x0066 => Home,
            0x00AC => Homepage,
            0x00D3 => Hp,
            0x0017 => I,
            0x01BA => Images,
            0x0166 => Info,
            0x01C2 => InsLine,
            0x006E => Insert,
            0x00AA => Iso,
            0x0024 => J,
            0x0242 => Journal,
            0x0025 => K,
            0x005A => Katakana,
            0x005D => KatakanaHiragana,
            0x0248 => KbdLayoutNext,
            0x02B8 => KbdLcdMenu1,
            0x02B9 => KbdLcdMenu2,
            0x02BA => KbdLcdMenu3,
            0x02BB => KbdLcdMenu4,
            0x02BC => KbdLcdMenu5,
            0x00E5 => KbdIllumDown,
            0x00E4 => KbdIllumToggle,
            0x00E6 => KbdIllumUp,
            0x0264 => KbdInputAssistAccept,
            0x0265 => KbdInputAssistCancel,
            0x0261 => KbdInputAssistNext,
            0x0263 => KbdInputAssistNextgroup,
            0x0260 => KbdInputAssistPrev,
            0x0262 => KbdInputAssistPrevgroup,
            0x0176 => Keyboard,
            0x0052 => Kp0,
            0x004F => Kp1,
            0x0050 => Kp2,
            0x0051 => Kp3,
            0x004B => Kp4,
            0x004C => Kp5,
            0x004D => Kp6,
            0x0047 => Kp7,
            0x0048 => Kp8,
            0x0049 => Kp9,
            0x0037 => KpAsterisk,
            0x0079 => KpComma,
            0x0053 => KpDott,
            0x0060 => KpEnter,
            0x0075 => KpEqual,
            0x005F => KpJpComma,
            0x00B3 => KpLeftParen,
            0x004A => KpMinus,
            0x004E => KpPlus,
            0x0076 => KpPlusMinus,
            0x00B4 => KpRightParen,
            0x0062 => KpSlash,
            0x0026 => L,
            0x0170 => Language,
            0x0195 => Last,
            0x0069 => Left,
            0x0269 => LeftDown,
            0x0268 => LeftUp,
            0x0038 => LeftAlt,
            0x001A => LeftBrace,
            0x001D => LeftCtrl,
            0x007D => LeftMeta,
            0x002A => LeftShift,
            0x021E => LightsToggle,
            0x0065 => LineFeed,
            0x018B => List,
            0x01B1 => LogOff,
            0x0032 => M,
            0x0070 => Macro,
            0x0290 => Macro1,
            0x0299 => Macro10,
            0x029A => Macro11,
            0x029B => Macro12,
            0x029C => Macro13,
            0x029D => Macro14,
            0x029E => Macro15,
            0x029F => Macro16,
            0x02A0 => Macro17,
            0x02A1 => Macro18,
            0x02A2 => Macro19,
            0x0291 => Macro2,
            0x02A3 => Macro20,
            0x02A4 => Macro21,
            0x02A5 => Macro22,
            0x02A6 => Macro23,
            0x02A7 => Macro24,
            0x02A8 => Macro25,
            0x02A9 => Macro26,
            0x02AA => Macro27,
            0x02AB => Macro28,
            0x02AC => Macro29,
            0x0292 => Macro3,
            0x02AD => Macro30,
            0x0293 => Macro4,
            0x0294 => Macro5,
            0x0295 => Macro6,
            0x0296 => Macro7,
            0x0297 => Macro8,
            0x0298 => Macro9,
            0x02B3 => MacroPreset1,
            0x02B4 => MacroPreset2,
            0x02B5 => MacroPreset3,
            0x02B2 => MacroPresetCycle,
            0x02B0 => MacroRecordStart,
            0x02B1 => MacroRecordStop,
            0x009B => Mail,
            0x00E2 => Media,
            0x01B7 => MediaRepeat,
            0x026B => MediaTopMenu,
            0x018C => Memo,
            0x008B => Menu,
            0x01AE => Messenger,
            0x016F => Mhp,
            0x00F8 => MicMute,
            0x000C => Minus,
            0x0175 => Mode,
            0x00AF => Move,
            0x0187 => Mp3,
            0x0097 => MsDos,
            0x005E => Muhenkan,
            0x0071 => Mute,
            0x0031 => N,
            0x000B => N0,
            0x0002 => N1,
            0x0056 => N102nd,
            0x01B9 => N10ChannelsDown,
            0x01B8 => N10ChannelsUp,
            0x0003 => N2,
            0x0004 => N3,
            0x026F => N3dMode,
            0x0005 => N4,
            0x0006 => N5,
            0x0007 => N6,
            0x0008 => N7,
            0x0009 => N8,
            0x000A => N9,
            0x00B5 => New,
            0x01AB => News,
            0x0197 => Next,
            0x0270 => NextFavorite,
            0x00A3 => Nextsong,
            0x0200 => Numeric0,
            0x0201 => Numeric1,
            0x026C => Numeric11,
            0x026D => Numeric12,
            0x0202 => Numeric2,
            0x0203 => Numeric3,
            0x0204 => Numeric4,
            0x0205 => Numeric5,
            0x0206 => Numeric6,
            0x0207 => Numeric7,
            0x0208 => Numeric8,
            0x0209 => Numeric9,
            0x020C => NumericA,
            0x020D => NumericB,
            0x020E => NumericC,
            0x020F => NumericD,
            0x020B => NumericPound,
            0x020A => NumericStar,
            0x0045 => NumLock,
            0x0018 => O,
            0x0160 => Ok,
            0x0278 => OnscreenKeyboard,
            0x0086 => Open,
            0x0165 => Option,
            0x0019 => P,
            0x006D => Pagedown,
            0x0068 => Pageup,
            0x0087 => Paste,
            0x0077 => Pause,
            0x0272 => PauseRecord,
            0x00C9 => PauseCd,
            0x0178 => Pc,
            0x00A9 => Phone,
            0x00CF => Play,
            0x00C8 => PlayCd,
            0x0183 => Player,
            0x00A4 => PlayPause,
            0x0074 => Power,
            0x0164 => Power2,
            0x01A9 => Presentation,
            0x019C => Previous,
            0x00A5 => PreviousSong,
            0x00D2 => Print,
            0x0279 => PrivacyScreenToggle,
            0x0094 => Prog1,
            0x0095 => Prog2,
            0x00CA => Prog3,
            0x00CB => Prog4,
            0x016A => Program,
            0x0082 => Props,
            0x016E => Pvr,
            0x0010 => Q,
            0x00D6 => Question,
            0x0013 => R,
            0x0181 => Radio,
            0x00A7 => Record,
            0x018E => Red,
            0x00B6 => Redo,
            0x00AD => Refresh,
            0x00E8 => Reply,
            0x0000 => Reserved,
            0x0198 => Restart,
            0x00A8 => Rewind,
            0x00F7 => RfKill,
            0x006A => Right,
            0x0267 => RightDown,
            0x0266 => RightUp,
            0x0064 => RightAlt,
            0x001B => RightBrace,
            0x0061 => RightCtrl,
            0x007E => RightMeta,
            0x0036 => RightShift,
            0x0059 => Ro,
            0x026A => RootMenu,
            0x0099 => RotateDisplay,
            0x0231 => RotateLockToggle,
            0x001F => S,
            0x017D => Sat,
            0x017E => Sat2,
            0x00EA => Save,
            0x0078 => Scale,
            0x0245 => Screensaver,
            0x00B2 => ScrollDown,
            0x0046 => ScrollLock,
            0x00B1 => ScrollUp,
            0x00D9 => Search,
            0x0161 => Select,
            0x027A => SelectiveScreenshot,
            0x0027 => Semicolon,
            0x00E7 => Send,
            0x0091 => SendFile,
            0x008D => Setup,
            0x00DD => Shop,
            0x019A => Shuffle,
            0x0035 => Slash,
            0x008E => Sleep,
            0x0199 => Slow,
            0x0276 => SlowReverse,
            0x00D5 => Sound,
            0x0039 => Space,
            0x01B0 => Spellcheck,
            0x00DC => Sport,
            0x01A7 => Spreadsheet,
            0x0080 => Stop,
            0x0271 => StopRecord,
            0x00A6 => StopCd,
            0x0172 => Subtitle,
            0x00CD => Suspend,
            0x00E3 => SwitchVideoMode,
            0x0063 => SysRq,
            0x0014 => T,
            0x000F => Tab,
            0x0180 => Tape,
            0x0241 => TaskManager,
            0x019E => Teen,
            0x0184 => Text,
            0x0167 => Time,
            0x0171 => Title,
            0x0214 => TouchpadOff,
            0x0213 => TouchpadOn,
            0x0212 => TouchpadToggle,
            0x0182 => Tuner,
            0x0179 => Tv,
            0x017A => Tv2,
            0x019F => Twen,
            0x0016 => U,
            0x0083 => Undo,
            0x00F0 => Unknown,
            0x0274 => Unmute,
            0x0067 => Up,
            0x00EF => Uwb,
            0x002F => V,
            0x017B => Vcr,
            0x017C => Vcr2,
            0x0168 => Vendor,
            0x0189 => Video,
            0x00F1 => VideoNext,
            0x00F2 => VideoPrev,
            0x01A0 => VideoPhone,
            0x0273 => Vod,
            0x0246 => VoiceCommand,
            0x01AC => VoiceMail,
            0x0072 => VolumeDown,
            0x0073 => VolumeUp,
            0x0011 => W,
            0x008F => WakeUp,
            0x00EE => Wlan,
            0x01A5 => WordProcessor,
            0x0211 => WpsButton,
            0x00F6 => Wwan,
            0x0096 => Www,
            0x002D => X,
            0x0093 => Xfer,
            0x0015 => Y,
            0x0190 => Yellow,
            0x007C => Yen,
            0x002C => Z,
            0x0055 => ZenkakuHankaku,
            0x01A2 => ZoomIn,
            0x01A3 => ZoomOut,
            0x01A4 => ZoomReset,
            _ => return None,
        };

        Some(key)
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.to_raw() == other.to_raw()
    }
}
