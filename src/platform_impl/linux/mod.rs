#![cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]

use std::collections::VecDeque;
use std::{env, mem};
use std::ffi::CStr;
use std::os::raw::*;
use std::sync::Arc;

use parking_lot::Mutex;
use sctk::reexports::client::ConnectError;

use dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize};
use icon::Icon;
use event_loop::{EventLoopClosed, ControlFlow, EventLoopWindowTarget as RootELW};
use monitor::MonitorHandle as RootMonitorHandle;
use window::{WindowAttributes, CreationError, MouseCursor};
//use self::x11::{XConnection, XError};
//use self::x11::ffi::XVisualInfo;
//pub use self::x11::XNotSupported;

mod dlopen;
pub mod wayland;
//pub mod x11;

/// Environment variable specifying which backend should be used on unix platform.
///
/// Legal values are x11 and wayland. If this variable is set only the named backend
/// will be tried by winit. If it is not set, winit will try to connect to a wayland connection,
/// and if it fails will fallback on x11.
///
/// If this variable is set with any other value, winit will panic.
const BACKEND_PREFERENCE_ENV_VAR: &str = "WINIT_UNIX_BACKEND";

#[derive(Clone, Default)]
pub struct PlatformSpecificWindowBuilderAttributes {
    //pub visual_infos: Option<XVisualInfo>,
    pub screen_id: Option<i32>,
    pub resize_increments: Option<(u32, u32)>,
    pub base_size: Option<(u32, u32)>,
    pub class: Option<(String, String)>,
    pub override_redirect: bool,
    //pub x11_window_type: x11::util::WindowType,
    pub gtk_theme_variant: Option<String>,
    pub app_id: Option<String>
}

//lazy_static!(
//    pub static ref X11_BACKEND: Mutex<Result<Arc<XConnection>, XNotSupported>> = {
//        Mutex::new(XConnection::new(Some(x_error_callback)).map(Arc::new))
//    };
//);

