/// # Re-export for Public API
#[doc(inline)]
pub use mouse::KEventMouse as KEventMouse;
pub use window::KEventWindow as KEventWindow;
pub use controller::KEventController as KEventController;
pub use keyboard::KEventKeyboard as KEventKeyboard;
pub use queue::KEventQueue as KEventQueue;



// Kleio KEventQueue
#[doc(hidden)]
pub mod queue;

// Kleio window events
#[doc(hidden)]
pub mod window;

// Kleio keyboard events
#[doc(hidden)]
pub mod keyboard;

// Kleio mouse events
#[doc(hidden)]
pub mod mouse;

// Kleio controller events
#[doc(hidden)]
pub mod controller;


/// Union of possible events into an enumeration.
pub enum KEvent {

    /// Window events
    Window(KEventWindow),

    /// Keyboard events
    Keyboard(KEventKeyboard),

    /// Mouse events
    Mouse(KEventMouse),

    /// Controller events
    Controller(KEventController),
}

