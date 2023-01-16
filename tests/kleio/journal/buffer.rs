use olympus_kleio::journal::{journal::{KJournalBuffer, KJOURNAL_BUFFER_MIN, KJOURNAL_BUFFER_MAX}, KJournalEntrySeverity, KJournalEntry};

/// Used test buffer size. Is padded in function that use it to make sure it is between MIN and MAX.
static KJOURNAL_BUFFER_TEST_SIZE:usize = 250;

/// # Test
/// kjournal_buffer_new
/// 
/// # Description
/// Create a new instance of KJournalBuffer with size in the middle of KJOURNAL_BUFFER_MIN and KJOURNAL_BUFFER_MAX
/// 
/// # Verification(s)
/// KJournalBuffer::new(KJOURNAL_BUFFER_TEST_SIZE) created without error.
#[test]
fn kjournal_buffer_new() {
    // Used test buffer size is KJOURNAL_BUFFER_TEST_SIZE. If min is bigger, it become min. If max is lower, it become max.
    let buffer_size:usize = std::cmp::min(std::cmp::max(KJOURNAL_BUFFER_TEST_SIZE, KJOURNAL_BUFFER_MIN), KJOURNAL_BUFFER_MAX);

    let _ = KJournalBuffer::new(buffer_size);
}

/// # Test
/// kjournal_buffer_new_min
/// 
/// # Description
/// Create a new instance of KJournalBuffer with minimal allowed size.
/// 
/// # Verification(s)
/// KJournalBuffer::new(KJOURNAL_BUFFER_MIN) created without error.
#[test]
fn kjournal_buffer_new_min() {
    let _ = KJournalBuffer::new(KJOURNAL_BUFFER_MIN);
}

/// # Test
/// kjournal_buffer_new_max
/// 
/// # Description
/// Create a new instance of KJournalBuffer with maximum allowed size.
/// 
/// # Verification(s)
/// KJournalBuffer::new(KJOURNAL_BUFFER_MAX) created without error.
#[test]
fn kjournal_buffer_new_max() {
    let _ = KJournalBuffer::new(KJOURNAL_BUFFER_MAX);
}

/// # Test
/// kjournal_buffer_new_below_min
/// 
/// # Description
/// Create a new instance of KJournalBuffer with size below allowed minimum.
/// 
/// # Verification(s)
/// * KJournalBuffer::new(KJOURNAL_BUFFER_MIN - 1) should panic.
#[test]
#[should_panic]
fn kjournal_buffer_new_below_min() {
    let _ = KJournalBuffer::new(KJOURNAL_BUFFER_MIN - 1);
}

/// # Test
/// kjournal_buffer_new_above_max
/// 
/// # Description
/// Create a new instance of KJournalBuffer with size above allowed maximum.
/// 
/// # Verification(s)
/// * KJournalBuffer::new(KJOURNAL_BUFFER_MAX + 1) should panic.
#[test]
#[should_panic]
fn kjournal_buffer_new_above_max() {
    let _ = KJournalBuffer::new(KJOURNAL_BUFFER_MAX + 1);
}

/// # Test
/// kjournal_buffer_size
/// 
/// # Description
/// Verify the size of KJournalBuffer created.
/// 
/// # Verification(s)
/// * KJournalBuffer::size() should be equal to created size.
#[test]
fn kjournal_buffer_size() {
    // Used test buffer size is KJOURNAL_BUFFER_TEST_SIZE. If min is bigger, it become min. If max is lower, it become max.
    let buffer_size:usize = std::cmp::min(std::cmp::max(KJOURNAL_BUFFER_TEST_SIZE, KJOURNAL_BUFFER_MIN), KJOURNAL_BUFFER_MAX);

    let kb = KJournalBuffer::new(buffer_size);
    assert!(kb.size() == buffer_size, "TEST 'kjournal_buffer_size' | KJournalBuffer::size() is incorrect!");
}

/// # Test
/// kjournal_buffer_none
/// 
/// # Description
/// Verify that KJournalBuffer::latest() return None when no entries.
/// 
/// # Verification(s)
/// * KJournalBuffer::latest() return None when empty.
#[test]
fn kjournal_buffer_latest_none() {
     // Used test buffer size is KJOURNAL_BUFFER_TEST_SIZE. If min is bigger, it become min. If max is lower, it become max.
     let buffer_size:usize = std::cmp::min(std::cmp::max(KJOURNAL_BUFFER_TEST_SIZE, KJOURNAL_BUFFER_MIN), KJOURNAL_BUFFER_MAX);
 
     let mut kb = KJournalBuffer::new(buffer_size);

     match kb.latest() {
        Some(_) => assert!(false, "TEST 'kjournal_buffer_latest_none' | KJournalBuffer::latest() should give None when empty!"),
        None => {}, // OK do nothing
    }
} 