pub enum Window {
    //X(x11::Window),
    Wayland(wayland::Window),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowId {
    //X(x11::WindowId),
    Wayland(wayland::WindowId),
}

impl WindowId {
    pub unsafe fn dummy() -> Self {
        WindowId::Wayland(wayland::WindowId::dummy())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeviceId {
    //X(x11::DeviceId),
    Wayland(wayland::DeviceId),
}

impl DeviceId {
    pub unsafe fn dummy() -> Self {
        DeviceId::Wayland(wayland::DeviceId::dummy())
    }
}

#[derive(Debug, Clone)]
pub enum MonitorHandle {
    //X(x11::MonitorHandle),
    Wayland(wayland::MonitorHandle),
}

impl MonitorHandle {
    #[inline]
    pub fn get_name(&self) -> Option<String> {
        match self {
            //&MonitorHandle::X(ref m) => m.get_name(),
            &MonitorHandle::Wayland(ref m) => m.get_name(),
        }
    }

    #[inline]
    pub fn get_native_identifier(&self) -> u32 {
        match self {
            //&MonitorHandle::X(ref m) => m.get_native_identifier(),
            &MonitorHandle::Wayland(ref m) => m.get_native_identifier(),
        }
    }

    #[inline]
    pub fn get_size(&self) -> PhysicalSize {
        match self {
            //&MonitorHandle::X(ref m) => m.get_size(),
            &MonitorHandle::Wayland(ref m) => m.get_size(),
        }
    }

    #[inline]
    pub fn get_position(&self) -> PhysicalPosition {
        match self {
            //&MonitorHandle::X(ref m) => m.get_position(),
            &MonitorHandle::Wayland(ref m) => m.get_position(),
        }
    }

    #[inline]
    pub fn get_hidpi_factor(&self) -> f64 {
        match self {
            //&MonitorHandle::X(ref m) => m.get_hidpi_factor(),
            &MonitorHandle::Wayland(ref m) => m.get_hidpi_factor() as f64,
        }
    }
}

impl Window {
    #[inline]
    pub fn new<T>(
        window_target: &EventLoopWindowTarget<T>,
        attribs: WindowAttributes,
        pl_attribs: PlatformSpecificWindowBuilderAttributes,
    ) -> Result<Self, CreationError> {
        match *window_target {
            EventLoopWindowTarget::Wayland(ref window_target) => {
                wayland::Window::new(window_target, attribs, pl_attribs).map(Window::Wayland)
            },
            //EventLoop::X(ref event_loop) => {
            //    x11::Window::new(event_loop, attribs, pl_attribs).map(Window::X)
            //},
        }
    }

    #[inline]
    pub fn id(&self) -> WindowId {
        match self {
            //&Window::X(ref w) => WindowId::X(w.id()),
            &Window::Wayland(ref w) => WindowId::Wayland(w.id()),
        }
    }

    #[inline]
    pub fn set_title(&self, title: &str) {
        match self {
            //&Window::X(ref w) => w.set_title(title),
            &Window::Wayland(ref w) => w.set_title(title),
        }
    }

    #[inline]
    pub fn show(&self) {
        match self {
            //&Window::X(ref w) => w.show(),
            &Window::Wayland(ref w) => w.show(),
        }
    }

    #[inline]
    pub fn hide(&self) {
        match self {
            //&Window::X(ref w) => w.hide(),
            &Window::Wayland(ref w) => w.hide(),
        }
    }

    #[inline]
    pub fn get_outer_position(&self) -> Option<LogicalPosition> {
        match self {
            //&Window::X(ref w) => w.get_position(),
            &Window::Wayland(ref w) => w.get_position(),
        }
    }

    #[inline]
    pub fn get_inner_position(&self) -> Option<LogicalPosition> {
        match self {
            //&Window::X(ref m) => m.get_inner_position(),
            &Window::Wayland(ref m) => m.get_inner_position(),
        }
    }

    #[inline]
    pub fn set_outer_position(&self, position: LogicalPosition) {
        match self {
            //&Window::X(ref w) => w.set_position(position),
            &Window::Wayland(ref w) => w.set_position(position),
        }
    }

    #[inline]
    pub fn get_inner_size(&self) -> Option<LogicalSize> {
        match self {
            //&Window::X(ref w) => w.get_inner_size(),
            &Window::Wayland(ref w) => w.get_inner_size(),
        }
    }

    #[inline]
    pub fn get_outer_size(&self) -> Option<LogicalSize> {
        match self {
            //&Window::X(ref w) => w.get_outer_size(),
            &Window::Wayland(ref w) => w.get_outer_size(),
        }
    }

    #[inline]
    pub fn set_inner_size(&self, size: LogicalSize) {
        match self {
            //&Window::X(ref w) => w.set_inner_size(size),
            &Window::Wayland(ref w) => w.set_inner_size(size),
        }
    }

    #[inline]
    pub fn set_min_inner_size(&self, dimensions: Option<LogicalSize>) {
        match self {
            //&Window::X(ref w) => w.set_min_inner_size(dimensions),
            &Window::Wayland(ref w) => w.set_min_inner_size(dimensions),
        }
    }

    #[inline]
    pub fn set_max_inner_size(&self, dimensions: Option<LogicalSize>) {
        match self {
            //&Window::X(ref w) => w.set_max_inner_size(dimensions),
            &Window::Wayland(ref w) => w.set_max_inner_size(dimensions),
        }
    }

    #[inline]
    pub fn set_resizable(&self, resizable: bool) {
        match self {
            //&Window::X(ref w) => w.set_resizable(resizable),
            &Window::Wayland(ref w) => w.set_resizable(resizable),
        }
    }

    #[inline]
    pub fn set_cursor_icon(&self, cursor: CursorIcon) {
        match self {
            //&Window::X(ref w) => w.set_cursor_icon(cursor),
            &Window::Wayland(ref w) => w.set_cursor_icon(cursor)
        }
    }

    #[inline]
    pub fn set_cursor_grab(&self, grab: bool) -> Result<(), String> {
        match self {
            //&Window::X(ref window) => window.set_cursor_grab(grab),
            &Window::Wayland(ref window) => window.set_cursor_grab(grab),
        }
    }

    #[inline]
    pub fn set_cursor_visible(&self, visible: bool) {
        match self {
            //&Window::X(ref window) => window.set_cursor_visible(visible),
            &Window::Wayland(ref window) => window.set_cursor_visible(visible),
        }
    }

    #[inline]
    pub fn get_hidpi_factor(&self) -> f64 {
       match self {
            //&Window::X(ref w) => w.get_hidpi_factor(),
            &Window::Wayland(ref w) => w.hidpi_factor() as f64,
        }
    }

    #[inline]
    pub fn set_cursor_position(&self, position: LogicalPosition) -> Result<(), String> {
        match self {
            //&Window::X(ref w) => w.set_cursor_position(position),
            &Window::Wayland(ref w) => w.set_cursor_position(position),
        }
    }

    #[inline]
    pub fn set_maximized(&self, maximized: bool) {
        match self {
            //&Window::X(ref w) => w.set_maximized(maximized),
            &Window::Wayland(ref w) => w.set_maximized(maximized),
        }
    }

    #[inline]
    pub fn set_fullscreen(&self, monitor: Option<RootMonitorHandle>) {
        match self {
            //&Window::X(ref w) => w.set_fullscreen(monitor),
            &Window::Wayland(ref w) => w.set_fullscreen(monitor)
        }
    }

    #[inline]
    pub fn set_decorations(&self, decorations: bool) {
        match self {
            //&Window::X(ref w) => w.set_decorations(decorations),
            &Window::Wayland(ref w) => w.set_decorations(decorations)
        }
    }

    #[inline]
    pub fn set_always_on_top(&self, always_on_top: bool) {
        match self {
            //&Window::X(ref w) => w.set_always_on_top(always_on_top),
            &Window::Wayland(_) => (),
        }
    }

    #[inline]
    pub fn set_window_icon(&self, window_icon: Option<Icon>) {
        match self {
            //&Window::X(ref w) => w.set_window_icon(window_icon),
            &Window::Wayland(_) => (),
        }
    }

    #[inline]
    pub fn set_ime_position(&self, position: LogicalPosition) {
        match self {
            //&Window::X(ref w) => w.set_ime_position(position),
            &Window::Wayland(_) => (),
        }
    }

    #[inline]
    pub fn request_redraw(&self) {
        match self {
            //&Window::X(ref w) => w.request_redraw(),
            &Window::Wayland(ref w) => w.request_redraw(),
        }
    }

    #[inline]
    pub fn get_current_monitor(&self) -> RootMonitorHandle {
        match self {
            //&Window::X(ref window) => RootMonitorHandle { inner: MonitorHandle::X(window.get_current_monitor()) },
            &Window::Wayland(ref window) => RootMonitorHandle { inner: MonitorHandle::Wayland(window.get_current_monitor()) },
        }
    }

    #[inline]
    pub fn get_available_monitors(&self) -> VecDeque<MonitorHandle> {
        match self {
            //&Window::X(ref window) => window.get_available_monitors()
            //    .into_iter()
            //    .map(MonitorHandle::X)
            //    .collect(),
            &Window::Wayland(ref window) => window.get_available_monitors()
                .into_iter()
                .map(MonitorHandle::Wayland)
                .collect(),
        }
    }

    #[inline]
    pub fn get_primary_monitor(&self) -> MonitorHandle {
        match self {
            //&Window::X(ref window) => MonitorHandle::X(window.get_primary_monitor()),
            &Window::Wayland(ref window) => MonitorHandle::Wayland(window.get_primary_monitor()),
        }
    }
}

/*
unsafe extern "C" fn x_error_callback(
    display: *mut x11::ffi::Display,
    event: *mut x11::ffi::XErrorEvent,
) -> c_int {
    let xconn_lock = X11_BACKEND.lock();
    if let Ok(ref xconn) = *xconn_lock {
        let mut buf: [c_char; 1024] = mem::uninitialized();
        (xconn.xlib.XGetErrorText)(
            display,
            (*event).error_code as c_int,
            buf.as_mut_ptr(),
            buf.len() as c_int,
        );
        let description = CStr::from_ptr(buf.as_ptr()).to_string_lossy();

        let error = XError {
            description: description.into_owned(),
            error_code: (*event).error_code,
            request_code: (*event).request_code,
            minor_code: (*event).minor_code,
        };

        error!("X11 error: {:#?}", error);

        *xconn.latest_error.lock() = Some(error);
    }
    // Fun fact: this return value is completely ignored.
    0
}
*/

pub enum EventLoop<T: 'static> {
    Wayland(wayland::EventLoop<T>),
    //X(x11::EventLoop)
}

#[derive(Clone)]
pub enum EventLoopProxy<T: 'static> {
    //X(x11::EventLoopProxy),
    Wayland(wayland::EventLoopProxy<T>),
}

impl<T:'static> EventLoop<T> {
    pub fn new() -> EventLoop<T> {
        if let Ok(env_var) = env::var(BACKEND_PREFERENCE_ENV_VAR) {
            match env_var.as_str() {
                "x11" => {
                    // TODO: propagate
                    return EventLoop::new_x11().expect("Failed to initialize X11 backend");
                },
                "wayland" => {
                    return EventLoop::new_wayland()
                        .expect("Failed to initialize Wayland backend");
                },
                _ => panic!(
                    "Unknown environment variable value for {}, try one of `x11`,`wayland`",
                    BACKEND_PREFERENCE_ENV_VAR,
                ),
            }
        }

        let wayland_err = match EventLoop::new_wayland() {
            Ok(event_loop) => return event_loop,
            Err(err) => err,
        };

        let x11_err = match EventLoop::new_x11() {
            Ok(event_loop) => return event_loop,
            Err(err) => err,
        };

        let err_string = format!(
            "Failed to initialize any backend! Wayland status: {:?} X11 status: {:?}",
            wayland_err,
            x11_err,
        );
        panic!(err_string);
    }

