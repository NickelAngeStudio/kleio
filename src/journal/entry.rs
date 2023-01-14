use std::time::SystemTime;


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
    /// Severity of the entry according to [`super::KJournalEntrySeverity`].
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
    /// * `severity` - [`super::KJournalEntrySeverity`] of the entry as u8.
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
    /// * `severity` - [`super::KJournalEntrySeverity`] of the entry as u8.
    /// * `description` - Description or metadata of entry.
    pub fn update(&mut self, severity : u8, description : String){
        self.date_time = SystemTime::now();
        self.severity = severity;
        self.description = description;
    }

    /// Get entry [`super::KJournalEntrySeverity`].
    /// 
    /// # Return
    /// Entry [`super::KJournalEntrySeverity`].
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
