/// Enumeration of possible mouse events
pub enum KEventMouse {

    // Mouse move event. Provides new (x, y) position and old (x, y) position.
    Moved((i32, i32), (i32, i32)),

    // Mouse button down event. Provides button number (up to 255) and cursor position (x,y).
    ButtonDown(u8, (i32, i32)),

    // Mouse button up event. Provides button number (up to 255) and cursor position (x,y).
    ButtonUp(u8, (i32, i32)),

    // Mouse wheel event. Provide amount scrolled horizontally and vertically.
    Wheel(i32, i32),

}