    pub fn new_wayland() -> Result<EventLoop<T>, ConnectError> {
        wayland::EventLoop::new()
            .map(EventLoop::Wayland)
    }

    pub fn new_x11() -> Result<EventLoop<T>, () /*XNotSupported*/> {
        //X11_BACKEND
        //    .lock()
        //    .as_ref()
        //    .map(Arc::clone)
        //    .map(x11::EventLoop::new)
        //    .map(EventLoop::X)
        //    .map_err(|err| err.clone())
        unimplemented!()
    }

    #[inline]
    pub fn get_available_monitors(&self) -> VecDeque<MonitorHandle> {
        match *self {
            EventLoop::Wayland(ref evlp) => evlp
                .get_available_monitors()
                .into_iter()
                .map(MonitorHandle::Wayland)
                .collect(),
            //EventLoop::X(ref evlp) => evlp
            //    .x_connection()
            //    .get_available_monitors()
            //    .into_iter()
            //    .map(MonitorHandle::X)
            //    .collect(),
        }
    }

    #[inline]
    pub fn get_primary_monitor(&self) -> MonitorHandle {
        match *self {
            EventLoop::Wayland(ref evlp) => MonitorHandle::Wayland(evlp.get_primary_monitor()),
            //EventLoop::X(ref evlp) => MonitorHandle::X(evlp.x_connection().get_primary_monitor()),
        }
    }

