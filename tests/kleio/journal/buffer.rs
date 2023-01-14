use olympus_kleio::journal::{journal::{KJournalBuffer, KJOURNAL_BUFFER_MIN, KJOURNAL_BUFFER_MAX}, KJournalEntrySeverity, KJournalEntry};

/// Create a new instance of KJournalBuffer.
#[test]
fn kjournal_buffer_new() {
    let _ = KJournalBuffer::new(50);
}

/// Create a new instance of KJournalBuffer smaller than KJOURNAL_BUFFER_MIN .
#[test]
#[should_panic]
fn kjournal_buffer_new_below_min() {
    let _ = KJournalBuffer::new(KJOURNAL_BUFFER_MIN - 1);
}

/// Create a new instance of KJournalBuffer bigger than KJOURNAL_BUFFER_MAX .
#[test]
#[should_panic]
fn kjournal_buffer_new_above_max() {
    let _ = KJournalBuffer::new(KJOURNAL_BUFFER_MAX + 1);
}

/// Test KJournalBuffer::size().
#[test]
fn kjournal_buffer_size() {
    let size = 250;
    let kb = KJournalBuffer::new(size);

    assert!(kb.size() == size, "TEST 'kjournal_buffer_size' | KJournalBuffer::size() is incorrect!");

}

/// Test writing entries to a journal buffer
#[test]
fn kjournal_buffer_write() {

    let size = 250;
    let mut kb = KJournalBuffer::new(size);

    // Write debug entry
    kb.write(KJournalEntrySeverity::DEBUG, "Debug entry".to_owned());

    // Get latest entry and make sure it matches what was written
    validate_journal_entry(kb.latest(), KJournalEntrySeverity::DEBUG, &"Debug entry".to_owned());

    // Write a bunch of entries.
    for _ in 0..255 {
        kb.write(KJournalEntrySeverity::DEBUG, "Debug entry".to_owned());
        kb.write(KJournalEntrySeverity::OTHER, "Other entry".to_owned());
        kb.write(KJournalEntrySeverity::INFORMATION, "Information entry".to_owned());
        kb.write(KJournalEntrySeverity::WARNING, "Warning entry".to_owned());
        kb.write(KJournalEntrySeverity::ERROR, "Error entry".to_owned());
        kb.write(KJournalEntrySeverity::FATAL, "Fatal entry".to_owned());
    }

    // Match latests entries
    validate_journal_entry(kb.latest(), KJournalEntrySeverity::FATAL, &"Fatal entry".to_owned());
    validate_journal_entry(kb.latest(), KJournalEntrySeverity::ERROR, &"Error entry".to_owned());
    validate_journal_entry(kb.latest(), KJournalEntrySeverity::WARNING, &"Warning entry".to_owned());
    validate_journal_entry(kb.latest(), KJournalEntrySeverity::INFORMATION, &"Information entry".to_owned());
    validate_journal_entry(kb.latest(), KJournalEntrySeverity::OTHER, &"Other entry".to_owned());
    validate_journal_entry(kb.latest(), KJournalEntrySeverity::DEBUG, &"Debug entry".to_owned());

}


