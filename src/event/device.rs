//! Raw hardware events that are not associated with any particular window.
//!
//! Useful for interactions that diverge significantly from a conventional 2D GUI, such as 3D camera or first-person
//! game controls. Many physical actions, such as mouse movement, can produce both device and window events. Because
//! window events typically arise from virtual devices (corresponding to GUI cursors and keyboard focus) the device IDs
//! may not match.
//!
//! All attached devices are guaranteed to emit an `Added` event upon the initialization of the event loop.
//!
//! Note that device events are always delivered regardless of window focus.

use crate::{
    platform_impl,
    dpi::PhysicalPosition,
    event::{AxisId, ButtonId, ElementState, KeyboardInput, MouseButton},
    event_loop::EventLoop,
};
use std::{fmt, io};

/// A hint suggesting the type of button that was pressed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GamepadButton {
    Start,
    Select,

    /// The north face button.
    ///
    /// * Nintendo: X
    /// * Playstation: Triangle
    /// * XBox: Y
    North,
    /// The south face button.
    ///
    /// * Nintendo: B
    /// * Playstation: X
    /// * XBox: A
    South,
    /// The east face button.
    ///
    /// * Nintendo: A
    /// * Playstation: Circle
    /// * XBox: B
    East,
    /// The west face button.
    ///
    /// * Nintendo: Y
    /// * Playstation: Square
    /// * XBox: X
    West,

    LeftStick,
    RightStick,

    LeftTrigger,
    RightTrigger,

    LeftShoulder,
    RightShoulder,

    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

/// A hint suggesting the type of axis that moved.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,

    RightStickX,
    RightStickY,

    LeftTrigger,
    RightTrigger,

    // @francesca64 review: why were these variants here? I don't see how it makes sense for the dpad or hat switch
    // to have axes, since they're both four separate buttons.

    // /// This is supposed to have a specialized meaning, referring to a point-of-view switch present on joysticks used
    // /// for flight simulation. However, Xbox 360 controllers (and their derivatives) use a hat switch for the D-pad.
    // HatSwitch,
    // DPadUp,
    // DPadDown,
    // DPadLeft,
    // DPadRight,
}

/// A given joystick's side on the gamepad.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Side {
    Left,
    Right,
}

/// Raw mouse events.
///
/// See the module-level docs for more information.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseEvent {
    /// A mouse device has been added.
    Added,
    /// A mouse device has been removed.
    Removed,
    /// A mouse button has been pressed or released.
    Button {
        state: ElementState,
        button: MouseButton,
    },
    /// Relative change in physical position of a pointing device.
    ///
    /// This represents raw, unfiltered physical motion, NOT the position of the mouse. Accordingly,
    /// the values provided here are the change in position of the mouse since the previous
    /// `MovedRelative` event.
    MovedRelative(f64, f64),
    /// Change in absolute position of a pointing device.
    ///
    /// The `PhysicalPosition` value is the new position of the cursor relative to the desktop. This
    /// generally doesn't get output by standard mouse devices, but can get output from tablet devices.
    MovedAbsolute(PhysicalPosition),
    /// Change in rotation of mouse wheel.
    Wheel(f64, f64),
}

/// Raw keyboard events.
///
/// See the module-level docs for more information.
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum KeyboardEvent {
    /// A keyboard device has been added.
    Added,
    /// A keyboard device has been removed.
    Removed,
    /// A key has been pressed or released.
    Input(KeyboardInput),
}

/// Raw HID event.
///
/// See the module-level docs for more information.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum HidEvent {
    /// A Human Interface Device device has been added.
    Added,
    /// A Human Interface Device device has been removed.
    Removed,
    /// A raw data packet has been received from the Human Interface Device.
    Data(Box<[u8]>),
}

/// Gamepad/joystick events.
///
/// These can be generated by any of a variety of game controllers, including (but not limited to)
/// gamepads, joysicks, and HOTAS devices.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum GamepadEvent {
    /// A gamepad/joystick device has been added.
    Added,
    /// A gamepad/joystick device has been removed.
    Removed,
    /// An analog axis value on the gamepad/joystick has changed.
    ///
    /// Winit does NOT provide [deadzone filtering](https://www.quora.com/What-does-the-term-joystick-deadzone-mean),
    /// and such filtering may have to be provided by API users for joystick axes.
    Axis {
        axis_id: AxisId,
        /// A hint regarding the physical axis that moved.
        ///
        /// On traditional gamepads (such as an X360 controller) this can be assumed to have a
        /// non-`None` value; however, other joystick devices with more varied layouts generally won't
        /// provide a value here.
        ///
        /// TODO: DISCUSS CONTROLLER MAPPING ONCE WE FIGURE OUT WHAT WE'RE DOING THERE.
        axis: Option<GamepadAxis>,
        value: f64,
        /// Whether or not this axis has also produced a [`GamepadEvent::Stick`] event.
        stick: bool,
    },
    /// A two-axis joystick's value has changed.
    ///
    /// This is mainly provided to assist with deadzone calculation, as proper deadzones should be
    /// calculated via the combined distance of each joystick axis from the center of the joystick,
    /// rather than per-axis.
    ///
    /// Note that this is only guaranteed to be emitted for traditionally laid out gamepads. More
    /// complex joysticks generally don't report specifics of their layout to the operating system,
    /// preventing Winit from automatically aggregating their axis input into two-axis stick events.
    Stick {
        /// The X axis' ID.
        x_id: AxisId,
        /// The Y axis' ID.
        y_id: AxisId,
        x_value: f64,
        y_value: f64,
        /// Which joystick side produced this event.
        side: Side,
    },
    Button {
        button_id: ButtonId,
        /// A hint regarding the location of the button.
        ///
        /// The caveats on the `Axis.hint` field also apply here.
        button: Option<GamepadButton>,
        state: ElementState,
    },
}

