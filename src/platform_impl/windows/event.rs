use std::{char, ptr};
use std::os::raw::c_int;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

use event::keyboard::{ModifierState, PhysicalKey, LogicalKey, EditKey, CommandKey, FunctionKey, NavigationKey, NumpadKey, ModifierKey, MediaKey};

use winapi::shared::minwindef::{WPARAM, LPARAM, UINT, HKL, HKL__};
use winapi::um::winuser;

fn key_pressed(vkey: c_int) -> bool {
    unsafe {
        (winuser::GetKeyState(vkey) & (1 << 15)) == (1 << 15)
    }
}

pub fn get_key_mods() -> ModifierState {
    unimplemented!()
    // let mut mods = ModifierState::default();
    // let filter_out_altgr = layout_uses_altgr() && key_pressed(winuser::VK_RMENU);

    // mods.shift = key_pressed(winuser::VK_SHIFT);
    // mods.ctrl = key_pressed(winuser::VK_CONTROL) && !filter_out_altgr;
    // mods.alt = key_pressed(winuser::VK_MENU) && !filter_out_altgr;
    // mods.logo = key_pressed(winuser::VK_LWIN) || key_pressed(winuser::VK_RWIN);
    // mods
}

unsafe fn get_char(keyboard_state: &[u8; 256], v_key: u32, hkl: HKL) -> Option<char> {
    let mut unicode_bytes = [0u16; 5];
    let len = winuser::ToUnicodeEx(v_key, 0, keyboard_state.as_ptr(), unicode_bytes.as_mut_ptr(), unicode_bytes.len() as _, 0, hkl);
    if len >= 1 {
        char::decode_utf16(unicode_bytes.into_iter().cloned()).next().and_then(|c| c.ok())
    } else {
        None
    }
}

/// Figures out if the keyboard layout has an AltGr key instead of an Alt key.
///
/// Unfortunately, the Windows API doesn't give a way for us to conveniently figure that out. So,
/// we use a technique blatantly stolen from [the Firefox source code][source]: iterate over every
/// possible virtual key and compare the `char` output when AltGr is pressed vs when it isn't. If
/// pressing AltGr outputs characters that are different from the standard characters, the layout
/// uses AltGr. Otherwise, it doesn't.
///
/// [source]: https://github.com/mozilla/gecko-dev/blob/265e6721798a455604328ed5262f430cfcc37c2f/widget/windows/KeyboardLayout.cpp#L4356-L4416
fn layout_uses_altgr() -> bool {
    unsafe {
        static ACTIVE_LAYOUT: AtomicPtr<HKL__> = AtomicPtr::new(ptr::null_mut());
        static USES_ALTGR: AtomicBool = AtomicBool::new(false);

        let hkl = winuser::GetKeyboardLayout(0);
        let old_hkl = ACTIVE_LAYOUT.swap(hkl, Ordering::SeqCst);

        if hkl == old_hkl {
            return USES_ALTGR.load(Ordering::SeqCst);
        }

        let mut keyboard_state_altgr = [0u8; 256];
        // AltGr is an alias for Ctrl+Alt for... some reason. Whatever it is, those are the keypresses
        // we have to emulate to do an AltGr test.
        keyboard_state_altgr[winuser::VK_MENU as usize] = 0x80;
        keyboard_state_altgr[winuser::VK_CONTROL as usize] = 0x80;

        let keyboard_state_empty = [0u8; 256];

        for v_key in 0..=255 {
            let key_noaltgr = get_char(&keyboard_state_empty, v_key, hkl);
            let key_altgr = get_char(&keyboard_state_altgr, v_key, hkl);
            if let (Some(noaltgr), Some(altgr)) = (key_noaltgr, key_altgr) {
                if noaltgr != altgr {
                    USES_ALTGR.store(true, Ordering::SeqCst);
                    return true;
                }
            }
        }

        USES_ALTGR.store(false, Ordering::SeqCst);
        false
    }
}

pub struct MappedKey {
    pub vkey: c_int,
    pub scancode: UINT,
    pub extended: bool,
    pub physical: Option<PhysicalKey>,
    pub logical: Option<LogicalKey>,
}

// Keys that are mapped the same way in PhysicalKey and LogicalKey.
enum CommonKey {
    Edit(EditKey),
    Command(CommandKey),
    Function(FunctionKey),
    Navigation(NavigationKey),
    /// Handles the non-numeric/decimal numpad keys
    Numpad(NumpadKey),
    Modifier(ModifierKey),
    Media(MediaKey),
}

