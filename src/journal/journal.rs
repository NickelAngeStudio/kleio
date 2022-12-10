use super::{KJournalEntry, KJournalListener};




/// ##### Journal use for logging events and information.
/// 
/// Act as a circular buffer.
/// 
/// TODO: More doc and examples
pub struct KJournal<'a> {

    /// Name of the journal
    name : String,

    /// Vector of entries for journal, used as a circular buffer.
    entries : Vec<KJournalEntry>,

    /// List of listeners
    listeners : KJournalListenerList<'a>,
    
    /// Maximum entries allowed before overwritting older entries. 
    max_entries : usize,

    /// Head of the circular buffer
    head : usize,

    /// Tail of the circular buffer
    tail : usize,



}


/// List of listeners listening to the journal.
struct KJournalListenerList<'a> {

    listener_entries : Vec<KJournalListenerListEntry<'a>>

}

/// Entry of a listener in the list.
struct KJournalListenerListEntry<'a> {
    /// Listener to notify.
    listener : &'a dyn KJournalListener,
    /// Severity(ies) this listener listen to.
    severity : u8
}