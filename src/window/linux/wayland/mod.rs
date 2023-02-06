use std::panic::catch_unwind;

use crate::window::event::KEvent;

use self::bind::wl_display_connect;

use super::KWindowLinux;

/// Waylind C function binds
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_upper_case_globals)]    // Imported C global aren't formatted according to convention.
#[allow(non_camel_case_types)]    // Imported C global aren't formatted according to convention.
pub mod bind;


/// # Wayland KWindow backend
pub struct KWindowWayland {
    


}


impl KWindowWayland {
    pub fn new(posX:isize, posY:isize, width:usize, height:usize) -> Option<KWindowWayland> {

        unsafe {
            // Try to call C function with error handling.
            let result = catch_unwind(|| {
                wl_display_connect(std::ptr::null())
            }); 
            match result {
                Ok(display) => {
                    if display == std::ptr::null_mut() {
                        None
                    } else {
                        // TODO: Wayland implementation
                        todo!()
                    }

                },
                // C function crashed. Wayland not compatible.
                Err(_) => None,
            }
        }
    }
    

}

impl KWindowLinux for KWindowWayland {
    fn poll_event(&mut self) -> KEvent {
        todo!()
    }
}
