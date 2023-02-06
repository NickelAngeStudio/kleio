use std::any::Any;

/// # Re-export for Public API
#[doc(inline)]
pub use event::KEvent as KEvent;
pub use event::mouse::KEventMouse as KEventMouse;
pub use event::window::KEventWindow as KEventWindow;
pub use event::controller::KEventController as KEventController;
pub use event::keyboard::KEventKeyboard as KEventKeyboard;
pub use event::dispatcher::KEventDispatcher as KEventDispatcher;
pub use event::dispatcher::KEventReceiver as KEventReceiver;

/// [KWindow] event definition.
#[doc(hidden)]
pub mod event;

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


/// # Manage a window frame in OS GUI.
/// TODO: More doc
pub struct KWindow {
    
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
                Ok(KWindow{ window_manager: Box::new(wm) })
            },
            // Try to create X11 Window Manager
            Err(_) => match linux::x11::KWindowManagerX11::new(pos_x, pos_y, width, height) {
                Ok(wm) => {
                    Ok(KWindow{ window_manager: Box::new(wm) })
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

    pub fn get_window_manager(&self) -> &Box<dyn KWindowManager> {
        &self.window_manager
    }

    /// Create a [KWindow] from a [KWindowManager]. 
    /// 
    /// The [KWindow] will take ownership of the [KWindowManager].
    pub fn from(window_manager : Box<dyn KWindowManager>) -> KWindow {
        KWindow { window_manager }
    }

    /// Pop a [KEvent] from event queue.
    /// TODO:Example
    pub fn poll_event(&mut self) -> KEvent {
        self.window_manager.poll_event()
    }




}

/// Enumeration of possible [KWindowManager] provider.
#[derive(PartialEq)]
pub enum KWindowManagerProvider {
    /// Linux Wayland display server.
    Wayland,

    /// Linux X11 display server.
    X11,

    /// Windows graphical user interface.
    Shell,

    /// Android graphical user interface.
    Android,

    /// MacOS graphical user interface
    MacOS,

    /// IOS graphical user interface
    Ios,

    /// Other / Custom user interface
    Other,
}

/// # Abstraction of window manager that supplies window frame.
pub trait KWindowManager {
    /// Create a new instance of [KWindowManager] and open the window frame.
    /// 
    /// Returns [KWindowManager] created.
    fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<Self, KWindowError> where Self: Sized;

    /// Get an event from window manager as [KEvent].
    fn poll_event(&mut self) -> KEvent;

    /// Get the [KWindowManagerProvider] id that manage the window.
    fn get_provider(&self) -> KWindowManagerProvider;

    /// Get self as [Any], allowing downcasting.
    /// Implementation code should be as follow :
    /// ```
    /// fn as_any(&self) -> &dyn std::any::Any {
    /// self
    /// }
    /// ```
    fn as_any(&self) -> &dyn Any;

    //fn as_any(&self) -> &dyn std::any::Any {
    //    self
    //}


}



#[cfg(test)]
mod test {
    
    use super::{KWindow, linux::x11::KWindowManagerX11, KWindowManagerProvider};


    #[test]
    #[ignore]
    fn kwindow_test() {
        let window = KWindow::new(100, 100, 200, 200);

        match window {
            Ok(mut window) => {
                if window.get_window_manager().get_provider() == KWindowManagerProvider::X11 {
                    let w  = window.get_window_manager().as_ref().as_any();
                    match w.downcast_ref::<KWindowManagerX11>() {
                        Some(dc) => {
                            println!("DC Worked! Proof, display={:?}", dc.get_display());
                        },
                        None => panic!("DC Failed!"),
                    }
                    //window as KWindowManagerX11;
                }
                loop {
                    window.poll_event();
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