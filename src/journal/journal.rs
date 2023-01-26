use super::{KJournalEntry, listener::KJournalListenerList, listener::{KJournalListener, KJournalListenerListError}, KJournalEntrySeverity};

/// ##### Journal use for logging events and information.
/// 
/// # Example
/// Create a new [KJournal] with minimum buffer size.
/// ```
/// use olympus_kleio::journal::{KJournal, KJOURNAL_BUFFER_MIN};
/// 
/// 
/// 
/// ```
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

/// Enumeration of possible [KJournal] errors.
pub enum KJournalError {

    /// Happens when [KJournal] buffer size is smaller then [KJOURNAL_BUFFER_MIN].
    BufferSizeTooSmall,

    /// Happens when [KJournal] buffer size is bigger then [KJOURNAL_BUFFER_MAX].
    BufferSizeTooBig,

}

impl std::fmt::Debug for KJournalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BufferSizeTooSmall => write!(f, "BufferSizeTooSmall"),
            Self::BufferSizeTooBig => write!(f, "BufferSizeTooBig"),
        }
    }
}

impl<'a> KJournal<'a> {

    /// Create a new instance of KJournal from name, severity it listen to and maximum entries allowed(buffer size).
    /// 
    /// Returns Ok([KJournal]) with new journal created if successful.
    /// 
    /// # Error(s)
    /// Returns Err([KJournalError::BufferSizeTooSmall]) if `max_entries` < [KJOURNAL_BUFFER_MIN].
    /// 
    /// Returns Err([KJournalError::BufferSizeTooBig]) if `max_entries` > [KJOURNAL_BUFFER_MAX].
    pub fn new(name : &str, severity : u8, max_entries : usize) -> Result<KJournal, KJournalError> {

        match  KJournalBuffer::new(max_entries) {
            Ok(buffer) => Ok( KJournal {
                name: name.to_owned(), 
                severity: severity, 
                listeners: KJournalListenerList::new(), 
                entries: buffer  }),
            Err(error) => Err(error),
        }

    }

    /// Write a new entry to [KJournal] with [`KJournalEntrySeverity`] and description if entry is not ignored.
    pub fn write(&mut self, severity : u8, description : &str) {
        todo!()
    }

    /// Pop the journal latest entry.
    /// 
    /// Returns [Some(KJournalEntry)](https://doc.rust-lang.org/beta/core/option/enum.Option.html#variant.Some) if any or [None] otherwise.
    pub fn read(&self) -> Option<&KJournalEntry> {
        todo!()
    }

    /// Get count of unread [KJournalEntry] as usize.
    pub fn unread(&self) -> usize {
        todo!()
    }

    /// Clear the [KJournal] to 0 entries.
    pub fn clear(&mut self) {
        todo!()
    }

    /// Add [KJournalListener] to the [KJournal].
    /// 
    /// Returns [OK(usize)][Ok] with index of new listener added.
    /// 
    /// # Error(s)
    /// Returns `Err(`[KJournalListenerListError::ListenerAlreadyExists]`)` if listener is already in list.
    pub fn add_listener(&mut self, listener : &'a (dyn KJournalListener + 'a)) -> Result<usize, KJournalListenerListError> {
        
        todo!()

    }

    /// Remove a [KJournalListener] from the [KJournal].
    /// 
    /// Returns [OK(usize)][Ok] with index of listener removed.
    /// 
    /// # Error(s)
    /// Returns `Err(`[KJournalListenerListError::ListenerNotFound]`)` if listener not found.
    pub fn remove_listener(&mut self, listener : &dyn KJournalListener) -> Result<usize, KJournalListenerListError> {
        todo!()
    }


    /// Set [`KJournalEntrySeverity`] flags to log in journal. Will ignore other severity and won't push them to listeners.
    pub fn set_severity(&mut self, severity : u8) {
        todo!()
    } 

    /// Get [`KJournalEntrySeverity`] the [KJournal] listen to.
    pub fn get_severity(&self) -> u8 {
        todo!()
    }

    /// Set the maximum entries kept in [KJournal].
    /// 
    /// Returns Ok([usize]) with the new size of the buffer.
    /// 
    /// # Note(s)
    /// PREVIOUS ENTRIES WILL BE LOST!
    /// 
    /// # Error(s)
    /// Returns Err([KJournalError::BufferSizeTooSmall]) if `max_entries` < [KJOURNAL_BUFFER_MIN].
    /// 
    /// Returns Err([KJournalError::BufferSizeTooBig]) if `max_entries` > [KJOURNAL_BUFFER_MAX].
    pub fn set_max_entries(&mut self, max_entries : usize) -> Result<usize, KJournalError> {
        todo!()

        // Recreate buffer

        // Store new maximum entries
    }

    /// Get the buffer size of [KJournal].
    pub fn get_max_entries(&self) -> usize {
        todo!()
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
    /// 
    /// Returns Ok([KJournalBuffer]) with new journal circular buffer created.
    /// 
    /// # Error(s)
    /// Returns Err([KJournalError::BufferSizeTooSmall]) if `size` < [KJOURNAL_BUFFER_MIN].
    /// 
    /// Returns Err([KJournalError::BufferSizeTooBig]) if `size` > [KJOURNAL_BUFFER_MAX].
    pub fn new(size : usize) -> Result<KJournalBuffer, KJournalError> {

        // Make sure size is >=  KJOURNAL_BUFFER_MIN
        if size < KJOURNAL_BUFFER_MIN {
            return Err(KJournalError::BufferSizeTooSmall)
        }

        // Make sure size is <= KJOURNAL_BUFFER_MAX
        if size > KJOURNAL_BUFFER_MAX {
            return Err(KJournalError::BufferSizeTooBig)
        }

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
        Ok(KJournalBuffer {
            entries, size : padded_size, head:0, tail:0
        })
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


