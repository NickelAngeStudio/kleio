use super::KJournalEntry;

/// Implementing this trait permit to listen to journal
/// 
pub trait KJournalListener {
    /// Notification of new entry with an unmutable reference to it.
    fn notify(&self, new_entry : &KJournalEntry);
}