use std::{panic::catch_unwind};

use crate::window::{linux::x11::{bind::{XDefaultRootWindow, XCreateSimpleWindow, XMapWindow, XSelectInput, XSync, XEventsQueued}, constant::{KeyPressMask, ButtonPressMask, ExposureMask}}, event::KEvent, self, KWindowManager, KWindowManagerId};

use self::{event::{ Display, Window, XEvent }, bind::{XOpenDisplay, XCloseDisplay, XNextEvent}};

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


/// # X11 KWindowManager backend
pub struct KWindowManagerX11 {
    /// Used to fetch events
    event : XEvent,

    /// X11 Display pointer
    display : *mut Display,

    /// X11 Window pointer
    window : *mut Window,
}

impl KWindowManagerX11 {
    pub fn get_display(&self) -> *mut Display {
        self.display
    }
}

impl KWindowManager for KWindowManagerX11 {
    fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<Self, window::KWindowError> where Self: Sized {
        unsafe {
            // Try to call C function with error handling.
            let result = catch_unwind(|| {
                let display = XOpenDisplay(std::ptr::null());
                println!("Display={:?}", display);

                if display == std::ptr::null_mut() {
                    // Display server connection failed
                    None
                } else {
                    let window = XCreateSimpleWindow(display, XDefaultRootWindow(display), pos_x as i32, pos_y as i32,
                        width as u32, height as u32, 4, 0, 0);

                    XMapWindow(display, window);
                    XSelectInput(display, window, KeyPressMask | ButtonPressMask | ExposureMask);


                    Some(KWindowManagerX11 { event : XEvent { _type: 0}, display : display, window : window })
                }
                

            }); 

            match result {
                Ok(window) => match window {
                    Some(window) => Ok(window),
                    None => Err(window::KWindowError::NotSupported),
                },
                Err(_) => Err(window::KWindowError::NotSupported),
            }
        }
    }


    fn poll_event(&mut self) -> KEvent {
        unsafe {
            XNextEvent(self.display, &mut self.event);


            
            if XEventsQueued(self.display, 0) > 0 {
                

                KEvent::Unknown
            } else {
                // Perform an XSync when no event queued
                

                // Return KEvent::None
                KEvent::Unknown
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_id(&self) -> u8 {
        KWindowManagerId::X11
    }

    fn get_event_count(&self) -> usize {
        unsafe {
            XEventsQueued(self.display, 0).try_into().unwrap()
        }   
    }

    fn sync_event(&self) {
        unsafe {
            // The XSync() function flushes the output buffer and then waits until all requests have been received and processed by the X server.
            XSync(self.display, false);
        }
    }
}


impl Drop for KWindowManagerX11 {
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
