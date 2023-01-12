use super::{KJournalEntry, KJournalListener, listener::KJournalListenerList, KJournalEntrySeverity};

/// ##### Journal use for logging events and information.
/// 
/// 
/// 
/// TODO: More doc and examples. Min and Max size
pub struct KJournal<'a> {

    /// Name of the journal
    name : String,

    /// Severities to log. KJournalEntrySeverity::ALL_WITH_DEBUG by default.
    severity : u8,

    /// List of listeners
    listeners : KJournalListenerList<'a>,

    /// Journal entries kept in circular buffer
    entries :  KJournalBuffer,
}

impl<'a> KJournal<'a> {

    /// Create a new instance of KJournal.
    /// 
    /// # Argument(s)
    /// * `name` - Name / Id to identify the journal.
    /// * `severity` - Severities of entry to log. Will ignore entries not included.
    /// * `max_entries` - Maximum count of entries kept before rewriting oldest entries
    /// 
    /// # Return
    /// New journal created.
    pub fn new(name : &str, severity : u8, max_entries : usize) -> KJournal {
        KJournal { name: name.to_owned(), severity: severity, listeners: KJournalListenerList::new(), entries: KJournalBuffer::new(max_entries)  }
    }

    /// Get the journal latest entry.
    /// 
    /// # Return
    /// [Some(KJournalEntry)](https://doc.rust-lang.org/beta/core/option/enum.Option.html#variant.Some) if any, [None] otherwise.
    pub fn get_latest_entry(&self) -> Option<&KJournalEntry> {
        todo!()
    }

    /// Set severity flags to log in journal. See [`KJournalEntrySeverity`].
    pub fn set_severity(severity : u8) {
        todo!()
    } 

    /// Set the maximum entries kept.
    /// 
    /// # Argument(s)
    /// * `max_entries` - Maximum count of entries kept before rewriting oldest entries
    /// 
    /// # Note(s)
    /// PREVIOUS ENTRIES WILL BE DESTROYED!
    pub fn set_max_entries(&mut self, max_entries : usize) {
        todo!()

        // Recreate buffer

        // Store new maximum entries
    }

    

}


/// Minimum possible buffer size for journal buffer.
pub const KJOURNAL_BUFFER_MIN: usize = 10;

/// Maximum possible buffer size for journal buffer.
pub const KJOURNAL_BUFFER_MAX:usize = 65535;


/// ##### Journal circular buffer containing entries.
pub struct KJournalBuffer {
    /// Vector of entries for journal, used as a circular buffer.
    entries : Vec<KJournalEntry>,

    /// Size of the circular buffer
    size : usize,

    /// Head of the circular buffer
    head : usize,

    /// Tail of the circular buffer
    tail : usize,

    
}

impl KJournalBuffer {

    /// Create a new journal circular buffer according to size parameter.
    /// # Argument(s)
    /// * `size` - Size of the circular buffer
    /// 
    /// # Return
    /// New journal circular buffer created.
    pub fn new(size : usize) -> KJournalBuffer {

        // Make sure size is between MIN and MAX
        assert!(size >= KJOURNAL_BUFFER_MIN && size <= KJOURNAL_BUFFER_MAX, "Journal size {} should be between {} and {}!", size, KJOURNAL_BUFFER_MIN, KJOURNAL_BUFFER_MAX);

        // Size is padded for head == tail conundrum 
        let padded_size = size + 1;

        // Create entries vector and reserve size.
        let mut entries : Vec<KJournalEntry> = Vec::new();
        entries.reserve(padded_size);

        // Create all entries.
        for _ in 0..padded_size {
            entries.push( KJournalEntry::new(KJournalEntrySeverity::OTHER, "".to_string()));
        }

        // Return KJournalBuffer. size is padded for head == tail conundrum
        KJournalBuffer {
            entries, size : padded_size, head:0, tail:0
        }
    }

    /// Get the count of unread entries.
    /// 
    /// # Return
    /// Count of unread entries.
    pub fn unread(&self) -> usize {
        if self.head < self.tail {
            return self.head + self.size - self.tail;
        } else {
            return self.head - self.tail;
        }
    }

    /// Get the size of the buffer
    /// 
    /// # Return
    /// Size of the buffer.
    pub fn size(&self) -> usize {
        self.size -1
    }

    /// Clear the buffer.
    pub fn clear(&mut self){
        self.tail = self.head;
    }

    /// Write a new entry to the buffer.
    /// 
    /// * `severity` - Severity of the new entry.
    /// * `description` - Description / Metadata of the new entry,
    pub fn write(&mut self, severity : u8, description : String) {

        // Increment head.
        self.inc_head();

        // Write entry into buffer
        self.entries[self.head].update(severity, description);
    }

    /// Get the latest [`KJournalEntry`] in the buffer.
    /// 
    /// # Return
    /// [Some(KJournalEntry)](https://doc.rust-lang.org/beta/core/option/enum.Option.html#variant.Some) if any, [None] otherwise.
    pub fn latest(&mut self) -> Option<&KJournalEntry> {
        
        if self.head == self.tail {
            // If head == tail, we have no entries.
            None
        } else {
            // Get current entry position AKA head.
            let current = self.head;

            // Decrease head.
            self.dec_head();

            // Return reference to current entry.
            Some(&self.entries[current])
        }
    }

    /// Increment head. If head >= size, head will go back to 0. If head == tail, will push tail.
    fn inc_head(&mut self) {

        self.head += 1;

        if self.head >= self.size {
            self.head = 0;
        }

        if self.head == self.tail {
            self.push_tail();
        }
    }

    /// Decrement head. (Usually when reading an entry). If head == 0, head will go to the end of buffer.
    fn dec_head(&mut self) {

        if self.head >= 1 {
            self.head -= 1;
        } else {
            self.head = self.size -1;
        }

    }

    /// Push the tail. Happens when the head catches the tail.
    fn push_tail(&mut self){
        self.tail += 1;

        if self.tail >= self.size {
            self.tail = 0;
        }
    }


}


