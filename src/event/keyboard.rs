// TODO: SERDE EVERYTHING

/// TODO: COME UP WITH REAL NAME. `InputEvent` IS VAGUE.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputEvent {
    pub chara: Option<char>,
    pub composition: Option<CompositionEvent>,
    pub keyboard: Option<KeyboardEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompositionEvent {
    CompositionStart(String),
    CompositionUpdate(String),
    CompositionEnd(String),
}

/// Describes a keyboard input event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyboardEvent {
    pub state: KeyState,
    pub key: Option<Key>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Key {
    pub physical: PhysicalKey,
    pub logical: LogicalKey,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum KeyState {
    Pressed,
    Repeated(usize),
    Released,
}

/// Representation of the physical location of a key.
///
/// See module-level documentation.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum PhysicalKey {
    AlphaNum(PhysicalAlphaNumKey),
    Navigation(NavigationKey),
    Command(CommandKey),
    Function(FunctionKey),
    Numpad(NumpadKey),
    Media(MediaKey),
    Modifier(ModifierKey),
    Edit(EditKey),
}

/// Representation of the logical semantics of a key.
///
/// See module-level documentation.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum LogicalKey {
    AlphaNum(LogicalAlphaNumKey),
    Navigation(NavigationKey),
    Command(CommandKey),
    Function(FunctionKey),
    Numpad(NumpadKey),
    Media(MediaKey),
    Modifier(ModifierKey),
    Edit(EditKey),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum PhysicalAlphaNumKey {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,

    /// Labeled <code>`~</code> on a US keyboard.
    IntlGrave,
    /// Labeled `-_` on a US keyboard.
    IntlDash,
    /// Labeled `=+` on a US keyboard.
    IntlEquals,
    /// Labeled `[{` on a US keyboard.
    IntlLeftBracket,
    /// Labeled `]}` on a US keyboard.
    IntlRightBracket,
    /// Labeled `;:` on a US keyboard.
    IntlSemicolon,
    /// Labeled `'"` on a US keyboard.
    IntlApostrophe,
    /// Labeled `,<` on a US keyboard.
    IntlComma,
    /// Labeled `.>` on a US keyboard.
    IntlPeriod,
    /// Labeled `/?` on a US keyboard.
    IntlSlash,
    /// Labeled `\|` on a UK keyboard. Doesn't exist on all keyboard layouts.
    IntlBackslashLeft,
    /// Labeled `\|` on a US keyboard. Doesn't exist on all keyboard layouts.
    IntlBackslashRright,
    /// Labeled `\ろ` (ro) on a Japanese keyboard. Doesn't exist on all keyboard layouts.
    IntlRo,
    /// Labeled `¥` (yen) on a Japanese keyboard and `\/` on a Russian keyboard. Doesn't exist on
    /// all keyboard layouts.
    IntlYen,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum LogicalAlphaNumKey {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,

    /// Labeled <code>`~</code> on a US keyboard.
    IntlGrave,
    /// Labeled `-_` on a US keyboard.
    IntlDash,
    /// Labeled `=+` on a US keyboard.
    IntlEquals,
    /// Labeled `[{` on a US keyboard.
    IntlLeftBracket,
    /// Labeled `]}` on a US keyboard.
    IntlRightBracket,
    /// Labeled `;:` on a US keyboard.
    IntlSemicolon,
    /// Labeled `'"` on a US keyboard.
    IntlApostrophe,
    /// Labeled `,<` on a US keyboard.
    IntlComma,
    /// Labeled `.>` on a US keyboard.
    IntlPeriod,
    /// Labeled `/?` on a US keyboard.
    IntlSlash,
    /// Labeled `\|` on a UK keyboard. Doesn't exist on all keyboard layouts.
    IntlBackslashLeft,
    /// Labeled `\|` on a US keyboard. Doesn't exist on all keyboard layouts.
    IntlBackslashRright,
    /// Labeled `\ろ` (ro) on a Japanese keyboard. Doesn't exist on all keyboard layouts.
    IntlRo,
    /// Labeled `¥` (yen) on a Japanese keyboard and `\/` on a Russian keyboard. Doesn't exist on
    /// all keyboard layouts.
    IntlYen,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum EditKey {
    Space,
    Tab,
    EnterLeft,
    EnterRight,
    Backspace,
    Insert,
    Delete,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum CommandKey {
    Escape,
    Menu,
    PrintScreen,
    Break,

    Cut,
    Copy,
    Paste,
    Undo,
    Redo,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum FunctionKey {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
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
    F20,
    F21,
    F22,
    F23,
    F24,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum MediaKey {
    PlayPause,
    Pause,
    Play,
    Stop,
    FastForward,
    Rewind,
    Record,
    TrackNext,
    TrackPrevious
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum ModifierKey {
    ControlLeft,
    ControlRight,
    ShiftLeft,
    ShiftRight,
    AltLeft,
    AltRight,
    AltGr,
    MetaLeft,
    MetaRight,
    /// Left thumb-shift key on Japanese keyboards.
    ThumbShiftLeft,
    /// Left thumb-shift key on Japanese keyboards.
    ThumbShiftRight,

    NumLock,
    CapsLock,
    ScrollLock,
    /// Kana input toggle on Japanese keyboards.
    KanaLock,
}

bitflags! {
    pub struct ModifierState: u32 {
        const CONTROL           = 0b000000000000011;
        const CONTROL_LEFT      = 0b000000000000001;
        const CONTROL_RIGHT     = 0b000000000000010;
        const SHIFT             = 0b000000000001100;
        const SHIFT_LEFT        = 0b000000000000100;
        const SHIFT_RIGHT       = 0b000000000001000;
        const ALT               = 0b000000000110000;
        const ALT_LEFT          = 0b000000000010000;
        const ALT_RIGHT         = 0b000000000100000;
        /// Note the `ALT` does not include `ALT_GR`.
        const ALT_GR            = 0b000000001000000;
        const META              = 0b000000110000000;
        const META_LEFT         = 0b000000010000000;
        const META_RIGHT        = 0b000000100000000;
        const THUMB_SHIFT       = 0b000011000000000;
        const THUMB_SHIFT_LEFT  = 0b000001000000000;
        const THUMB_SHIFT_RIGHT = 0b000010000000000;
        const NUM_LOCK          = 0b000100000000000;
        const CAPS_LOCK         = 0b001000000000000;
        const SCROLL_LOCK       = 0b010000000000000;
        const KANA_LOCK         = 0b100000000000000;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum NavigationKey {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum NumpadKey {
    Divide,
    Multiply,
    Subtract,
    Add,
    Period,

    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
}

/// Retrieves the given key's label in the host keymap.
pub trait KeyLabel {
    /// Append the label to the end of the given string.
    fn key_label_into(&self, string: &mut String);

    fn key_label(&self) -> String {
        let mut string = String::new();
        self.key_label_into(&mut string);
        string
    }
}

impl KeyLabel for PhysicalKey {
    fn key_label_into(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for LogicalKey {
    fn key_label_into(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for PhysicalAlphaNumKey {
    fn key_label_into(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for LogicalAlphaNumKey {
    fn key_label_into(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for EditKey {
    fn key_label_into(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for CommandKey {
    fn key_label_into(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for FunctionKey {
    fn key_label_into(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for MediaKey {
    fn key_label_into(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for ModifierKey {
    fn key_label_into(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for NavigationKey {
    fn key_label_into(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for NumpadKey {
    fn key_label_into(&self, _string: &mut String) {
        unimplemented!()
    }
}
