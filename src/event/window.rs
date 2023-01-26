use crate::window::KWindow;

/// Enumeration of possible events for a window
pub enum KEventWindow<'a> {

    /// Happens when KWindow is shown.
    Shown(&'a KWindow),

    /// Happens when KWindow is hidden.
    Hidden(&'a KWindow),

    /// Happens when KWindow is exposed.
    Exposed(&'a KWindow),

    /// Happens when KWindow is moved. Provide pair of usize of new position.
    Moved(&'a KWindow, (usize, usize)),

    /// Happens when KWindow is Resized. Provide pair of usize of new size.
    Resized(&'a KWindow, (usize, usize)),

    /// Happens when KWindow size changed without user input. Provide pair of usize of new size.
    SizeChanged(&'a KWindow, (usize, usize)),

    /// Happens when KWindow is minimized.
    Minimized(&'a KWindow),

    /// Happens when KWindow is maximized.
    Maximized(&'a KWindow),

    /// Happens when KWindow is restored.
    Restored(&'a KWindow),

    /// Happens when mouse enter KWindow.
    MouseEnter(&'a KWindow),

    /// Happens when mouse leave KWindow.
    MouseLeave(&'a KWindow),

    /// Happens when KWindow gain focus.
    Focus(&'a KWindow),

    /// Happens when KWindow lose focus.
    Blur(&'a KWindow),

    /// Happens when KWindow closes.
    Close(&'a KWindow),
}
