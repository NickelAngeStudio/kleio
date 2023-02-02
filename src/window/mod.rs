/// [KWindow] event queue module.
pub mod event;

/// X11 implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "linux", not(display = "wayland")))]
#[doc(hidden)]
pub mod x11;

/// Wayland implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "linux", display = "wayland"))]
#[doc(hidden)]
pub mod wayland;

/// Windows shell implementations of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "windows"))]
#[doc(hidden)]
pub mod shell;

/// Android implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "android"))]
#[doc(hidden)]
pub mod android;

/// IOS implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "ios"))]
#[doc(hidden)]
pub mod ios;

/// MacOS implementation of KWindow
#[cfg(all(not(target_family = "wasm"), target_os = "macos"))]
#[doc(hidden)]
pub mod macos;

/// Web assembly implementation of KWindow
#[cfg(target_family = "wasm")]
#[doc(hidden)]
pub mod wasm;

/// Structure that manage a display frame.
/// 
/// # Implementation(s)
/// Any [impl](https://doc.rust-lang.org/std/keyword.impl.html) or port of KWindow MUST provide those methods :
/// 
/// TODO : Write default methods
pub struct KWindow {

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