use crate::window::{ KWindow};

use super::KEvent;


impl KWindow {
    pub fn add_event_receiver(&self) {
        todo!()
    }

    pub fn remove_event_receiver() {
        todo!()
    }

    /// Dispatch [KEvent] to [KEventReceiver].
    /// 
    /// This function should be called at the beginning of each main loop. 
    /// TODO:Example
    pub fn dispatch_events(&mut self) -> KEvent {
        todo!()
    }
}


/// # Receive [KEvent] from [KWindow](struct.KWindow.html) and handle them if needed. 
/// TODO: More doc
pub trait KEventReceiver {

    /// # Receive a [KEvent] from the dispatcher.
    /// 
    /// Return True if the [KEvent] has been handled, which will prevent other receiver from handling it.
    /// Return False if the [KEvent] wasn't handled, giving it to the next receiver.
    fn receive(&self, event : KEvent) -> bool;
}

/// [KEventDispatcher] unit tests.
#[cfg(test)]
mod unit_tests {
    #[test]
    #[ignore]
    fn kwindow_test() {
    }
}
