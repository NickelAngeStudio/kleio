
use std::time::SystemTime;

/// ##### Enumeration of journal entry severities as bytes flags in order of severity.
#[allow(non_snake_case)]
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

/// ##### Journal entry. Contains the severity, the date and time of entry and a description.
/// 
/// # Example(s)
/// ```
/// // Import Journal entry and severity
/// use olympus_kleio::journal::{ KJournalEntry, KJournalEntrySeverity};
/// 
/// // Create an entry with severity and description as String. (Date and time is added automatically upon creation).
/// let mut j = KJournalEntry::new(KJournalEntrySeverity::ERROR, String::from("This is an example of an error entry!"));
/// 
/// // You can also recycle / update entries (ie. for circular buffer). (Date and time is added automatically updated).
/// j.update(KJournalEntrySeverity::INFORMATION, String::from("This is now a recycle entry!"));
/// ```
pub struct KJournalEntry {
    /// Severity of the entry according to [`KJournalEntrySeverity`].
    severity : u8,

    /// Date and time entry occurred.
    date_time : SystemTime,

    /// Entry description / Metadata.
    description : String,
}


impl KJournalEntry {
    /// Create a new [`KJournalEntry`] from severity and description. Date and time will be added automatically.
    /// 
    /// # Argument(s)
    /// * `severity` - [`KJournalEntrySeverity`] of the entry as u8.
    /// * `description` - Description or metadata of entry.
    /// 
    /// # Return
    /// New [`KJournalEntry`] with new date and time.
    pub fn new(severity : u8, description : String) -> KJournalEntry {
        KJournalEntry { severity, date_time: SystemTime::now(), description }
    }

    /// Update Journal entry with a new severity and description. Date and time will be modified automatically.
    /// 
    /// # Argument(s)
    /// * `severity` - [`KJournalEntrySeverity`] of the entry as u8.
    /// * `description` - Description or metadata of entry.
    pub fn update(&mut self, severity : u8, description : String){
        self.date_time = SystemTime::now();
        self.severity = severity;
        self.description = description;
    }

    /// Get entry [`KJournalEntrySeverity`].
    /// 
    /// # Return
    /// Entry [`KJournalEntrySeverity`].
    pub fn get_severity(&self) -> u8{
        self.severity
    }

    /// Get entry date and time as [`SystemTime`].
    /// 
    /// # Return
    /// Entry date and time.
    pub fn get_date_time(&self) -> SystemTime{
        self.date_time
    }

    /// Get entry description / metadata.
    /// 
    /// # Return
    /// Get entry description / metadata.
    pub fn get_description(&self) -> &String {
        &self.description
    }
}
