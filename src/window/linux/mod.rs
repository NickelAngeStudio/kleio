// The Linux KWindow will try to open a connection with Wayland first. If it failed, it will open a connection with X11 then.
// NOTE : If Wayland becomes DE FACTO standard, X11 support might be dropped.

use self::{x11::KWindowX11, wayland::KWindowWayland};
use crate::window::KWindowError;
use super::event::KEvent;



/// Wayland KWindow
pub mod wayland;

/// X11 KWindow
pub mod x11;

/// # Manage a window frame in OS GUI.
/// TODO: More doc
pub struct KWindow {



    window : Box<dyn KWindowLinux>,

   
}



impl KWindow {

    /// Create a new [KWindow] using position and size.
    /// 
    /// Return New [`KWindow`].
    /// 
    /// 
    /// # Error(s)
    /// Returns [KWindowError::NoDisplayServer] if no display server found on Linux.
    /// 
    /// # Note(s)
    /// On Linux distribution, this will try to create a [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) window first then
    /// a [x11](https://en.wikipedia.org/wiki/X_Window_System) window if not compatible with Wayland.
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<KWindow, KWindowError> {
        
        let wayland = KWindowWayland::new(pos_x, pos_y, width, height);

        match wayland {
            Some(window) => {
                // Wayland is compatible and used.
                Ok(KWindow { window : Box::new(window) })
            },
            None => {
                // Wayland not compatible. X11 used instead.
                let x11 = KWindowX11::new(pos_x, pos_y, width, height);

                match x11 {
                    Some(window) => {
                        // X11 is compatible and used.
                        Ok(KWindow { window : Box::new(window) })
                    },
                    None => panic!("No windowing system found!"),
                }
            },
        }
    }


    pub fn poll_event(&mut self) -> KEvent {
        self.window.poll_event()

    }

}

/// Linux Kwindow trait used for multiple display server.
trait KWindowLinux {
    fn poll_event(&mut self) -> KEvent;
}


#[cfg(test)]
mod test {
    use super::KWindow;


    #[test]
    #[ignore]
    fn kwindow_test() {
        let window = KWindow::new(100, 100, 200, 200);

        match window {
            Ok(mut window) => {
                loop {
                    window.poll_event();
                }
            },
            Err(_) => panic!("Error while creating window!"),
        }
    }
    /*
    use crate::window::x11::{bind::{XCreateSimpleWindow, XDefaultRootWindow, XMapWindow, XSelectInput, XNextEvent, XOpenDisplay}, constant::{KeyPressMask, ButtonPressMask, ExposureMask, EnterWindowMask, LeaveWindowMask}};

    use std::ptr;

    use super::event::XEvent;

    #[test]
    #[ignore]
    fn waytrest() {
        unsafe {
            let display  = XOpenDisplay(ptr::null());
            if display == ptr::null_mut() {
                panic!("Failed to connect to X11 display!");
            }



            println!("X11={:?}", display);

            let window = XCreateSimpleWindow(display, XDefaultRootWindow(display), 100, 100, 200, 200, 4, 0, 0);

            let event : *mut XEvent  = &mut XEvent { _type: 0};
  
            XMapWindow(display, window);

            XSelectInput(display, window, KeyPressMask | ButtonPressMask | ExposureMask);
            
            loop {
                XNextEvent(display, event);
                println!("ET={:?}", (*event)._type);
            }


        }

    }
    */
}
