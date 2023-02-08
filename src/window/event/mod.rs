use super::{KEventWindow, KEventKeyboard, KEventMouse, KEventController};

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

    /// Unknown Event
    Unknown,

    /// Window events
    Window(KEventWindow),

    /// Keyboard events
    Keyboard(KEventKeyboard),

    /// Mouse events
    Mouse(KEventMouse),

    /// Controller events
    Controller(KEventController),
}