use std::ops::Index;

use super::KJournalEntry;

/// Implementing this trait permit to listen to journal
/// 
/// Listeners are notified of new journal entry according to severity they listen
/// to according to set_severity().
pub trait KJournalListener {
    /// Notification of new entry with an unmutable reference to it.
    fn notify(&self, entry : &KJournalEntry);

    /// Set the severity the listener will listen to.
    fn set_severity(&mut self, severity:u8);

    /// Get the severity the listener is listening to.
    fn get_severity(&self) -> u8;
}


/// List of listeners listening to the journal.
pub struct KJournalListenerList<'a> {

    listeners : Vec<&'a dyn KJournalListener>

}

impl<'a> KJournalListenerList<'a> {

    /// Create a new instance of KJournalListenerList
    pub fn new() -> KJournalListenerList<'a>{
        // Create listeners vector.
        let listeners : Vec<&'a dyn KJournalListener> = Vec::new();

        // Create List instance
        KJournalListenerList { listeners }
    }

    /// Notify listeners with a new entry. Will notify listeners according to their severity settings.
    /// 
    /// # Argument(s)
    /// * `entry` - KJournalEntry sent with notification.
    pub fn notify(&self, entry : &super::KJournalEntry) {
        for listener in &self.listeners {
            // Verify that listener is listening to this severity.
            if listener.get_severity() & entry.get_severity() > 0 {
                listener.notify(entry);
            }
        }
    }

    /// Add listener to the list.
    /// 
    /// # Argument(s)
    /// * `listener` - KJournalListener implementation to add to the list.
    /// 
    /// # Panic
    /// Will panic if trying to add the same listener twice.
    pub fn add_listener(&mut self, listener : &'a (dyn KJournalListener + 'a)) {
        match self.get_listener_index(listener) {
            Ok(_) => panic!("Cannot add the same KJournalListener twice!"),
            Err(_) => self.listeners.push(listener),
        }
    }

    /// Remove a listener from the list.
    /// 
    /// # Argument(s)
    /// * `listener` - KJournalListener implementation to remove from the list.
    /// 
    /// # Panic
    /// Will panic if trying to remove a listener not in the list.
    pub fn remove_listener(&mut self, listener : &dyn KJournalListener) {
        match self.get_listener_index(listener) {
            Ok(index) => { self.listeners.remove(index); },
            Err(_) => panic!("KJournalListener to remove not found!!"),
        }
    }

    /// Get the count of listeners.
    /// 
    /// # Return
    /// Count of listeners in list.
    pub fn count(&self)->usize {
        self.listeners.len()
    }

    /// Clear the list of listeners.
    pub fn clear(&mut self) {
        self.listeners.clear();
    }

    /// Get the index of a listener from the list.
    /// 
    /// # Argument(s)
    /// * `listener` - Listener to find in list.
    /// 
    /// # Return
    /// Ok(usize) containing index of the list. Err() otherwise.
    fn get_listener_index(&self, listener : &dyn KJournalListener)-> Result<usize, &str> {
        let mut found = false;
        let mut index: usize = 0;

        for i in 0..self.listeners.len() {
            if std::ptr::eq(listener, self.listeners[i]) {
                found = true;
                index = i;
                break;
            }
        }
        
        if found {
            Ok(index)
        }
        else {
            Err("Listener not found!")
        }
    }
}


