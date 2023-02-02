/// Contains X11 contants definition
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_upper_case_globals)]    // Imported C global aren't formatted according to convention.
pub mod constant;

/// Contains X11 C Bind
pub mod bind;

/// Contains X11 KWindow implementation
pub mod window;

use super::KWindow;



impl KWindow {
    /// New new
    pub fn new() -> KWindow {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::window::x11::{bind::{XCreateSimpleWindow, XDefaultRootWindow, XMapWindow, XSelectInput, XNextEvent, XOpenDisplay}, constant::{KeyPressMask, ButtonPressMask, ExposureMask, EnterWindowMask, LeaveWindowMask}};

    use std::ptr;

    use super::bind::XEvent;

    #[test]
    fn waytrest() {
        unsafe {
            let display  = XOpenDisplay(ptr::null());
            if display == ptr::null_mut() {
                panic!("Failed to connect to X11 display!");
            }



            println!("X11={:?}", display);

            let window = XCreateSimpleWindow(display, XDefaultRootWindow(display), 100, 100, 200, 200, 4, 0, 0);

            let event : *mut XEvent  = &mut XEvent { _type:0 };
  

            XMapWindow(display, window);

            XSelectInput(display, window, KeyPressMask | ButtonPressMask | ExposureMask);
            
            loop {
                XNextEvent(display, event);
                println!("ET={:?}", (*event)._type);
            }


        }

    }
}