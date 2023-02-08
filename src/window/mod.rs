use std::any::Any;
use std::rc::Rc;

/// # Re-export for Public API
#[doc(inline)]
pub use renderer::KWindowRenderer as KWindowRenderer;
pub use manager::KWindowManager as KWindowManager;
pub use manager::KWindowManagerId as KWindowManagerId;
pub use event::KEvent as KEvent;
pub use event::mouse::KEventMouse as KEventMouse;
pub use event::window::KEventWindow as KEventWindow;
pub use event::controller::KEventController as KEventController;
pub use event::keyboard::KEventKeyboard as KEventKeyboard;
pub use receiver::KEventReceiver as KEventReceiver;

/// [KWindow] event definition.
#[doc(hidden)]
pub mod event;

/// [KWindowManager] definition.
#[doc(hidden)]
pub mod manager;

/// [KWindow] renderer abstraction.
#[doc(hidden)]
pub mod renderer;

/// [KWindow] receiver abstraction and dispatcher implementation.
#[doc(hidden)]
pub mod receiver;

/// [KWindow] properties implementation.
#[doc(hidden)]
pub mod property;

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

    /// Happens when trying to add the same [KEventReceiver] twice to a [KWindow].
    ReceiverAlreadyExists,

    /// Happens when trying to remove a  [KEventReceiver] not added to a [KWindow].
    ReceiverNotFound,

    /// Happens when trying to dispatch events when no [KEventReceiver] were added.
    DispatchNoReceiver,


}

impl std::fmt::Debug for KWindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "KWindowError[{:?}]", self),
        }
    }
}


/// # Create and manage a window frame in OS GUI.
/// 
/// [KWindow] broadcasts [KEvent] to multiple [KEventReceiver] via [KWindow::dispatch_events()].
/// 
/// TODO: More doc about OS, dispatch, from and window manager and Examples
pub struct KWindow {
    /// List of receivers.
    receivers : Vec<Rc<dyn KEventReceiver>>,

    /// Window manager that manage this [KWindow].
    window_manager : Box<dyn KWindowManager>,
}

/// Implementation of [KWindow] [KEventReceiver] handling.
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
                Ok(KWindow{ receivers : Vec::new(), window_manager : Box::new(wm)})
            },
            // Try to create X11 Window Manager
            Err(_) => match linux::x11::KWindowManagerX11::new(pos_x, pos_y, width, height) {
                Ok(wm) => {
                    Ok(KWindow{ receivers : Vec::new(), window_manager: Box::new(wm) })
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

    /// Create a [KWindow] from a [KWindowManager]. 
    /// 
    /// The [KWindow] will take ownership of the [KWindowManager].
    /// 
    /// Used when porting [KWindow] to another platform.
    pub fn from(window_manager : Box<dyn KWindowManager>) -> KWindow {
        KWindow { receivers : Vec::new(),  window_manager }
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
    /// 
    /// # Example(s)
    /// Downcasting to KWindowManagerX11 : 
    /// ```no_run
    /// match window.downcast_window_manager::<KWindowManagerX11>() {
    ///     Some(wmx11 : &KWindowManagerX11) => todo!(),
    ///     None => todo!(),
    /// }
    /// ```
    pub fn downcast_window_manager<T: Any>(&self) -> Option<&T> {
        self.window_manager.as_any().downcast_ref::<T>()
    }
}

