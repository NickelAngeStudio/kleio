use std::{ptr::{null_mut, null}, panic::catch_unwind};

use crate::window::{linux::x11::{bind::{XDefaultRootWindow, XCreateSimpleWindow, XMapWindow, XSelectInput, XSync, XEventsQueued}, constant::{KeyPressMask, ButtonPressMask, ExposureMask}}, event::KEvent, self};

use self::{event::{ Display, Window, XEvent }, bind::{XOpenDisplay, XCloseDisplay, XNextEvent}};

use super::KWindowLinux;

/// Contains X11 contants definition
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_upper_case_globals)]    // Imported C global aren't formatted according to convention.
pub mod constant;

/// Contains X11 Event definition
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_snake_case)]            // Imported C members aren't formatted according to convention.
pub mod event;

/// Contains X11 C functions Bind
pub mod bind;


/// # X11 KWindow backend
pub struct KWindowX11 {
    /// Used to fetch events
    event : XEvent,

    /// X11 Display pointer
    display : *mut Display,

    /// X11 Window pointer
    window : *mut Window,
}

impl KWindowX11 {
    /// Create a new KWindowX11 window. Since it CAN fail, an option is returned if X11 server not available.
    pub fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Option<KWindowX11> {
        unsafe {
            // Try to call C function with error handling.
            let result = catch_unwind(|| {
                let display = XOpenDisplay(std::ptr::null());

                if display == std::ptr::null_mut() {
                    // Display server connection failed
                    None
                } else {
                    let window = XCreateSimpleWindow(display, XDefaultRootWindow(display), pos_x as i32, pos_y as i32,
                        width as u32, height as u32, 4, 0, 0);

                    XMapWindow(display, window);
                    XSelectInput(display, window, KeyPressMask | ButtonPressMask | ExposureMask);


                    Some(KWindowX11 { event : XEvent { _type: 0}, display : display, window : window })
                }
                

            }); 

            match result {
                Ok(window) => window,
                Err(_) => None,
            }
        }
    }

}


impl KWindowLinux for KWindowX11 {
    fn poll_event(&mut self) -> KEvent {
        unsafe {
            if XEventsQueued(self.display, 0) > 0 {
                XNextEvent(self.display, &mut self.event);

                KEvent::None
            } else {
                // Perform an XSync when no event queued
                XSync(self.display, false);

                // Return KEvent::None
                KEvent::None
            }
        }
    }
}

impl Drop for KWindowX11 {
    /// KWindowX11 destructor. Will disconnect display server.
    fn drop(&mut self) {
        unsafe {
            println!("Dropping {:?}", self.display);
            // Close display server connection.
            XCloseDisplay(self.display);
        }
    }
}


#[cfg(test)]
mod test {
   
    use std::ptr;

    use crate::window::linux::x11::{bind::{ XOpenDisplay, XCreateSimpleWindow, XDefaultRootWindow, XMapWindow, XSelectInput, XNextEvent, XSync, XEventsQueued }, constant::{KeyPressMask, ButtonPressMask, ExposureMask}};

    use super::event::XEvent;

    #[test]
    #[ignore]
    fn waytrest() {
        unsafe {
            let display  = XOpenDisplay(ptr::null());
            if display == ptr::null_mut() {
                panic!("Failed to connect to X11 display!");
            }


            let window = XCreateSimpleWindow(display, XDefaultRootWindow(display), 100, 100, 200, 200, 4, 0, 0);
            let event : *mut XEvent  = &mut XEvent { _type: 0};
  
            XMapWindow(display, window);

            XSelectInput(display, window, KeyPressMask | ButtonPressMask | ExposureMask);
            
            loop {
                if XEventsQueued(display, 0) > 0 {
                    XNextEvent(display, event);
                    println!("ET={:?}", (*event)._type);
                }
                XSync(display, false);
                
                
            }


        }

    }
}
