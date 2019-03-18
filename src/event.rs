//! The `Event` enum and assorted supporting types.
//!
//! These are sent to the closure given to [`EventLoop::run(...)`][event_loop_run], where they get
//! processed and used to modify the program state. For more details, see the root-level documentation.
//!
//! [event_loop_run]: ../event_loop/struct.EventLoop.html#method.run

/// This macro is a hack so that we can embed `winit-keyboard-map.svg` in the generated documentation.
/// It's necessary because attributes can't take macros as inputs, and this works around that.
macro_rules! keyboard_module {
    (
        include $doc_path:expr
    ) => {
        keyboard_module!(doc include_str!($doc_path));
    };
    (doc $doc_str:expr) => {
        /// Keyboard event types.
        ///
        /// `PhysicalKey` represents the physical location of the key, and is independent of keymap.
        /// The exact position of each key is shown on the chart below.
        ///
        /// `LogicalKey` represents the logical meaning of the key. For latin-script keyboards, the
        /// outputted alphanumeric key is the remapped latin letter for that keymap. The `Intl` keys
        /// are positioned with the following rules:
        /// - If the corresponding QWERTY key is in a different location in the keymap (e.g. DVORAK
        ///   semicolon), the code corresponds to that key.
        /// - Otherwise, the key appears in the physical location of the keymap, as shown in the chart.
        ///
        /// TODO BEFORE MERGE: MAKE ABOVE LINES MORE FRIENDLY AND DOCUMENT EACH TYPE IN DETAIL.
        ///
        /// TODO: ADD LINKS TO VARIANTS IN SVG
        #[doc = $doc_str]
        pub mod keyboard;
    };
}
keyboard_module!(include "./event/winit-keyboard-map.svg");

use std::time::Instant;
use std::path::PathBuf;

use dpi::{LogicalPosition, LogicalSize};
use self::keyboard::{InputEvent, ModifierState};
use platform_impl;
use window::WindowId;

pub mod device;

/// A generic event.
#[derive(Clone, Debug, PartialEq)]
pub enum Event<T> {
    /// Emitted when the OS sends an event to a winit window.
    WindowEvent {
        window_id: WindowId,
        event: WindowEvent,
    },

    /// Emitted when a mouse device has generated input.
    MouseEvent(device::MouseId, device::MouseEvent),
    /// Emitted when a keyboard device has generated input.
    KeyboardEvent(device::KeyboardId, device::KeyboardEvent),
    HidEvent(device::HidId, device::HidEvent),
    /// Emitted when a gamepad/joystick device has generated input.
    GamepadEvent(device::GamepadHandle, device::GamepadEvent),

    /// Emitted when an event is sent from [`EventLoopProxy::send_event`](../event_loop/struct.EventLoopProxy.html#method.send_event)
    UserEvent(T),
    /// Emitted when new events arrive from the OS to be processed.
    NewEvents(StartCause),
    /// Emitted when all of the event loop's events have been processed and control flow is about
    /// to be taken away from the program.
    EventsCleared,

    /// Emitted when the event loop is being shut down. This is irreversable - if this event is
    /// emitted, it is guaranteed to be the last event emitted.
    LoopDestroyed,

    /// Emitted when the application has been suspended or resumed.
    ///
    /// The parameter is true if app was suspended, and false if it has been resumed.
    Suspended(bool),
}

impl<T> Event<T> {
    pub fn map_nonuser_event<U>(self) -> Result<Event<U>, Event<T>> {
        use self::Event::*;
        match self {
            UserEvent(_) => Err(self),
            WindowEvent{window_id, event} => Ok(WindowEvent{window_id, event}),
            MouseEvent(id, event) => Ok(MouseEvent(id, event)),
            KeyboardEvent(id, event) => Ok(KeyboardEvent(id, event)),
            HidEvent(id, event) => Ok(HidEvent(id, event)),
            GamepadEvent(id, event) => Ok(GamepadEvent(id, event)),
            NewEvents(cause) => Ok(NewEvents(cause)),
            EventsCleared => Ok(EventsCleared),
            LoopDestroyed => Ok(LoopDestroyed),
            Suspended(suspended) => Ok(Suspended(suspended)),
        }
    }
}

/// The reason the event loop is resuming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartCause {
    /// Sent if the time specified by `ControlFlow::WaitUntil` has been reached. Contains the
    /// moment the timeout was requested and the requested resume time. The actual resume time is
    /// guaranteed to be equal to or after the requested resume time.
    ResumeTimeReached {
        start: Instant,
        requested_resume: Instant
    },

    /// Sent if the OS has new events to send to the window, after a wait was requested. Contains
    /// the moment the wait was requested and the resume time, if requested.
    WaitCancelled {
        start: Instant,
        requested_resume: Option<Instant>
    },

    /// Sent if the event loop is being resumed after the loop's control flow was set to
    /// `ControlFlow::Poll`.
    Poll,

    /// Sent once, immediately after `run` is called. Indicates that the loop was just initialized.
    Init
}

/// An event from a `Window`.
#[derive(Clone, Debug, PartialEq)]
pub enum WindowEvent {
    /// The size of the window has changed. Contains the client area's new dimensions.
    Resized(LogicalSize),

    /// The position of the window has changed. Contains the window's new position.
    Moved(LogicalPosition),

    /// The window has been requested to close.
    CloseRequested,