/// # Test
/// kjournal_buffer_write
/// 
/// # Description
/// Verify writing entries into buffer.
/// 
/// # Verification(s)
/// * KJournalBuffer::write() write an entry into buffer without error.
/// * Entry created is verified.
/// * Write multiple different entries without error.
/// * Write more entries that buffer MAX without error.
/// * Retrieve and verify entries in correct order. (latest to oldest)
#[test]
fn kjournal_buffer_write() {
    // Used test buffer size is KJOURNAL_BUFFER_TEST_SIZE. If min is bigger, it become min. If max is lower, it become max.
    let buffer_size:usize = std::cmp::min(std::cmp::max(KJOURNAL_BUFFER_TEST_SIZE, KJOURNAL_BUFFER_MIN), KJOURNAL_BUFFER_MAX);
    let funct_name = "kjournal_buffer_write";

    let mut kb = KJournalBuffer::new(buffer_size);

    // Write debug entry
    kb.write(KJournalEntrySeverity::DEBUG, "Debug entry".to_owned());

    // Get latest entry and make sure it matches what was written
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::DEBUG, &"Debug entry".to_owned(), funct_name);

    // Write a bunch of entries.
    for _ in 0..(KJOURNAL_BUFFER_MAX + 5) {
        kb.write(KJournalEntrySeverity::DEBUG, "Debug entry".to_owned());
        kb.write(KJournalEntrySeverity::OTHER, "Other entry".to_owned());
        kb.write(KJournalEntrySeverity::INFORMATION, "Information entry".to_owned());
        kb.write(KJournalEntrySeverity::WARNING, "Warning entry".to_owned());
        kb.write(KJournalEntrySeverity::ERROR, "Error entry".to_owned());
        kb.write(KJournalEntrySeverity::FATAL, "Fatal entry".to_owned());
    }

    // Match latests entries
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::FATAL, &"Fatal entry".to_owned(), funct_name);
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::ERROR, &"Error entry".to_owned(), funct_name);
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::WARNING, &"Warning entry".to_owned(), funct_name);
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::INFORMATION, &"Information entry".to_owned(), funct_name);
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::OTHER, &"Other entry".to_owned(), funct_name);
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::DEBUG, &"Debug entry".to_owned(), funct_name);

}

/// # Test
/// kjournal_buffer_unread
/// 
/// # Description
/// Verify that buffer unread count is accurate.
/// 
/// # Verification(s)
/// * KJournalBuffer::unread() initial size must be 0.
/// * KJournalBuffer::unread() size must be 1 after writing an entry.
/// * KJournalBuffer::unread() should go back to 0 after reading entry.
/// * KJournalBuffer::unread() size should be buffer_size once full.
/// * KJournalBuffer::unread() size should be buffer_size - 1 once an entry is read.
/// * KJournalBuffer::unread() size should be buffer_size - (buffer_size / 2) - 1 after reading another buffer_size / 2 entries.
/// * KJournalBuffer::clear() should clear the buffer without error.
/// * KJournalBuffer::unread() size must be 0 after clear().
#[test]
fn kjournal_buffer_unread() {
    // Used test buffer size is KJOURNAL_BUFFER_TEST_SIZE. If min is bigger, it become min. If max is lower, it become max.
    let buffer_size:usize = std::cmp::min(std::cmp::max(KJOURNAL_BUFFER_TEST_SIZE, KJOURNAL_BUFFER_MIN), KJOURNAL_BUFFER_MAX);

    let mut kb = KJournalBuffer::new(buffer_size);

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
    for _ in 0..(KJOURNAL_BUFFER_MAX + 5) {
        kb.write(KJournalEntrySeverity::DEBUG, "Debug entry".to_owned());
        kb.write(KJournalEntrySeverity::OTHER, "Other entry".to_owned());
        kb.write(KJournalEntrySeverity::INFORMATION, "Information entry".to_owned());
        kb.write(KJournalEntrySeverity::WARNING, "Warning entry".to_owned());
        kb.write(KJournalEntrySeverity::ERROR, "Error entry".to_owned());
        kb.write(KJournalEntrySeverity::FATAL, "Fatal entry".to_owned());

    }

    // Unread should be size.
    assert!(kb.unread() == kb.size(), "TEST 'kjournal_buffer_unread' | Unread ({}) count incorrect! Should be {}!", kb.unread(), buffer_size);

     // Get latest
     kb.latest();

     // Unread should be size - 1.
    assert!(kb.unread() == kb.size() - 1, "TEST 'kjournal_buffer_unread' | Unread ({}) count incorrect! Should be {}!", kb.unread(), buffer_size - 1);

    let entries_read = buffer_size / 2;
    // Get 128 entries
    for _ in 0..entries_read{
        kb.latest();
    }

    // Unread should be size - entries_read - 1.
    assert!(kb.unread() == kb.size() - entries_read - 1, "TEST 'kjournal_buffer_unread' | Unread ({}) count incorrect! Should be {}!", kb.unread(), buffer_size - entries_read - 1);

    // Clear entries
    kb.clear();

     // Unread should be 0.
     assert!(kb.unread() == 0, "TEST 'kjournal_buffer_unread' | Unread ({}) count incorrect! Should be {}!", kb.unread(), 0);


}


