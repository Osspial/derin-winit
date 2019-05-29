#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyboardEvent {
    pub chara: Option<char>,
    pub composition: Option<CompositionEvent>,
    pub key: Option<KeyEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CompositionEvent {
    Start(String),
    Update(String),
    End(String),
    Cancel,
}

/// Describes a keyboard input event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyEvent {
    pub state: KeyState,
    pub key: Option<Key>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Key {
    pub physical: PhysicalKey,
    pub logical: LogicalKey,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KeyState {
    Pressed,
    Repeated(usize),
    Released,
}

/// Representation of the physical location of a key.
///
/// See module-level documentation.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    /// Kana Mode on Japanese keyboards. Changes the input method.
    IMEMode,
    NonConvert,
    Convert,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    NumLock,
    CapsLock,
    ScrollLock,
}

bitflags! {
    /// TODO: ADD SERDE SUPPORT. Just serializing this as a number is ugly, so we should find a better way.
    pub struct ModifierState: u32 {
        /// Either control key is pressed.
        const CONTROL           = 0b0000000000000001;
        const CONTROL_LEFT      = 0b0000000000000010;
        const CONTROL_RIGHT     = 0b0000000000000100;
        /// Set of all `CONTROL*` flags.
        const CONTROL_ALL       = 0b0000000000000111;
        /// Either shift key is pressed.
        const SHIFT             = 0b0000000000001000;
        const SHIFT_LEFT        = 0b0000000000010000;
        const SHIFT_RIGHT       = 0b0000000000100000;
        /// Set of all `SHIFT*` flags.
        const SHIFT_ALL         = 0b0000000000111000;
        /// Either alt key is pressed.
        const ALT               = 0b0000000001000000;
        const ALT_LEFT          = 0b0000000010000000;
        const ALT_RIGHT         = 0b0000000100000000;
        /// Set of all `ALT*` flags, except `ALT_GR`.
        const ALT_ALL           = 0b0000000111000000;
        /// Note that setting `ALT_GR` doesn't set `ALT.
        const ALT_GR            = 0b0000001000000000;
        /// Either meta key is pressed.
        const META              = 0b0000010000000000;
        const META_LEFT         = 0b0000100000000000;
        const META_RIGHT        = 0b0001000000000000;
        /// Set of all `META*` flags.
        const META_ALL          = 0b0001110000000000;
        const NUM_LOCK          = 0b0010000000000000;
        const CAPS_LOCK         = 0b0100000000000000;
        const SCROLL_LOCK       = 0b1000000000000000;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    fn write_key_label(&self, string: &mut String);

    fn key_label(&self) -> String {
        let mut string = String::new();
        self.write_key_label(&mut string);
        string
    }
}

impl KeyLabel for PhysicalKey {
    fn write_key_label(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for LogicalKey {
    fn write_key_label(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for PhysicalAlphaNumKey {
    fn write_key_label(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for LogicalAlphaNumKey {
    fn write_key_label(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for EditKey {
    fn write_key_label(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for CommandKey {
    fn write_key_label(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for FunctionKey {
    fn write_key_label(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for MediaKey {
    fn write_key_label(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for ModifierKey {
    fn write_key_label(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for NavigationKey {
    fn write_key_label(&self, _string: &mut String) {
        unimplemented!()
    }
}

impl KeyLabel for NumpadKey {
    fn write_key_label(&self, _string: &mut String) {
        unimplemented!()
    }
}
