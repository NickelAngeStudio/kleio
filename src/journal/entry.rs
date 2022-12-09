
use std::time::SystemTime;

/// ##### Enumeration of journal entry severities as bytes flags in order of severity.
pub mod KJournalEntrySeverity {

    /// Lowest severity. Debug message. Listeners shouldn't listener to those by default.
    pub const DEBUG: u8 = 1;

    /// Other message with low severity impact.
    pub const OTHER: u8 = 2;

    /// Information about event as they occur.
    pub const INFORMATION: u8 = 4;

    /// An occurred event that can be potentially severe.
    pub const WARNING: u8 = 8;

    /// A severe error that occurred but didn't cause the program to crash.
    pub const ERROR: u8 = 16;

    /// Highest severity. Causes program to crash.
    pub const FATAL: u8 = 32;
    
    /// Quick shortcut to all severity flags without DEBUG.
    pub const ALL_NO_DEBUG: u8 = 62;

    /// Quick shortcut to all severity flags including DEBUG.
    pub const ALL_WITH_DEBUG: u8 = 63;

   

}

pub struct KJournalEntry {
    
    severity : u8,

    date_time : SystemTime,

    description : String,
    
}