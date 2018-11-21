use {MouseCursor, WindowAttributes};
use std::io;
use dpi::LogicalSize;
use platform::platform::icon::WinIcon;
use platform::platform::{util, Cursor};
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

    pub saved_window: Option<SavedWindow>,
    pub dpi_factor: f64,

    fullscreen: Option<::MonitorId>,
    window_flags: WindowFlags,
}

#[derive(Clone)]
pub struct SavedWindow {
    pub client_rect: RECT,
    pub dpi_factor: f64,
}

#[derive(Clone)]
pub struct MouseProperties {
    pub cursor: MouseCursor,
    cursor_flags: CursorFlags,
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
    pub fn new(
        attributes: &WindowAttributes,
        window_icon: Option<WinIcon>,
        taskbar_icon: Option<WinIcon>,
        dpi_factor: f64
    ) -> WindowState {
        WindowState {
            mouse: MouseProperties {
                cursor: MouseCursor::default(),
                cursor_flags: CursorFlags::empty(),
            },

            min_size: attributes.min_dimensions,
            max_size: attributes.max_dimensions,

            window_icon,
            taskbar_icon,

            saved_window: None,
            dpi_factor,

            fullscreen: None,
            window_flags: WindowFlags::empty()
        }
    }

    pub fn window_flags(&self) -> WindowFlags {
        self.window_flags
    }

    pub fn fullscreen(&self) -> &Option<::MonitorId> {
        &self.fullscreen
    }

    pub fn set_fullscreen(&mut self, window: HWND, fullscreen: Option<::MonitorId>) {
        self.fullscreen = fullscreen;

        // Update the FULLSCREEN flag
        self.set_window_flags(window, |_| ());
    }

    pub fn set_window_flags<F>(&mut self, window: HWND, f: F)
        where F: FnOnce(&mut WindowFlags)
    {
        let old_flags = self.window_flags;
        f(&mut self.window_flags);
        self.window_flags.set(WindowFlags::FULLSCREEN, self.fullscreen.is_some());

        old_flags.apply_diff(self.window_flags, window);
    }
}

impl MouseProperties {
    pub fn cursor_flags(&self) -> CursorFlags {
        self.cursor_flags
    }

    pub fn set_cursor_flags<F>(&mut self, window: HWND, f: F) -> Result<(), io::Error>
        where F: FnOnce(&mut CursorFlags)
    {
        let old_flags = self.cursor_flags;
        f(&mut self.cursor_flags);
        match self.cursor_flags.refresh_os_cursor(window) {
            Ok(()) => (),
            Err(e) => {
                self.cursor_flags = old_flags;
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn refresh_os_cursor(&self, window: HWND) -> Result<(), io::Error> {
        self.cursor_flags.refresh_os_cursor(window)?;
        Ok(())
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

    pub fn to_window_styles(self) -> (DWORD, DWORD) {
        use winapi::um::winuser::*;

        let (mut style, mut style_ex) = (0, 0);

        if self.contains(WindowFlags::RESIZABLE) {
            style |= winuser::WS_SIZEBOX | winuser::WS_MAXIMIZEBOX;
        }
        if self.contains(WindowFlags::DECORATIONS) {
            style |= WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_BORDER;
            style_ex = WS_EX_WINDOWEDGE;
        }
        if self.contains(WindowFlags::VISIBLE) {
            style |= WS_VISIBLE;
        }
        if self.contains(WindowFlags::ON_TASKBAR) {
            style_ex |= WS_EX_APPWINDOW;
        } else {
            style |= WS_POPUP;
        }
        if self.contains(WindowFlags::ALWAYS_ON_TOP) {
            style_ex |= WS_EX_TOPMOST;
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
        if self.contains(WindowFlags::MAXIMIZED) {
            style |= WS_MAXIMIZE;
        }

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
        if diff.contains(WindowFlags::ALWAYS_ON_TOP) {
            unsafe {
                winuser::SetWindowPos(
                    window,
                    match new.contains(WindowFlags::ALWAYS_ON_TOP) {
                        true  => winuser::HWND_TOPMOST,
                        false => winuser::HWND_NOTOPMOST,
                    },
                    0,
                    0,
                    0,
                    0,
                    winuser::SWP_ASYNCWINDOWPOS | winuser::SWP_NOMOVE | winuser::SWP_NOSIZE,
                );
                winuser::UpdateWindow(window);
            }
        }

        let (style, style_ex) = new.to_window_styles();
        unsafe{
            winuser::SetWindowLongW(window, winuser::GWL_STYLE, style as _);
            winuser::SetWindowLongW(window, winuser::GWL_EXSTYLE, style_ex as _);
        }
        unsafe {
            winuser::SetWindowPos(
                window,
                ::std::ptr::null_mut(),
                0, 0, 0, 0,
                winuser::SWP_NOMOVE | winuser::SWP_NOSIZE | winuser::SWP_NOZORDER | winuser::SWP_NOREDRAW | winuser::SWP_FRAMECHANGED
            );
        }
    }
}

impl CursorFlags {
    fn refresh_os_cursor(self, window: HWND) -> Result<(), io::Error> {
        let client_rect = util::get_client_rect(window)?;

        if util::is_focused(window) {
            if self.contains(CursorFlags::GRABBED) {
                util::set_cursor_clip(Some(client_rect))?;
            } else {
                util::set_cursor_clip(None);
            }
        }

        let cursor_in_client = util::get_cursor_pos()
            .map(|pos| util::rect_contains(client_rect, pos))
            .unwrap_or(false);
        if cursor_in_client {
            util::set_cursor_hidden(self.contains(CursorFlags::HIDDEN));
        }

        Ok(())
    }
}