impl CommonKey {
    fn from_raw(vkey: c_int, extended: bool) -> Option<CommonKey> {
        use self::{
            CommonKey::*,
            EditKey::*,
            CommandKey::*,
            FunctionKey::*,
            NavigationKey::*,
            NumpadKey::*,
            ModifierKey::*,
            MediaKey::*,
        };
        // VK_* codes are documented here https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx
        match vkey {
            winuser::VK_BACK => Some(Edit(Backspace)),
            winuser::VK_TAB => Some(Edit(Tab)),
            winuser::VK_RETURN if !extended => Some(Edit(EnterLeft)),
            winuser::VK_RETURN if extended => Some(Edit(EnterRight)),
            winuser::VK_LSHIFT => Some(Modifier(ShiftLeft)),
            winuser::VK_RSHIFT => Some(Modifier(ShiftRight)),
            winuser::VK_LCONTROL => Some(Modifier(ControlLeft)),
            winuser::VK_RCONTROL => Some(Modifier(ControlRight)),
            winuser::VK_LMENU => Some(Modifier(AltLeft)),
            // AltGr?
            winuser::VK_RMENU => Some(Modifier(AltRight)),
            winuser::VK_PAUSE => Some(Command(PauseBreak)),
            winuser::VK_CAPITAL => Some(Modifier(CapsLock)),
            // winuser::VK_KANA => None,
            //winuser::VK_JUNJA => Some(VirtualKeyCode::Junja),
            //winuser::VK_FINAL => Some(VirtualKeyCode::Final),
            // winuser::VK_KANJI => Some(VirtualKeyCode::Kanji),
            winuser::VK_ESCAPE => Some(Command(Escape)),
            // VK_HANGUL and VK_HANJA are rolled in Convert/NonConvert, for simplicity. We may want
            // to rename those keys to more accurately expose that it does that.
            winuser::VK_HANGUL   |
            winuser::VK_CONVERT => Some(Command(Convert)),
            winuser::VK_HANJA       |
            winuser::VK_NONCONVERT => Some(Command(NonConvert)),
            winuser::VK_MODECHANGE => Some(Command(IMEMode)),
            winuser::VK_SPACE => Some(Edit(Space)),
            winuser::VK_PRIOR if extended => Some(Navigation(PageUp)),
            winuser::VK_NEXT if extended => Some(Navigation(PageDown)),
            winuser::VK_END if extended => Some(Navigation(End)),
            winuser::VK_HOME if extended => Some(Navigation(Home)),
            winuser::VK_LEFT if extended => Some(Navigation(Left)),
            winuser::VK_UP if extended => Some(Navigation(Up)),
            winuser::VK_RIGHT if extended => Some(Navigation(Right)),
            winuser::VK_DOWN if extended => Some(Navigation(Down)),
            winuser::VK_SNAPSHOT => Some(Command(PrintScreen)),
            winuser::VK_INSERT if extended => Some(Edit(Insert)),
            winuser::VK_DELETE if extended => Some(Edit(Delete)),
            winuser::VK_LWIN => Some(Modifier(MetaLeft)),
            winuser::VK_RWIN => Some(Modifier(MetaRight)),
            winuser::VK_APPS => Some(Command(Menu)),
            winuser::VK_MULTIPLY => Some(Numpad(Multiply)),
            winuser::VK_ADD => Some(Numpad(Add)),
            winuser::VK_SUBTRACT => Some(Numpad(Subtract)),
            winuser::VK_DECIMAL => Some(Numpad(Period)),
            winuser::VK_DIVIDE => Some(Numpad(Divide)),
            winuser::VK_NUMPAD0 => Some(Numpad(Num0)),
            winuser::VK_NUMPAD1 => Some(Numpad(Num1)),
            winuser::VK_NUMPAD2 => Some(Numpad(Num2)),
            winuser::VK_NUMPAD3 => Some(Numpad(Num3)),
            winuser::VK_NUMPAD4 => Some(Numpad(Num4)),
            winuser::VK_NUMPAD5 => Some(Numpad(Num5)),
            winuser::VK_NUMPAD6 => Some(Numpad(Num6)),
            winuser::VK_NUMPAD7 => Some(Numpad(Num7)),
            winuser::VK_NUMPAD8 => Some(Numpad(Num8)),
            winuser::VK_NUMPAD9 => Some(Numpad(Num9)),
            winuser::VK_F1 => Some(Function(F1)),
            winuser::VK_F2 => Some(Function(F2)),
            winuser::VK_F3 => Some(Function(F3)),
            winuser::VK_F4 => Some(Function(F4)),
            winuser::VK_F5 => Some(Function(F5)),
            winuser::VK_F6 => Some(Function(F6)),
            winuser::VK_F7 => Some(Function(F7)),
            winuser::VK_F8 => Some(Function(F8)),
            winuser::VK_F9 => Some(Function(F9)),
            winuser::VK_F10 => Some(Function(F10)),
            winuser::VK_F11 => Some(Function(F11)),
            winuser::VK_F12 => Some(Function(F12)),
            winuser::VK_F13 => Some(Function(F13)),
            winuser::VK_F14 => Some(Function(F14)),
            winuser::VK_F15 => Some(Function(F15)),
            winuser::VK_F16 => Some(Function(F16)),
            winuser::VK_F17 => Some(Function(F17)),
            winuser::VK_F18 => Some(Function(F18)),
            winuser::VK_F19 => Some(Function(F19)),
            winuser::VK_F20 => Some(Function(F20)),
            winuser::VK_F21 => Some(Function(F21)),
            winuser::VK_F22 => Some(Function(F22)),
            winuser::VK_F23 => Some(Function(F23)),
            winuser::VK_F24 => Some(Function(F24)),
            winuser::VK_NUMLOCK => Some(Modifier(NumLock)),
            winuser::VK_SCROLL => Some(Modifier(ScrollLock)),
            winuser::VK_MEDIA_NEXT_TRACK => Some(Media(TrackNext)),
            winuser::VK_MEDIA_PREV_TRACK => Some(Media(TrackPrevious)),
            winuser::VK_MEDIA_STOP => Some(Media(Stop)),
            winuser::VK_MEDIA_PLAY_PAUSE => Some(Media(PlayPause)),
            _ => None
        }
    }
}

