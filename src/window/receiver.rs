use std::rc::Rc;

use crate::window::{ KWindow};

use super::{KEvent, KWindowError};

/// Implementation of [KWindow] [KEventReceiver] handling.
impl KWindow {   
    /// Add a [KEventReceiver] to the [KWindow] that will receive [KEvent] dispatched.
    /// 
    /// Returns [OK(usize)][Ok] with index of new receiver added.
    /// 
    /// # Error(s)
    /// Returns `Err(`[KWindowError::ReceiverAlreadyExists]`)` if receiver is already in list.
    /// 
    /// # Note(s)
    /// [KEvent] dispatch starts from the most recent added [KEventReceiver] to the older, like a [Stack](https://en.wikipedia.org/wiki/Stack_(abstract_data_type)).
    /// That means that most recent [KEventReceiver] can mask events for older [KEventReceiver].
    pub fn add_event_receiver(&mut self, receiver : Rc<dyn KEventReceiver>) -> Result<usize, KWindowError> {
        
        match self.get_receiver_index(receiver.clone()) {
            Ok(_) => Err(KWindowError::ReceiverAlreadyExists),
            Err(_) => { self.receivers.push(receiver.clone()); Ok(self.receivers.len() - 1) },
        }

    }

    /// Remove a [KEventReceiver] from the [KWindow] list.
    /// 
    /// Returns [OK(usize)][Ok] with index of receiver removed.
    /// 
    /// # Error(s)
    /// Returns `Err(`[KWindowError::ReceiverNotFound]`)` if receiver not found.
    pub fn remove_event_receiver(&mut self, receiver : Rc<dyn KEventReceiver>) -> Result<usize, KWindowError> {
        
        match self.get_receiver_index(receiver) {
            Ok(index) => { self.receivers.remove(index); Ok(index) },
            Err(_) => Err(KWindowError::ReceiverNotFound),
        }

    }

    /// Dispatch [KEvent] to [KEventReceiver].
    /// 
    /// This function should be called at the beginning of each main loop.
    /// 
    /// Returns the count of [KEvent] dispatched.
    /// 
    /// # Example(s)
    /// Dispatching at each loop call in Main loop
    /// ```
    /// // Create a KWindow
    /// let mut w = KWindow::new(100,100,100,100);
    /// 
    /// ... add receivers to KWindow ...
    /// 
    /// loop {
    ///     match w.dispatch_events(){
    ///         Ok(event_count) => println!("{} events dispatched!", event_count),
    ///         Err(_) => println!("No receivers added for dispatch!"),
    ///     }
    /// }
    /// ```
    /// 
    /// # Error(s)
    /// Returns `Err(`[KWindowError::DispatchNoReceiver]`)` if no receiver added to handle events.
    pub fn dispatch_events(&mut self) -> Result<usize, KWindowError> {

        // If no receivers, return error
        if self.receivers.is_empty() {
            Err(KWindowError::DispatchNoReceiver)
        } else {
        
            // First get the event count to poll. This is important to prevent bloking.
            let event_count = self.window_manager.get_event_count();

            // Count of unknown events.
            let mut unknown_count:usize = 0;

            for _ in 0..event_count {
                // Fetch event
                let event = self.window_manager.poll_event();

                match event {
                    // Unknown event are ignored and deduced from event_count
                    KEvent::Unknown => {
                        unknown_count = unknown_count - 1;
                    },
                    _ => {
                        // Iterate enabled receivers from newest to oldest
                        for receiver in self.receivers.iter().rev().filter(|x| x.is_enabled() ) {
                            if receiver.receive(&event) {
                                break;  // Break loop since event was handled
                            }
                        }
                    },
                }

            }

            // Return count of event handled
            Ok(event_count - unknown_count)
        }

    }

    /// Returns the index of a receiver from the list.
    /// 
    /// # Error(s)
    /// Returns `Err(`[KWindowError::ReceiverNotFound]`)` if receiver not found.
    fn get_receiver_index(&self, receiver : Rc<dyn KEventReceiver>)-> Result<usize, KWindowError> {
        let mut found = false;
        let mut index: usize = 0;

        for i in 0..self.receivers.len() {
            if std::ptr::eq(receiver.as_ref(), self.receivers[i].as_ref()) {
                found = true;
                index = i;
                break;
            }
        }
        
        if found {
            Ok(index)
        }
        else {
            Err(KWindowError::ReceiverNotFound)
        }
    }
}


/// # Receive [KEvent] from [KWindow](struct.KWindow.html) and handle them if needed. 
/// TODO: More doc
pub trait KEventReceiver {

    /// # Receive a [KEvent] from the dispatcher.
    /// 
    /// Return True if the [KEvent] has been handled, which will prevent other receiver from handling it.
    /// Return False if the [KEvent] wasn't handled, giving it to the next receiver.
    fn receive(&self, event : &KEvent) -> bool;

    /// Returns if [KEventReceiver] is enabled and ready to receive [KEvent].
    /// 
    /// If False, the [KEventReceiver] will NOT receive [KEvent].
    fn is_enabled(&self) -> bool;
}