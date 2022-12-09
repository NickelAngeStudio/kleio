/*
 * @file kleio/window/mod.rs
 *
 * @module olympus::kleio::window
 *
 * @brief Contains KWindow trait, KWindowProperty struct, KWindowSupplier trait and KWindowRenderer trait
 * 
 * @details
 * Contains KWindow trait, KWindowProperty struct, KWindowSupplier trait and KWindowRenderer trait
 * 
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-10-03
 *
 * @version
 * 1.0 : 2022-10-03 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

/// TODO : Doc
pub trait KWindow {
    
}

/// TODO : Doc
pub trait KWindowSupplier {
    
}

/// TODO : Doc
pub trait KWindowRenderer {
    
}

/// TODO : Doc
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