use std::any::Any;
use self::event::dispatcher::KEventDispatcher;

/// # Re-export for Public API
#[doc(inline)]
pub use renderer::KWindowRenderer as KWindowRenderer;
pub use event::KEvent as KEvent;
pub use event::mouse::KEventMouse as KEventMouse;
pub use event::window::KEventWindow as KEventWindow;
pub use event::controller::KEventController as KEventController;
pub use event::keyboard::KEventKeyboard as KEventKeyboard;
pub use event::dispatcher::KEventReceiver as KEventReceiver;

/// [KWindow] event definition.
#[doc(hidden)]
pub mod event;

/// [KWindow] renderer abstraction.
#[doc(hidden)]
pub mod renderer;

/// Linux implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "linux"))]
#[doc(hidden)]
pub mod linux;

/// Windows shell implementations of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "windows"))]
#[doc(hidden)]
pub mod shell;

/// Android implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "android"))]
#[doc(hidden)]
pub mod android;

/// IOS implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "ios"))]
#[doc(hidden)]
pub mod ios;

/// MacOS implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "macos"))]
#[doc(hidden)]
pub mod macos;

/// Web assembly implementation of KWindow
#[cfg(target_family = "wasm")]
#[doc(hidden)]
pub mod wasm;


/// Enumeration of possible [KWindow] errors.
pub enum KWindowError {

    /// Happens when a window manager is not supported.
    NotSupported,

    /// Happens when no display server is found.
    NoDisplayServer,

    /// Happens when an error occurred while creating a [KWindow] using KWindow::from().
    FromWindowManagerError,



}


/// # Create and manage a window frame in OS GUI.
/// 
/// [KWindow] broadcasts [KEvent] to multiple [KEventReceiver] via [KWindow::dispatch_events()].
/// 
/// TODO: More doc about OS, dispatch, from and window manager and Examples
pub struct KWindow {
    
    /// Dispatcher of KEvent to receivers.
    dispatcher : KEventDispatcher,

    /// Window manager that manage this [KWindow].
    window_manager : Box<dyn KWindowManager>,
}

impl KWindow {
    /// Create a new [KWindow] using position and size.
    /// 
    /// Return New [`KWindow`].
    /// 
    /// 
    /// # Error(s)
    /// Returns [KWindowError::NoDisplayServer] if no display server found on Linux.
    /// 
    /// # Note(s)
    /// On Linux distribution, this will try to create a [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) window first then
    /// a [x11](https://en.wikipedia.org/wiki/X_Window_System) window if not compatible with Wayland.
    #[cfg(all(not(target_family = "wasm"), target_os = "linux"))]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {

        match linux::wayland::KWindowManagerWayland::new(pos_x, pos_y, width, height) {
            Ok(wm) => {
                Ok(KWindow{ dispatcher : KEventDispatcher::new(), window_manager: Box::new(wm) })
            },
            // Try to create X11 Window Manager
            Err(_) => match linux::x11::KWindowManagerX11::new(pos_x, pos_y, width, height) {
                Ok(wm) => {
                    Ok(KWindow{ dispatcher : KEventDispatcher::new(), window_manager: Box::new(wm) })
                },

                // If failed, no display server is available.
                Err(_) => Err(KWindowError::NoDisplayServer),
            },
        } 
    }

    #[cfg(all(not(target_family = "wasm"), target_os = "windows"))]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {
        todo!()
    }

    #[cfg(all(not(target_family = "wasm"), target_os = "android"))]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {
        todo!()
    }

    #[cfg(all(not(target_family = "wasm"), target_os = "ios"))]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {
        todo!()
    }

    #[cfg(all(not(target_family = "wasm"), target_os = "macos"))]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {
        todo!()
    }

    #[cfg(target_family = "wasm")]
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {
        todo!()
    }

    /// Returns the [KWindowManagerId] of the [KWindowManager].
    /// 
    /// Used by [KWindowRenderer] to target the correct display server.
    pub fn get_window_manager_id(&self) -> u8 {
        self.window_manager.get_id()
    }

    /// Used to downcast [KWindowManager] trait to target the correct display server for [KWindowRenderer].
    /// 
    /// Returns some reference to the inner value if it is of type T, or None if it isn’t.
    pub fn downcast_window_manager<T: Any>(&self) -> Option<&T> {
        self.window_manager.as_any().downcast_ref::<T>()
    }

    /// Create a [KWindow] from a [KWindowManager]. 
    /// 
    /// The [KWindow] will take ownership of the [KWindowManager].
    /// 
    /// Used when porting [KWindow] to another platform.
    pub fn from(window_manager : Box<dyn KWindowManager>) -> KWindow {
        KWindow { dispatcher : KEventDispatcher::new(), window_manager }
    }



    /// Dispatch [KEvent] to [KEventReceiver].
    /// 
    /// This function should be called at the beginning of each main loop. 
    /// TODO:Example
    pub fn dispatch_events(&mut self) -> KEvent {
        todo!()
    }




}

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

    /// Get an event from window manager as [KEvent].
    fn poll_event(&mut self) -> KEvent;

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


#[cfg(test)]
mod test {
    
    use super::{KWindow, linux::x11::KWindowManagerX11, KWindowManagerId};


    #[test]
    #[ignore]
    fn kwindow_test() {
        let window = KWindow::new(100, 100, 200, 200);

        match window {
            Ok(mut window) => {
                if window.get_window_manager().get_id() == KWindowManagerId::X11 {
                    match window.get_window_manager().as_any().downcast_ref::<KWindowManagerX11>() {
                        Some(dc) => {
                            
                            println!("DC Worked! Proof, display={:?}", dc.get_display());
                        },
                        None => panic!("DC Failed!"),
                    }
                }
                loop {
                    //window.poll_event();
                }
            },
            Err(_) => panic!("Error while creating window!"),
        }

    
    }
    
}


/*
pub struct KWindowProperty {

    // Windows handle id
    handle : usize,

    // Window title
    title : String,

    // Window position x,y
    position : (usize, usize),

    // Window size W, H
    size : (usize, usize),

    // Fullscreen flag
    fullscreen : bool,

    // Minimized flag
    minimized : bool,

    // Maximized flag
    maximized : bool,

    // Shown flag
    shown : bool,

    // Hidden flag
    hidden : bool,

    // Borderless flag
    borderless : bool,

    // Resizable flag
    resizable : bool
}
*/