pub fn handle_extended_keys(mut vkey: c_int, mut scancode: UINT, extended: bool) -> Option<MappedKey> {
    // Welcome to hell https://blog.molecular-matters.com/2011/09/05/properly-handling-keyboard-input/

    match (vkey, scancode, extended) {
        (winuser::VK_SHIFT, _, _) => {
            vkey = unsafe {
                winuser::MapVirtualKeyA(
                    scancode,
                    winuser::MAPVK_VSC_TO_VK_EX,
                ) as _
            };
        },
        (winuser::VK_CONTROL, _, true) => vkey = winuser::VK_RCONTROL,
        (winuser::VK_CONTROL, _, false) => vkey = winuser::VK_LCONTROL,
        (winuser::VK_MENU, _, true) => vkey = winuser::VK_RMENU,
        (winuser::VK_MENU, _, false) => vkey = winuser::VK_LMENU,
        (winuser::VK_NUMLOCK, _, _) => {
            scancode = unsafe{ winuser::MapVirtualKeyA(vkey as _, winuser::MAPVK_VK_TO_VSC) } | 0x100;
        },

        // This is only triggered when using raw input. Without this check, we get two events whenever VK_PAUSE is
        // pressed, the first one having scancode 0x1D but vkey VK_PAUSE...
        (winuser::VK_PAUSE, 0x1D, _) => return None,
        // ...and the second having scancode 0x45 but an unmatched vkey!
        (_, 0x45, _) => vkey = winuser::VK_PAUSE,
        // VK_PAUSE and VK_SCROLL have the same scancode when using modifiers, alongside incorrect vkey values.
        (_, 0x46, true) => {
            scancode = 0x45;
            vkey = winuser::VK_PAUSE
        },
        (_, 0x46, false) => {
            vkey = winuser::VK_SCROLL;
        },
        _ => (),
    };

    let common_key = CommonKey::from_raw(vkey, extended);
    Some(MappedKey {
        vkey,
        scancode,
        extended,
        physical: match common_key {
            Some(CommonKey::Edit(e)) => Some(PhysicalKey::Edit(e)),
            Some(CommonKey::Command(e)) => Some(PhysicalKey::Command(e)),
            Some(CommonKey::Function(e)) => Some(PhysicalKey::Function(e)),
            Some(CommonKey::Navigation(e)) => Some(PhysicalKey::Navigation(e)),
            Some(CommonKey::Numpad(e)) => Some(PhysicalKey::Numpad(e)),
            Some(CommonKey::Modifier(e)) => Some(PhysicalKey::Modifier(e)),
            Some(CommonKey::Media(e)) => Some(PhysicalKey::Media(e)),
            None => {
                use event::keyboard::{
                    PhysicalKey::*,
                    PhysicalAlphaNumKey::*,
                    NumpadKey::*,
                };

                match scancode {
                    0x29 => Some(AlphaNum(IntlGrave)),
                    0x02 => Some(AlphaNum(Key1)),
                    0x03 => Some(AlphaNum(Key2)),
                    0x04 => Some(AlphaNum(Key3)),
                    0x05 => Some(AlphaNum(Key4)),
                    0x06 => Some(AlphaNum(Key5)),
                    0x07 => Some(AlphaNum(Key6)),
                    0x08 => Some(AlphaNum(Key7)),
                    0x09 => Some(AlphaNum(Key8)),
                    0x0A => Some(AlphaNum(Key9)),
                    0x0B => Some(AlphaNum(Key0)),
                    0x0C => Some(AlphaNum(IntlDash)),
                    0x0D => Some(AlphaNum(IntlEquals)),
                    0x7D => Some(AlphaNum(IntlYen)),
                    0x10 => Some(AlphaNum(Q)),
                    0x11 => Some(AlphaNum(W)),
                    0x12 => Some(AlphaNum(E)),
                    0x13 => Some(AlphaNum(R)),
                    0x14 => Some(AlphaNum(T)),
                    0x15 => Some(AlphaNum(Y)),
                    0x16 => Some(AlphaNum(U)),
                    0x17 => Some(AlphaNum(I)),
                    0x18 => Some(AlphaNum(O)),
                    0x19 => Some(AlphaNum(P)),
                    0x1A => Some(AlphaNum(IntlLeftBracket)),
                    0x1B => Some(AlphaNum(IntlRightBracket)),
                    0x1E => Some(AlphaNum(A)),
                    0x1F => Some(AlphaNum(S)),
                    0x20 => Some(AlphaNum(D)),
                    0x21 => Some(AlphaNum(F)),
                    0x22 => Some(AlphaNum(G)),
                    0x23 => Some(AlphaNum(H)),
                    0x24 => Some(AlphaNum(J)),
                    0x25 => Some(AlphaNum(K)),
                    0x26 => Some(AlphaNum(L)),
                    0x27 => Some(AlphaNum(IntlSemicolon)),
                    0x28 => Some(AlphaNum(IntlApostrophe)),
                    0x2B => Some(AlphaNum(IntlBackslashRight)),
                    0x2C => Some(AlphaNum(Z)),
                    0x2D => Some(AlphaNum(X)),
                    0x2E => Some(AlphaNum(C)),
                    0x2F => Some(AlphaNum(V)),
                    0x30 => Some(AlphaNum(B)),
                    0x31 => Some(AlphaNum(N)),
                    0x32 => Some(AlphaNum(M)),
                    0x33 => Some(AlphaNum(IntlComma)),
                    0x34 => Some(AlphaNum(IntlPeriod)),
                    0x35 => Some(AlphaNum(IntlSlash)),
                    0x56 => Some(AlphaNum(IntlBackslashLeft)),
                    0x73 => Some(AlphaNum(IntlRo)),
                    _ => if extended {
                        None
                    } else {
                        match vkey {
                            winuser::VK_INSERT => Some(Numpad(Num0)),
                            winuser::VK_END => Some(Numpad(Num1)),
                            winuser::VK_DOWN => Some(Numpad(Num2)),
                            winuser::VK_NEXT => Some(Numpad(Num3)),
                            winuser::VK_LEFT => Some(Numpad(Num4)),
                            winuser::VK_CLEAR => Some(Numpad(Num5)),
                            winuser::VK_RIGHT => Some(Numpad(Num6)),
                            winuser::VK_HOME => Some(Numpad(Num7)),
                            winuser::VK_UP => Some(Numpad(Num8)),
                            winuser::VK_PRIOR => Some(Numpad(Num9)),
                            winuser::VK_DELETE => Some(Numpad(Period)),
                            _ => None
                        }
                    }
                }
            }
        },
        logical: match common_key {
            Some(CommonKey::Edit(e)) => Some(LogicalKey::Edit(e)),
            Some(CommonKey::Command(e)) => Some(LogicalKey::Command(e)),
            Some(CommonKey::Function(e)) => Some(LogicalKey::Function(e)),
            Some(CommonKey::Navigation(e)) => Some(LogicalKey::Navigation(e)),
            Some(CommonKey::Numpad(e)) => Some(LogicalKey::Numpad(e)),
            Some(CommonKey::Modifier(e)) => Some(LogicalKey::Modifier(e)),
            Some(CommonKey::Media(e)) => Some(LogicalKey::Media(e)),
            None => {
                use event::keyboard::{
                    LogicalKey::*,
                    LogicalAlphaNumKey::*,
                };

                match vkey {
                    0x30 => Some(AlphaNum(Key0)),
                    0x31 => Some(AlphaNum(Key1)),
                    0x32 => Some(AlphaNum(Key2)),
                    0x33 => Some(AlphaNum(Key3)),
                    0x34 => Some(AlphaNum(Key4)),
                    0x35 => Some(AlphaNum(Key5)),
                    0x36 => Some(AlphaNum(Key6)),
                    0x37 => Some(AlphaNum(Key7)),
                    0x38 => Some(AlphaNum(Key8)),
                    0x39 => Some(AlphaNum(Key9)),
                    0x41 => Some(AlphaNum(A)),
                    0x42 => Some(AlphaNum(B)),
                    0x43 => Some(AlphaNum(C)),
                    0x44 => Some(AlphaNum(D)),
                    0x45 => Some(AlphaNum(E)),
                    0x46 => Some(AlphaNum(F)),
                    0x47 => Some(AlphaNum(G)),
                    0x48 => Some(AlphaNum(H)),
                    0x49 => Some(AlphaNum(I)),
                    0x4A => Some(AlphaNum(J)),
                    0x4B => Some(AlphaNum(K)),
                    0x4C => Some(AlphaNum(L)),
                    0x4D => Some(AlphaNum(M)),
                    0x4E => Some(AlphaNum(N)),
                    0x4F => Some(AlphaNum(O)),
                    0x50 => Some(AlphaNum(P)),
                    0x51 => Some(AlphaNum(Q)),
                    0x52 => Some(AlphaNum(R)),
                    0x53 => Some(AlphaNum(S)),
                    0x54 => Some(AlphaNum(T)),
                    0x55 => Some(AlphaNum(U)),
                    0x56 => Some(AlphaNum(V)),
                    0x57 => Some(AlphaNum(W)),
                    0x58 => Some(AlphaNum(X)),
                    0x59 => Some(AlphaNum(Y)),
                    0x5A => Some(AlphaNum(Z)),

                    winuser::VK_OEM_3 => Some(AlphaNum(IntlGrave)),
                    winuser::VK_OEM_MINUS => Some(AlphaNum(IntlDash)),
                    winuser::VK_OEM_PLUS => Some(AlphaNum(IntlEquals)),
                    winuser::VK_OEM_4 => Some(AlphaNum(IntlLeftBracket)),
                    winuser::VK_OEM_6 => Some(AlphaNum(IntlRightBracket)),
                    winuser::VK_OEM_1 => Some(AlphaNum(IntlSemicolon)),
                    winuser::VK_OEM_7 => Some(AlphaNum(IntlApostrophe)),
                    winuser::VK_OEM_5 => Some(AlphaNum(IntlBackslashRight)),
                    winuser::VK_OEM_COMMA => Some(AlphaNum(IntlComma)),
                    winuser::VK_OEM_PERIOD => Some(AlphaNum(IntlPeriod)),
                    winuser::VK_OEM_2 => Some(AlphaNum(IntlSlash)),
                    winuser::VK_OEM_102 => Some(AlphaNum(IntlRo)),
                    _ => match scancode {
                        0x7D => Some(AlphaNum(IntlYen)),
                        0x56 => Some(AlphaNum(IntlBackslashLeft)),
                        _ => None,
                    }
                }
            }
        }
    })
}

