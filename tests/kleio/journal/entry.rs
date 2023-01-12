/***************************
* KJournalEntry UNIT TESTS * 
***************************/

use std::{time::{SystemTime, Duration}, thread::sleep};
use olympus_kleio::journal::{KJournalEntry, KJournalEntrySeverity};

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
