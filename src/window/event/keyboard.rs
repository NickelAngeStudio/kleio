/// Enumeration of possible Keyboard events
pub enum KEventKeyboard {

    // Keyboard key down event. Provides unicode of key pressed.
    KeyDown(u16),

    // Keyboard key up event. Provides unicode of key released.
    KeyUp(u16),
}