use super::{KEventWindow, KEventKeyboard, KEventMouse, KEventController};

// Kleio KEventDispatcher and Receiver
#[doc(hidden)]
pub mod dispatcher;

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

    /// No Event
    None,

    /// Window events
    Window(KEventWindow),

    /// Keyboard events
    Keyboard(KEventKeyboard),

    /// Mouse events
    Mouse(KEventMouse),

    /// Controller events
    Controller(KEventController),
}