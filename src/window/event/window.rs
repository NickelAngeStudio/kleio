/// Enumeration of possible events for a window
pub enum KEventWindow {

    /// Happens when KWindow is shown.
    Shown(),

    /// Happens when KWindow is hidden.
    Hidden(),

    /// Happens when KWindow is exposed.
    Exposed(),

    /// Happens when KWindow is moved. Provides x and y as new position.
    Moved(usize, usize),

    /// Happens when KWindow is Resized. Provides height and width of new size.
    Resized(usize, usize),

    /// Happens when KWindow size changed without user input. Provides height and width of new size.
    SizeChanged(usize, usize),

    /// Happens when KWindow is minimized.
    Minimized(),

    /// Happens when KWindow is maximized.
    Maximized(),

    /// Happens when KWindow is restored.
    Restored(),

    /// Happens when mouse enter KWindow.
    MouseEnter(),

    /// Happens when mouse leave KWindow.
    MouseLeave(),

    /// Happens when KWindow gain focus.
    Focus(),

    /// Happens when KWindow lose focus.
    Blur(),

    /// Happens when KWindow closes.
    Close(),
}
