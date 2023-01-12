use olympus_kleio::journal::journal::KJournalBuffer;

/******************
* KJournalBuffer *
*****************/
/// Create a new instance of KJournalBuffer.
#[test]
fn kjournal_buffer_new() {
    let kb = KJournalBuffer::new(50);
}

/// Test KJournalBuffer::size().
#[test]
fn kjournal_buffer_size() {
    let size = 250;
    let kb = KJournalBuffer::new(size);

    assert!(kb.size() == size, "KJournalBuffer::size() is incorrect!");

}

/// Test writing entry to a journal buffer
#[test]
fn kjournal_buffer_write() {
    todo!()
}

/// Test getting latest entry for journal buffer
#[test]
fn kjournal_buffer_latest() {
    todo!()
}

/// Test getting count of unread entries
#[test]
fn kjournal_buffer_unread() {
    todo!()
}

/// Test clearing buffer
#[test]
fn kjournal_buffer_clear() {
    todo!()
}


/// Stress test buffer
#[test]
fn kjournal_buffer_stress() {
    todo!()
}






/***********************
* KJournalListenerList *
***********************/

/***********
* KJournal * 
***********/




// KJournal::new()



//KJournal::get_max_entries()

//KJournal::listeners()




