use super::{KJournalListener, get_severity_symbol};

/// Implementation of [KJournalListener] that print new entry into console.
pub struct KJournalListenerPrint {
    /// Severity to be printed.
    severity : u8,

}

impl KJournalListenerPrint {
    /// Create a new instance of KJournalListenerPrint.
    /// 
    /// # Argument(s)
    /// * `severity` - Severities of entry to print. Will ignore other entries.
    /// 
    /// # Return
    /// New KJournalListenerPrint created.
    pub fn new(severity : u8) -> KJournalListenerPrint {
        KJournalListenerPrint { severity }
    }
}

impl KJournalListener for KJournalListenerPrint {
    fn notify(&mut self, new_entry : &super::KJournalEntry) {
        println!("[{} {:?}] {}", get_severity_symbol(new_entry.get_severity()), new_entry.get_date_time(), new_entry.get_description())
    }

    fn set_severity(&mut self, severity:u8) {
        self.severity = severity;
    }

    fn get_severity(&self) -> u8 {
       self.severity
    }
}