/// # Test
/// kjournal_buffer_stress
/// 
/// # Description
/// Stress test KJournalBuffer to test stability and limit.
/// 
/// # Verification(s)
/// * Multiple different buffer size ranging from MIN to MAX
/// * Verify initial size() is 0.
/// * Write an entry into buffer and verify size is 1.
/// * Verify that entry values are correct.
/// * Fill buffer with entries.
/// * Verify size() is equal to buffer_size.
/// * Verify 6 latest entries.
/// * Verify size() is equal to buffer_size - 6.
/// * Read each entry until None is returned.
/// * Verify that size() is now 0.
/// * Repeat until MAX is reached.
#[test]
#[ignore]
fn kjournal_buffer_stress() {
    let funct_name = "kjournal_buffer_stress";
    
    // Test an array of available buffer size
    for buffer_size in (KJOURNAL_BUFFER_MIN..KJOURNAL_BUFFER_MAX).step_by((KJOURNAL_BUFFER_MAX - KJOURNAL_BUFFER_MIN) / 100) {
        let mut kb = KJournalBuffer::new(buffer_size);

        // Unread should be 0.
        assert!(kb.unread() == 0, "TEST 'kjournal_buffer_stress' | Unread ({}) count incorrect! Should be {}!", kb.unread(), 0);

        // Write debug entry
        kb.write(KJournalEntrySeverity::DEBUG, "Debug entry".to_owned());

        // Unread should be 1
        assert!(kb.unread() == 1, "TEST 'kjournal_buffer_stress' | Unread ({}) count incorrect! Should be {}!", kb.unread(), 1);

        // Get latest entry and make sure it matches what was written
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::DEBUG, &"Debug entry".to_owned(), funct_name);

        // Fill buffer with a bunch of entries.
        for _ in 0..buffer_size + 5{
            kb.write(KJournalEntrySeverity::DEBUG, "Debug entry".to_owned());
            kb.write(KJournalEntrySeverity::OTHER, "Other entry".to_owned());
            kb.write(KJournalEntrySeverity::INFORMATION, "Information entry".to_owned());
            kb.write(KJournalEntrySeverity::WARNING, "Warning entry".to_owned());
            kb.write(KJournalEntrySeverity::ERROR, "Error entry".to_owned());
            kb.write(KJournalEntrySeverity::FATAL, "Fatal entry".to_owned());

        }

        // Unread should be size.
        assert!(kb.unread() == buffer_size, "TEST 'kjournal_buffer_stress' | Unread ({}) count incorrect! Should be {}!", kb.unread(), buffer_size);

        // Match latests entries
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::FATAL, &"Fatal entry".to_owned(), funct_name);
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::ERROR, &"Error entry".to_owned(), funct_name);
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::WARNING, &"Warning entry".to_owned(), funct_name);
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::INFORMATION, &"Information entry".to_owned(), funct_name);
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::OTHER, &"Other entry".to_owned(), funct_name);
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::DEBUG, &"Debug entry".to_owned(), funct_name);

        // Unread should be size - 6.
        assert!(kb.unread() == buffer_size - 6, "TEST 'kjournal_buffer_stress' | Unread ({}) count incorrect! Should be {}!", kb.unread(), buffer_size);

        // Read entries until none is reached. Will assert an error if None is never reached.
        for i in 0..buffer_size {
            match kb.latest() {
                Some(_) => assert!(i < buffer_size - 1, "TEST 'kjournal_buffer_stress' | None neaver reached while reading entries!"), 
                None => break,  // Break loop
            }
        }

        // Unread should be 0.
        assert!(kb.unread() == 0, "TEST 'kjournal_buffer_stress' | Unread ({}) count incorrect! Should be {}!", kb.unread(), 0);
    }
}


/************
* FUNCTIONS * 
************/
/// Verify a journal entry. Will panic if entry verification failed.
/// 
/// # Argument(s)
/// * `entry` - Entry to verify.
/// * `severity` - Severity that the entry should have.
/// * `desc` - Description the entry should have.
/// * `funct_name` - Name of the test function that called verify.
/// 
/// # Panic
/// Will panic if any entry parameters are wrong.
fn verify_journal_entry(entry: Option<&KJournalEntry>, severity:u8, desc : &String, funct_name : &str){

    match entry {
        Some(entry) => {
            assert!(entry.get_severity() == severity, "TEST {:?} | New entry severity incorrect. {} != {}!", funct_name, entry.get_severity(), severity);
            assert!(entry.get_description().eq(desc), "TEST {:?} | New entry description incorrect!", funct_name);
            match entry.get_date_time().elapsed() {
                // Elapsed should be really small.
                Ok(elapsed) => assert!(elapsed.as_millis() <= 10,  "TEST {:?} | New entry date and time incorrect!", funct_name),
                Err(_) => assert!(false, "TEST {:?} | New entry date and time incorrect!", funct_name),
            }
        },
        None => assert!(false, "TEST {:?} | Journal entry write failed!", funct_name),
    }

}