/// Error reported if a rumble attempt unexpectedly failed.
#[derive(Debug)]
pub enum RumbleError {
    /// The device is no longer connected.
    DeviceNotConnected,
    /// An unknown OS error has occured.
    OsError(io::Error),
}

/// A typed identifier for a mouse device.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MouseId(pub(crate) platform_impl::MouseId);
/// A typed identifier for a keyboard device.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyboardId(pub(crate) platform_impl::KeyboardId);
/// A typed if for a Human Interface Device.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HidId(pub(crate) platform_impl::HidId);
/// A handle to a gamepad/joystick device.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GamepadHandle(pub(crate) platform_impl::GamepadHandle);

impl MouseId {
    /// Returns a dummy `MouseId`, useful for unit testing. The only guarantee made about the return
    /// value of this function is that it will always be equal to itself and to future values returned
    /// by this function.  No other guarantees are made. This may be equal to a real `MouseId`.
    ///
    /// **Passing this into a winit function will result in undefined behavior.**
    pub unsafe fn dummy() -> Self {
        MouseId(platform_impl::MouseId::dummy())
    }

    /// Enumerate all attached mouse devices.
    pub fn enumerate<T>(event_loop: &EventLoop<T>) -> impl '_ + Iterator<Item=Self> {
        platform_impl::MouseId::enumerate(&event_loop.event_loop)
    }

    /// Check to see if this mouse device is still connected.
    pub fn is_connected(&self) -> bool {
        self.0.is_connected()
    }
}

impl KeyboardId {
    /// Returns a dummy `KeyboardId`, useful for unit testing. The only guarantee made about the return
    /// value of this function is that it will always be equal to itself and to future values returned
    /// by this function.  No other guarantees are made. This may be equal to a real `KeyboardId`.
    ///
    /// **Passing this into a winit function will result in undefined behavior.**
    pub unsafe fn dummy() -> Self {
        KeyboardId(platform_impl::KeyboardId::dummy())
    }

    /// Enumerate all attached keyboard devices.
    pub fn enumerate<T>(event_loop: &EventLoop<T>) -> impl '_ + Iterator<Item=Self> {
        platform_impl::KeyboardId::enumerate(&event_loop.event_loop)
    }

    /// Check to see if this keyboard device is still connected.
    pub fn is_connected(&self) -> bool {
        self.0.is_connected()
    }
}

impl HidId {
    /// Returns a dummy `HidId`, useful for unit testing. The only guarantee made about the return
    /// value of this function is that it will always be equal to itself and to future values returned
    /// by this function.  No other guarantees are made. This may be equal to a real `HidId`.
    ///
    /// **Passing this into a winit function will result in undefined behavior.**
    pub unsafe fn dummy() -> Self {
        HidId(platform_impl::HidId::dummy())
    }

    /// Enumerate all attached keyboard devices.
    pub fn enumerate<T>(event_loop: &EventLoop<T>) -> impl '_ + Iterator<Item=Self> {
        platform_impl::HidId::enumerate(&event_loop.event_loop)
    }

    /// Check to see if this keyboard device is still connected.
    pub fn is_connected(&self) -> bool {
        self.0.is_connected()
    }
}

impl GamepadHandle {
    /// Returns a dummy `GamepadHandle`, useful for unit testing. The only guarantee made about the return
    /// value of this function is that it will always be equal to itself and to future values returned
    /// by this function.  No other guarantees are made. This may be equal to a real `GamepadHandle`.
    ///
    /// **Passing this into a winit function will result in undefined behavior.**
    pub unsafe fn dummy() -> Self {
        GamepadHandle(platform_impl::GamepadHandle::dummy())
    }

    /// Enumerate all attached gamepad/joystick devices.
    pub fn enumerate<T>(event_loop: &EventLoop<T>) -> impl '_ + Iterator<Item=Self> {
        platform_impl::GamepadHandle::enumerate(&event_loop.event_loop)
    }

    /// Check to see if this gamepad/joystick device is still connected.
    pub fn is_connected(&self) -> bool {
        self.0.is_connected()
    }

    /// Attempts to set the rumble values for an attached controller. Input values are automatically
    /// bound to a [`0.0`, `1.0`] range.
    ///
    /// Certain gamepads assign different usages to the left and right motors - for example, X360
    /// controllers treat the left motor as a low-frequency rumble and the right motor as a
    /// high-frequency rumble. However, this cannot necessarily be assumed for all gamepad devices.
    ///
    /// Note that, if the given gamepad does not support rumble, no error value gets thrown.
    pub fn rumble(&self, left_speed: f64, right_speed: f64) -> Result<(), RumbleError> {
        self.0.rumble(left_speed, right_speed)
    }

    /// Gets the port number assigned to the gamepad.
    pub fn port(&self) -> Option<u8> {
        self.0.port()
    }

    /// Gets the controller's battery level.
    ///
    /// If the controller doesn't report a battery level, this returns `None`.
    pub fn battery_level(&self) -> Option<BatteryLevel> {
        self.0.battery_level()
    }
}

/// TODO: IS THIS THE RIGHT ABSTRACTION FOR ALL PLATFORMS?
/// This is exposed in its current form because it's what Microsoft does for XInput, and that's my
/// (@Osspial's) main point of reference. If you're implementing this on a different platform and
/// that platform exposes battery level differently, please bring it up in the tracking issue!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BatteryLevel {
    Empty,
    Low,
    Medium,
    Full
}

impl fmt::Debug for MouseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl fmt::Debug for KeyboardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl fmt::Debug for HidId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl fmt::Debug for GamepadHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}
