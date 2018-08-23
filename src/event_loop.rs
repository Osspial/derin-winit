/// Provides a way to retrieve events from the system and from the windows that were registered to
/// the events loop.
///
/// An `EventLoop` can be seen more or less as a "context". Calling `EventLoop::new()`
/// initializes everything that will be required to create windows. For example on Linux creating
/// an events loop opens a connection to the X or Wayland server.
///
/// To wake up an `EventLoop` from a another thread, see the `EventLoopProxy` docs.
///
/// Note that the `EventLoop` cannot be shared accross threads (due to platform-dependant logic
/// forbiding it), as such it is neither `Send` nor `Sync`. If you need cross-thread access, the
/// `Window` created from this `EventLoop` _can_ be sent to an other thread, and the
/// `EventLoopProxy` allows you to wakeup an `EventLoop` from an other thread.
pub struct EventLoop<T> {
    events_loop: platform_impl::EventLoop<T>,
    _marker: ::std::marker::PhantomData<*mut ()> // Not Send nor Sync
}

/// Returned by the user callback given to the `EventLoop::run_forever` method.
///
/// Indicates whether the `run_forever` method should continue or complete.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ControlFlow {
    /// When the current loop iteration finishes, suspend the thread until another event arrives.
    Wait,
    /// When the current loop iteration finishes, suspend the thread until either another event
    /// arrives or the given time is reached.
    WaitUntil(Instant),
    /// When the current loop iteration finishes, immediately begin a new iteration regardless of
    /// whether or not new events are available to process.
    Poll,
    /// Send a `LoopDestroyed` event and stop the event loop.
    Exit
}

impl Default for ControlFlow {
    #[inline(always)]
    fn default() -> ControlFlow {
        ControlFlow::Poll
    }
}

impl EventLoop<()> {
    pub fn new() -> EventLoop<()> {
        EventLoop::<()>::new_user_event()
    }
}

impl<T> EventLoop<T> {
    /// Builds a new events loop.
    ///
    /// Usage will result in display backend initialisation, this can be controlled on linux
    /// using an environment variable `WINIT_UNIX_BACKEND`. Legal values are `x11` and `wayland`.
    /// If it is not set, winit will try to connect to a wayland connection, and if it fails will
    /// fallback on x11. If this variable is set with any other value, winit will panic.
    pub fn new_user_event() -> EventLoop<T> {
        EventLoop {
            events_loop: platform_impl::EventLoop::new(),
            _marker: ::std::marker::PhantomData,
        }
    }

    /// Returns the list of all the monitors available on the system.
    ///
    // Note: should be replaced with `-> impl Iterator` once stable.
    #[inline]
    pub fn get_available_monitors(&self) -> AvailableMonitorsIter {
        let data = self.events_loop.get_available_monitors();
        AvailableMonitorsIter{ data: data.into_iter() }
    }

    /// Returns the primary monitor of the system.
    #[inline]
    pub fn get_primary_monitor(&self) -> MonitorId {
        MonitorId { inner: self.events_loop.get_primary_monitor() }
    }

    /// Hijacks the calling thread and initializes the `winit` event loop. Can take a
    /// `FnMut(Event, &EventLoop) -> ControlFlow` or a custom `EventHandler` type.
    ///
    /// Any values not passed to this function will *not* be dropped.
    #[inline]
    pub fn run<F>(self, event_handler: F) -> !
        where F: 'static + FnMut(Event<T>, &EventLoop<T>, &mut ControlFlow)
    {
        self.events_loop.run(event_handler)
    }

    /// Creates an `EventLoopProxy` that can be used to wake up the `EventLoop` from another
    /// thread.
    pub fn create_proxy(&self) -> EventLoopProxy<T> {
        EventLoopProxy {
            events_loop_proxy: self.events_loop.create_proxy(),
        }
    }
}

/// Used to wake up the `EventLoop` from another thread.
#[derive(Clone)]
pub struct EventLoopProxy<T> {
    events_loop_proxy: platform_impl::EventLoopProxy<T>,
}

impl<T> EventLoopProxy<T> {
    /// Send an event to the `EventLoop` from which this proxy was created. This emits a
    /// `UserEvent(event)` event in the event loop, where `event` is the value passed to this
    /// function.
    ///
    /// Returns an `Err` if the associated `EventLoop` no longer exists.
    pub fn send_event(&self, event: T) -> Result<(), EventLoopClosed> {
        self.events_loop_proxy.send_event(event)
    }
}

/// The error that is returned when an `EventLoopProxy` attempts to wake up an `EventLoop` that
/// no longer exists.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EventLoopClosed;

impl std::fmt::Display for EventLoopClosed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", std::error::Error::description(self))
    }
}

impl std::error::Error for EventLoopClosed {
    fn description(&self) -> &str {
        "Tried to wake up a closed `EventLoop`"
    }
}

