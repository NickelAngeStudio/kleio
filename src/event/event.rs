use super::{KEventWindow, KEventKeyboard, KEventMouse, KEventController};

/// Union of possible events into an enumeration.
pub enum KEvent<'a> {

    /// Window events
    Window(&'a KEventWindow<'a>),

    /// Keyboard events
    Keyboard(&'a KEventKeyboard),

    /// Mouse events
    Mouse(&'a KEventMouse),

    /// Controller events
    Controller(&'a KEventController),
}