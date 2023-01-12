use super::KJournalEntry;

/// Implementing this trait permit to listen to journal
/// 
pub trait KJournalListener {
    /// Notification of new entry with an unmutable reference to it.
    fn notify(&self, new_entry : &KJournalEntry);
}


/// List of listeners listening to the journal.
pub struct KJournalListenerList<'a> {

    listeners : Vec<KJournalListenerListEntry<'a>>

}

impl<'a> KJournalListenerList<'a> {

    /// Create a new instance of KJournalListenerList
    pub fn new() -> KJournalListenerList<'a>{
        // Create listeners vector.
        let mut listeners : Vec<KJournalListenerListEntry> = Vec::new();

        // Create List instance
        KJournalListenerList { listeners }
    }
}

/// Entry of a listener in the list.
pub struct KJournalListenerListEntry<'a> {
    /// Listener to notify.
    listener : &'a dyn KJournalListener,
    /// Severity(ies) this listener listen to.
    severity : u8
}