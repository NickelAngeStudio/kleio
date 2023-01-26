/// # Re-export for Public API
#[doc(inline)]
pub use mouse::KEventMouse as KEventMouse;
pub use window::KEventWindow as KEventWindow;
pub use controller::KEventController as KEventController;
pub use keyboard::KEventKeyboard as KEventKeyboard;
pub use event::KEvent as KEvent;
pub use register::KEventRegister as KEventRegister;


// Kleio KEvent
#[doc(hidden)]
pub mod event;

// Kleio KEventRegister
#[doc(hidden)]
pub mod register;

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

