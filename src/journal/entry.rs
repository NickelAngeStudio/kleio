
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

/********
* TESTS *
********/
/// Contains unit tests for KJournalEntry
#[cfg(test)]
mod tests {
    use std::{time::{SystemTime, Duration}, thread::sleep};
    use super::{KJournalEntry, KJournalEntrySeverity};


    /// KJournalEntry::new()
    /// Create a new KJournal entry
    #[test]
    fn kjournal_entry_new() {
        // Initial variables for comparison.
        let severity = KJournalEntrySeverity::INFORMATION;
        let description = String::from("Entry kjournal_entry_new");

        // Create new entry.
        let j = KJournalEntry::new(severity, description.clone());

        // Verify each parameter value.
        assert!(j.get_severity() == severity, "Error! Created entry severity is different!");
        assert!(j.get_description().eq(&description), "Error! Created entry description is different!");

        // Make sure that system date and time taken is valid.
        match j.get_date_time().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => 
            assert!(n.as_secs() > 0, "Error! Date and time not recorded!"),
            Err(_) => panic!("Error! SystemTime before UNIX EPOCH!"),
        }

    }
    
    /// KJournalEntry::update()
    #[test]
    fn kjournal_entry_update() {
        // Initial variables for comparison.
        let severity = KJournalEntrySeverity::INFORMATION;
        let description = String::from("Entry kjournal_entry_new");

        // Create new entry.
        let mut j = KJournalEntry::new(severity, description.clone());

        // Copy previous date and time
        let dt = j.get_date_time().clone();

        // Wait 1 seconds (to get a different system time).
        sleep(Duration::new(1, 0));

        // Update value and entry
        let severity = KJournalEntrySeverity::ERROR;
        let description = String::from("Entry updated");
        j.update(severity, description.clone());

        // Verify each parameter value.
        assert!(j.get_severity() == severity, "Error! Updated entry severity is different!");
        assert!(j.get_description().eq(&description), "Error! Updated entry description is different!");

        // Make sure that system date and time taken is valid.
        match j.get_date_time().duration_since(dt) {
            Ok(n) => 
            assert!(n.as_millis() > 0, "Error! Date and time difference should be higher than 0!"),
            Err(_) => panic!("Error! SystemTime before UNIX EPOCH!"),
        }

    }
    



}