    /// The window has been destroyed.
    Destroyed,

    /// A file has been dropped into the window.
    ///
    /// When the user drops multiple files at once, this event will be emitted for each file
    /// separately.
    DroppedFile(PathBuf),

    /// A file is being hovered over the window.
    ///
    /// When the user hovers multiple files at once, this event will be emitted for each file
    /// separately.
    HoveredFile(PathBuf),

    /// A file was hovered, but has exited the window.
    ///
    /// There will be a single `HoveredFileCancelled` event triggered even if multiple files were
    /// hovered.
    HoveredFileCancelled,

    /// The window received a unicode character.
    ReceivedCharacter(char),

    /// The window gained or lost focus.
    ///
    /// The parameter is true if the window has gained focus, and false if it has lost focus.
    Focused(bool),

    /// An event from the keyboard has been received.
    KeyboardInput(InputEvent),

    /// The keymap has been changed, and any labels displayed to the user should be reloaded.
    KeymapChanged,

    /// The cursor has moved on the window.
    CursorMoved {
        /// (x,y) coords in pixels relative to the top-left corner of the window. Because the range of this data is
        /// limited by the display area and it may have been transformed by the OS to implement effects such as cursor
        /// acceleration, it should not be used to implement non-cursor-like interactions such as 3D camera control.
        position: LogicalPosition,
        modifiers: ModifierState
    },

    /// The cursor has entered the window.
    CursorEntered,

    /// The cursor has left the window.
    CursorLeft,

    /// A mouse wheel movement or touchpad scroll occurred.
    MouseWheel { delta: MouseScrollDelta, phase: TouchPhase, modifiers: ModifierState },

    /// An mouse button press has been received.
    MouseInput { state: ElementState, button: MouseButton, modifiers: ModifierState },


    /// Touchpad pressure event.
    ///
    /// At the moment, only supported on Apple forcetouch-capable macbooks.
    /// The parameters are: pressure level (value between 0 and 1 representing how hard the touchpad
    /// is being pressed) and stage (integer representing the click level).
    TouchpadPressure { pressure: f32, stage: i64 },

    /// The OS or application has requested that the window be redrawn.
    RedrawRequested,

    /// Touch event has been received
    Touch(Touch),

    /// The DPI factor of the window has changed.
    ///
    /// The following user actions can cause DPI changes:
    ///
    /// * Changing the display's resolution.
    /// * Changing the display's DPI factor (e.g. in Control Panel on Windows).
    /// * Moving the window to a display with a different DPI factor.
    ///
    /// For more information about DPI in general, see the [`dpi`](dpi/index.html) module.
    HiDpiFactorChanged(f64),
}

/// Represents raw hardware events that are not associated with any particular window.
///
/// Useful for interactions that diverge significantly from a conventional 2D GUI, such as 3D camera or first-person
/// game controls. Many physical actions, such as mouse movement, can produce both device and window events. Because
/// window events typically arise from virtual devices (corresponding to GUI cursors and keyboard focus) the device IDs
/// may not match.
///
/// Note that these events are delivered regardless of input focus.
#[derive(Clone, Debug, PartialEq)]
pub enum DeviceEvent {
    Added,
    Removed,

    /// Change in physical position of a pointing device.
    ///
    /// This represents raw, unfiltered physical motion. Not to be confused with `WindowEvent::CursorMoved`.
    MouseMotion {
        /// (x, y) change in position in unspecified units.
        ///
        /// Different devices may use different units.
        delta: (f64, f64),
    },

    /// Physical scroll event
    MouseWheel {
        delta: MouseScrollDelta,
    },

    /// Motion on some analog axis.  This event will be reported for all arbitrary input devices
    /// that winit supports on this platform, including mouse devices.  If the device is a mouse
    /// device then this will be reported alongside the MouseMotion event.
    Motion { axis: AxisId, value: f64 },

    Button { button: ButtonId, state: ElementState },
    Key(InputEvent),
    Text { codepoint: char },
}

/// Describes touch-screen input state.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    /// The touch has been cancelled by the OS.
    ///
    /// This can occur in a variety of situations, such as the window losing focus.
    Cancelled,
}

/// A touch event.
///
/// Every event is guaranteed to start with a `Start` event, and may end with either an `End` or
/// `Cancelled` event.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Touch {
    pub phase: TouchPhase,
    pub location: LogicalPosition,
    /// Unique identifier of a finger.
    ///
    /// This may get reused by the system after the touch ends.
    pub id: u64
}

/// Hardware-dependent keyboard scan code.
pub type ScanCode = u32;

/// Identifier for a specific analog axis on some device.
pub type AxisId = u32;

/// Identifier for a specific button on some device.
pub type ButtonId = u32;

/// The input state of a key or button.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ElementState {
    Pressed,
    Released,
}

/// A button on a mouse.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

/// A difference in the mouse scroll wheel state.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MouseScrollDelta {
	/// Amount in lines or rows to scroll in the horizontal
	/// and vertical directions.
	///
	/// Positive values indicate movement forward
	/// (away from the user) or rightwards.
	LineDelta(f32, f32),
	/// Amount in pixels to scroll in the horizontal and
	/// vertical direction.
	///
	/// Scroll events are expressed as a PixelDelta if
	/// supported by the device (eg. a touchpad) and
	/// platform.
	PixelDelta(LogicalPosition),
}
