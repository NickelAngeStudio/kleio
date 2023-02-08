use super::{ KWindow, KWindowError };

/// Minimum [KWindow] width allowed.
pub const KWINDOW_MIN_WIDTH : usize = 1;

/// Minimum [KWindow] height allowed.
pub const KWINDOW_MIN_HEIGHT : usize = 1;

/// Maximum [KWindow] width allowed.
pub const KWINDOW_MAX_WIDTH : usize = 65535;

/// Maximum [KWindow] width allowed.
pub const KWINDOW_MAX_HEIGHT : usize = 65535;


/// Implementation of [KWindow] properties.
impl KWindow {   

    /// Set a new title for the [KWindow].
    pub fn set_title(&self, title : &str) {
        todo!()
    }

    /// Returns the [KWindow] title. 
    pub fn title(&self) -> &str {
        todo!()
    }

    /// Set position of [KWindow] according to position (x,y).
    pub fn set_position(&self, position : (isize, isize)){
        todo!()
    }

    /// Returns position (x,y) of the [KWindow].
    pub fn position(&self) -> (isize, isize) {
        todo!()
    }

    /// Set dimension of [KWindow] according to size (width, height).
    /// 
    /// Returns Ok(0) if successful.
    /// 
    /// # Error(s)
    /// Returns [KWindowError::WindowSizeError] if width and/or height not within allowed boundaries.
    pub fn set_size(&self, dimension : (usize, usize)) -> Result<u8, KWindowError>{
        todo!()
    }

    /// Returns dimension (width, height) of the [KWindow].
    pub fn size(&self) -> (usize, usize) {
        todo!()
    }

    /// Set the [KWindow] as fullscreen according to parameters.
    pub fn set_fullscreen(&self, fullscreen : bool) {
        todo!()
    }

    /// Returns if the [KWindow] is fullscreen or not.
    pub fn fullscreen(&self) -> bool {
        todo!()
    }
    /*
  
    // Minimized flag
    minimized : bool,

    // Maximized flag
    maximized : bool,

    // Shown flag
    shown : bool,

    // Hidden flag
    hidden : bool,

    cursor
    */

}

/*
pub struct KWindowProperty {

    // Windows handle id
    handle : usize,

    // Window title
    title : String,

    // Window position x,y
    position : (usize, usize),

    // Window size W, H
    size : (usize, usize),

    // Fullscreen flag
    fullscreen : bool,

    // Minimized flag
    minimized : bool,

    // Maximized flag
    maximized : bool,

    // Shown flag
    shown : bool,

    // Hidden flag
    hidden : bool,

    // Borderless flag
    borderless : bool,

    // Resizable flag
    resizable : bool
}
*/