    pub fn create_proxy(&self) -> EventLoopProxy<T> {
        match *self {
            EventLoop::Wayland(ref evlp) => EventLoopProxy::Wayland(evlp.create_proxy()),
            //EventLoop::X(ref evlp) => EventLoopProxy::X(evlp.create_proxy()),
        }
    }

    pub fn run_return<F>(&mut self, callback: F)
        where F: FnMut(::event::Event<T>, &RootELW<T>, &mut ControlFlow)
    {
        match *self {
            EventLoop::Wayland(ref mut evlp) => evlp.run_return(callback),
            //EventLoop::X(ref mut evlp) => evlp.run_return(callback)
        }
    }

    pub fn run<F>(self, callback: F) -> !
        where F: 'static + FnMut(::event::Event<T>, &RootELW<T>, &mut ControlFlow)
    {
        match self {
            EventLoop::Wayland(evlp) => evlp.run(callback),
            //EventLoop::X(ref mut evlp) => evlp.run(callback)
        }
    }

    #[inline]
    pub fn is_wayland(&self) -> bool {
        match *self {
            EventLoop::Wayland(_) => true,
            //EventLoop::X(_) => false,
        }
    }

    pub fn window_target(&self) -> &::event_loop::EventLoopWindowTarget<T> {
        match *self {
            EventLoop::Wayland(ref evl) => evl.window_target(),
            //EventLoop::X(ref evl) => evl.window_target()
        }
    }

    //#[inline]
    //pub fn x_connection(&self) -> Option<&Arc<XConnection>> {
    //    match *self {
    //        EventLoop::Wayland(_) => None,
    //        EventLoop::X(ref ev) => Some(ev.x_connection()),
    //    }
    //}
}

impl<T: 'static> EventLoopProxy<T> {
    pub fn send_event(&self, event: T) -> Result<(), EventLoopClosed> {
        match *self {
            EventLoopProxy::Wayland(ref proxy) => proxy.send_event(event),
            //EventLoopProxy::X(ref proxy) => proxy.wakeup(),
        }
    }
}

pub enum EventLoopWindowTarget<T> {
    Wayland(wayland::EventLoopWindowTarget<T>),
    //X(x11::EventLoopWIndowTarget<T>)
}
