use std::any::Any;

use super::{KWindowError, KEvent};


/// Enumeration of possible [KWindowManager] provider id.
/// 
/// Since [KWindowManagerId] are u8, user can use his own value > 6
/// when porting to another platform.
#[allow(non_snake_case)]
pub mod KWindowManagerId {
    /// Linux Wayland display server.
    pub const WAYLAND : u8 = 1;

    /// Linux X11 display server.
    pub const X11 : u8 = 2;

    /// Windows graphical user interface.
    pub const SHELL : u8 = 3;

    /// Android graphical user interface.
    pub const ANDROID : u8 = 4;

    /// MacOS graphical user interface
    pub const MACOS : u8 = 5;

    /// IOS graphical user interface
    pub const IOS : u8 = 6;
}

/// # Abstraction of window manager that supplies window frame.
pub trait KWindowManager {
    /// Create a new instance of [KWindowManager] and open the window frame.
    /// 
    /// Returns [KWindowManager] created.
    fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<Self, KWindowError> where Self: Sized;

    /// Returns count of events to be polled.
    fn get_event_count(&self) -> usize;

    /// Get an event from window manager as [KEvent].
    fn poll_event(&mut self) -> KEvent;

    /// Sync event with display manager.
    fn sync_event(&self);

    /// Get the [KWindowManagerId] that manage the window.
    fn get_id(&self) -> u8;

    /// Get self as [Any], allowing downcasting.
    /// 
    /// Implementation code should be as follow :
    /// ```no_run
    /// fn as_any(&self) -> &dyn Any {
    ///     self
    /// }
    /// ```
    /// 
    /// # Example(s)
    /// Downcasting to KWindowManagerX11
    /// ```no_run
    /// match window.get_window_manager().as_any().downcast_ref::<KWindowManagerX11>() {
    /// Some(dc) => {
    ///    println!("DC Worked! Proof, display={:?}", dc.get_display());
    ///     },
    ///     None => panic!("DC Failed!"),
    /// }
    /// ```
    fn as_any(&self) -> &dyn Any;
}