/// Test getting count of unread entries
#[test]
fn kjournal_buffer_unread() {
    
    let size = 250;
    let mut kb = KJournalBuffer::new(size);

    // Unread should be 0.
    assert!(kb.unread() == 0, "TEST 'kjournal_buffer_unread' | Unread ({}) count incorrect! Should be {}!", kb.unread(), 0);

    // Write debug entry
    kb.write(KJournalEntrySeverity::DEBUG, "Debug entry".to_owned());

    // Unread should be 1
    assert!(kb.unread() == 1, "TEST 'kjournal_buffer_unread' | Unread ({}) count incorrect! Should be {}!", kb.unread(), 1);

    // Get latest
    kb.latest();

    // Unread should be 0.
    assert!(kb.unread() == 0, "TEST 'kjournal_buffer_unread' | Unread ({}) count incorrect! Should be {}!", kb.unread(), 0);

    // Write a bunch of entries.
    for _ in 0..255 {
        kb.write(KJournalEntrySeverity::DEBUG, "Debug entry".to_owned());
        kb.write(KJournalEntrySeverity::OTHER, "Other entry".to_owned());
        kb.write(KJournalEntrySeverity::INFORMATION, "Information entry".to_owned());
        kb.write(KJournalEntrySeverity::WARNING, "Warning entry".to_owned());
        kb.write(KJournalEntrySeverity::ERROR, "Error entry".to_owned());
        kb.write(KJournalEntrySeverity::FATAL, "Fatal entry".to_owned());

    }

    // Unread should be size.
    assert!(kb.unread() == kb.size(), "TEST 'kjournal_buffer_unread' | Unread ({}) count incorrect! Should be {}!", kb.unread(), kb.size());

     // Get latest
     kb.latest();

     // Unread should be size - 1.
    assert!(kb.unread() == kb.size() - 1, "TEST 'kjournal_buffer_unread' | Unread ({}) count incorrect! Should be {}!", kb.unread(), kb.size() - 1);

    // Get 128 entries
    for _ in 0..128{
        kb.latest();
    }

    // Unread should be size - 129.
    assert!(kb.unread() == kb.size() - 129, "TEST 'kjournal_buffer_unread' | Unread ({}) count incorrect! Should be {}!", kb.unread(), kb.size() - 129);

    // Clear entries
    kb.clear();

     // Unread should be 0.
     assert!(kb.unread() == 0, "TEST 'kjournal_buffer_unread' | Unread ({}) count incorrect! Should be {}!", kb.unread(), 0);


}



/// Stress test buffer. Test is ignored since long run time.
#[test]
#[ignore]
fn kjournal_buffer_stress() {
    
    // Test an array of available buffer size
    for size in (KJOURNAL_BUFFER_MIN..KJOURNAL_BUFFER_MAX).step_by((KJOURNAL_BUFFER_MAX - KJOURNAL_BUFFER_MIN) / 100) {
        let mut kb = KJournalBuffer::new(size);

        // Unread should be 0.
        assert!(kb.unread() == 0, "TEST 'kjournal_buffer_stress' | Unread ({}) count incorrect! Should be {}!", kb.unread(), 0);

        // Write debug entry
        kb.write(KJournalEntrySeverity::DEBUG, "Debug entry".to_owned());

        // Unread should be 1
        assert!(kb.unread() == 1, "TEST 'kjournal_buffer_stress' | Unread ({}) count incorrect! Should be {}!", kb.unread(), 1);

        // Write a bunch of entries.
        for _ in 0..size * 2 {
            kb.write(KJournalEntrySeverity::DEBUG, "Debug entry".to_owned());
            kb.write(KJournalEntrySeverity::OTHER, "Other entry".to_owned());
            kb.write(KJournalEntrySeverity::INFORMATION, "Information entry".to_owned());
            kb.write(KJournalEntrySeverity::WARNING, "Warning entry".to_owned());
            kb.write(KJournalEntrySeverity::ERROR, "Error entry".to_owned());
            kb.write(KJournalEntrySeverity::FATAL, "Fatal entry".to_owned());

        }

        // Unread should be size.
        assert!(kb.unread() == kb.size(), "TEST 'kjournal_buffer_stress' | Unread ({}) count incorrect! Should be {}!", kb.unread(), kb.size());
    }
}


/************
* FUNCTIONS * 
************/
/// Validate a journal entry. Will panic if entry validation failed.
fn validate_journal_entry(entry: Option<&KJournalEntry>, severity:u8, desc : &String){

    match entry {
        Some(entry) => {
            assert!(entry.get_severity() == severity, "TEST 'kjournal_buffer_write' | New entry severity incorrect. {} != {}!", entry.get_severity(), severity);
            assert!(entry.get_description().eq(desc), "TEST 'kjournal_buffer_write' | New entry description incorrect!");
            match entry.get_date_time().elapsed() {
                // Elapsed should be really small.
                Ok(elapsed) => assert!(elapsed.as_millis() <= 10,  "TEST 'kjournal_buffer_write' | New entry date and time incorrect!"),
                Err(_) => assert!(false, "TEST 'kjournal_buffer_write' | New entry date and time incorrect!"),
            }
        },
        None => assert!(false, "TEST 'kjournal_buffer_write' | Journal entry write failed!"),
    }

}