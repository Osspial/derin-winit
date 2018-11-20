use dpi::LogicalSize;
use platform::platform::icon::WinIcon;
use platform::platform::Cursor;
use winapi::shared::windef::{RECT, HWND};
use winapi::shared::minwindef::DWORD;
use winapi::um::winnt::LONG;
use winapi::um::winuser;

/// Contains information about states and the window that the callback is going to use.
#[derive(Clone)]
pub struct WindowState {
    pub mouse: MouseProperties,

    /// Used by `WM_GETMINMAXINFO`.
    pub min_size: Option<LogicalSize>,
    pub max_size: Option<LogicalSize>,

    pub window_icon: Option<WinIcon>,
    pub taskbar_icon: Option<WinIcon>,

    pub saved_window: SavedWindow,
    pub dpi_factor: f64,

    pub fullscreen: Option<::MonitorId>,
    window_flags: WindowFlags,
}

#[derive(Clone)]
pub struct SavedWindow {
    pub client_rect: RECT,
    pub dpi_factor: f64,
}

#[derive(Clone)]
pub struct MouseProperties {
    /// Cursor to set at the next `WM_SETCURSOR` event received.
    pub cursor: Cursor,
    pub flags: CursorFlags,
}

bitflags! {
    pub struct CursorFlags: u8 {
        const GRABBED   = 1 << 0;
        const HIDDEN    = 1 << 1;
        const IN_WINDOW = 1 << 2;
    }
}
bitflags! {
    pub struct WindowFlags: u32 {
        const RESIZABLE      = 1 << 0;
        const DECORATIONS    = 1 << 1;
        const VISIBLE        = 1 << 2;
        const ON_TASKBAR     = 1 << 3;
        const ALWAYS_ON_TOP  = 1 << 4;
        const NO_BACK_BUFFER = 1 << 5;
        const TRANSPARENT    = 1 << 6;
        const CHILD          = 1 << 7;
        const MAXIMIZED      = 1 << 8;
        /// Marker flag for fullscreen. Should always match `WindowState::fullscreen`, but is
        /// included here to make masking easier.
        const FULLSCREEN     = 1 << 9;

        const FULLSCREEN_AND_MASK = !(
            WindowFlags::DECORATIONS.bits |
            WindowFlags::RESIZABLE.bits |
            WindowFlags::MAXIMIZED.bits
        );
        const INVISIBLE_AND_MASK = !WindowFlags::MAXIMIZED.bits;
    }
}

impl WindowState {
    pub fn window_flags(&self) -> WindowFlags {
        self.window_flags
    }

    pub fn set_window_flags<F>(&mut self, window: HWND, f: F)
        where F: FnOnce(&mut WindowFlags)
    {
        let old_flags = self.window_flags;
        f(&mut self.window_flags);
        if self.fullscreen.is_some() {
            self.window_flags |= WindowFlags::FULLSCREEN;
        } else {
            self.window_flags &= !WindowFlags::FULLSCREEN;
        }

        old_flags.apply_diff(self.window_flags, window);
    }

    pub fn refresh_window_flags(&mut self, window: HWND) {
        self.set_window_flags(window, |_| ());
    }
}

impl WindowFlags {
    fn mask(mut self) -> WindowFlags {
        if self.contains(WindowFlags::FULLSCREEN) {
            self &= WindowFlags::FULLSCREEN_AND_MASK;
        }
        if !self.contains(WindowFlags::VISIBLE) {
            self &= WindowFlags::INVISIBLE_AND_MASK;
        }
        self
    }

    fn to_window_styles(self) -> (DWORD, DWORD) {
        use winapi::um::winuser::*;

        let (mut style, mut style_ex) = (0, 0);

        if self.contains(WindowFlags::RESIZABLE) {
            style |= winuser::WS_SIZEBOX | winuser::WS_MAXIMIZEBOX;
        }
        if self.contains(WindowFlags::DECORATIONS) {
            style |= WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX;
            style_ex = WS_EX_WINDOWEDGE;
        }
        // if self.contains(WindowFlags::VISIBLE) {
        //     // Handled with ShowWindow call in apply_diff
        // }
        if self.contains(WindowFlags::ON_TASKBAR) {
            style_ex |= WS_EX_APPWINDOW;
        }
        if self.contains(WindowFlags::ALWAYS_ON_TOP) {
            style_ex | WS_EX_TOPMOST;
        }
        if self.contains(WindowFlags::NO_BACK_BUFFER) {
            style_ex |= WS_EX_NOREDIRECTIONBITMAP;
        }
        if self.contains(WindowFlags::TRANSPARENT) {
            // Is this necessary? The docs say that WS_EX_LAYERED requires a windows class without
            // CS_OWNDC, and Winit windows have that flag set.
            style_ex |= WS_EX_LAYERED;
        }
        if self.contains(WindowFlags::CHILD) {
            style |= WS_CHILD; // This is incompatible with WS_POPUP if that gets added eventually.
        }
        // if self.contains(WindowFlags::MAXIMIZED) {
        //     // Handled with ShowWindow call in apply_diff
        // }

        style |= WS_CLIPSIBLINGS | WS_CLIPCHILDREN;
        style_ex |= WS_EX_ACCEPTFILES;

        (style, style_ex)
    }

    fn apply_diff(mut self, mut new: WindowFlags, window: HWND) {
        self = self.mask();
        new = new.mask();

        let diff = self ^ new;
        if diff == WindowFlags::empty() {
            return;
        }

        let (style, style_ex) = new.to_window_styles();
        unsafe{
            winuser::SetWindowLongW(window, winuser::GWL_STYLE, style as _);
            winuser::SetWindowLongW(window, winuser::GWL_EXSTYLE, style_ex as _);
        }

        if diff.contains(WindowFlags::MAXIMIZED) {
            unsafe {
                winuser::ShowWindow(
                    window,
                    match new.contains(WindowFlags::MAXIMIZED) {
                        true => winuser::SW_MAXIMIZE,
                        false => winuser::SW_RESTORE
                    }
                );
            }
        }
        if diff.contains(WindowFlags::VISIBLE) {
            unsafe {
                winuser::ShowWindow(
                    window,
                    match new.contains(WindowFlags::VISIBLE) {
                        true => winuser::SW_SHOW,
                        false => winuser::SW_HIDE
                    }
                );
            }
        }
    }
}
