/// Contains X11 contants definition
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
    use crate::window::x11::constant::xevent_masks;
    use crate::window::x11::bind::{XCreateSimpleWindow, XDefaultRootWindow, XMapWindow, XSelectInput, XNextEvent, XOpenDisplay};

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

            let event : *mut XEvent = ptr::null_mut();

            XMapWindow(display, window);

            XSelectInput(display, window, xevent_masks::KEY_PRESS_MASK | xevent_masks::BUTTON_PRESS_MASK | xevent_masks::EXPOSURE_MASK);
            
            loop {
                XNextEvent(display, event);
                //println!("ET={:?}", (*event).r#type);
            }


        }

    }
}