use super::KJournalEntry;

/// Implementing this trait permit to listen to journal
/// 
/// Listeners are notified of new journal entry according to severity they listen
/// to according to set_severity().
pub trait KJournalListener {
    /// Notification of new entry with an unmutable reference to it.
    fn notify(&mut self, new_entry : &KJournalEntry);

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
        let mut listeners : Vec<&'a dyn KJournalListener> = Vec::new();

        // Create List instance
        KJournalListenerList { listeners }
    }

    pub fn notify(&self, new_entry : &super::KJournalEntry) {
        todo!()
    }

    pub fn add_listener(&mut self, listener : &dyn KJournalListener) {
        todo!()
    }

    pub fn remove_listener(&mut self, listener : &dyn KJournalListener) {
        todo!()
    }

    pub fn count(&self)->usize {
        todo!()
    }

    pub fn clear(&self) {
        todo!()
    }
}