/*
pub fn process_key_params(wparam: WPARAM, lparam: LPARAM) -> Option<(u32, Option<VirtualKeyCode>)> {
    let scancode = ((lparam >> 16) & 0xff) as UINT;
    let extended = (lparam & 0x01000000) != 0;
    handle_extended_keys(wparam as _, scancode, extended)
        .map(|(vkey, scancode)| (scancode, vkey_to_winit_vkey(vkey)))
}

// This is needed as windows doesn't properly distinguish
// some virtual key codes for different keyboard layouts
fn map_text_keys(win_virtual_key: i32) -> Option<VirtualKeyCode> {
    unimplemented!()
    let char_key = unsafe { winuser::MapVirtualKeyA(win_virtual_key as u32, winuser::MAPVK_VK_TO_CHAR) } & 0x7FFF;
    match char::from_u32(char_key) {
        Some(';') => Some(VirtualKeyCode::Semicolon),
        Some('/') => Some(VirtualKeyCode::Slash),
        Some('`') => Some(VirtualKeyCode::Grave),
        Some('[') => Some(VirtualKeyCode::LBracket),
        Some(']') => Some(VirtualKeyCode::RBracket),
        Some('\'') => Some(VirtualKeyCode::Apostrophe),
        Some('\\') => Some(VirtualKeyCode::Backslash),
        _ => None
    }
}
*/
