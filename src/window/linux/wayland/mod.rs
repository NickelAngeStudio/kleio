use std::panic::catch_unwind;

use crate::window::{event::KEvent, KWindowManager, KWindowManagerId, KWindowError};

use self::bind::wl_display_connect;


/// Waylind C function binds
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_upper_case_globals)]    // Imported C global aren't formatted according to convention.
#[allow(non_camel_case_types)]    // Imported C global aren't formatted according to convention.
pub mod bind;


/// # Wayland KWindow backend
pub struct KWindowManagerWayland {
    


}


impl KWindowManager for KWindowManagerWayland {
    fn new(pos_x:isize, pos_y:isize, width:usize, height:usize) -> Result<Self, crate::window::KWindowError> where Self: Sized {
        
        unsafe {
            // Try to call C function with error handling.
            let result = catch_unwind(|| {
                wl_display_connect(std::ptr::null())
            }); 
            match result {
                Ok(display) => {
                    if display == std::ptr::null_mut() {
                        Err(KWindowError::NotSupported)
                    } else {
                        // TODO: Wayland implementation
                        todo!()
                    }

                },
                // C function crashed. Wayland not supported.
                Err(_) => Err(KWindowError::NotSupported),
            }
        }

    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn poll_event(&mut self) -> KEvent {
        todo!()
    }

    fn get_id(&self) -> u8 {
        KWindowManagerId::WAYLAND
    }
}