use olympus_kleio::journal::{listener::KJournalListenerList, KJournalListenerPrint, KJournalEntrySeverity};

/// # Test
/// kjournal_listener_list_new
/// 
/// # Description
/// Create a new intance of KJournalListenerList.
/// 
/// # Verification(s)
/// V1 | KJournalListenerList::new() create a new instance without error.
#[test]
fn kjournal_listener_list_new() {
    // V1 | KJournalListenerList::new() create a new instance without error.
    let _ = KJournalListenerList::new();
}

/// # Test
/// kjournal_listener_list_add_listener
/// 
/// # Description
/// Add listener to instance of KJournalListenerList.
/// 
/// # Verification(s)
/// V1 | Initial KJournalListenerList::count() should be 0.
/// V2 | KJournalListenerPrint::new() create a new instance without error.
/// V3 | KJournalListenerList::add_listener() add listener created.
/// V4 | KJournalListenerList::count() should be 1.
#[test]
fn kjournal_listener_list_add_listener() {

    let mut list = KJournalListenerList::new();

    // V1 | Initial KJournalListenerList::count() should be 0.
    assert!(list.count() == 0, "TEST 'kjournal_listener_list_add_listener' | KJournalListenerList::count() should be 0!");

    // V2 | KJournalListenerPrint::new() create a new instance without error
    let listener = KJournalListenerPrint::new(KJournalEntrySeverity::ALL_WITH_DEBUG);

    // V3 | KJournalListenerList::add_listener() add listener created.
    list.add_listener(&listener);

    // V4 | KJournalListenerList::count() should be 1.
    assert!(list.count() == 1, "TEST 'kjournal_listener_list_add_listener' | KJournalListenerList::count() should be 1!");
}

/// # Test
/// kjournal_listener_list_add_listener_twice
/// 
/// # Description
/// Add the same listener twice to KJournalListenerList.
/// 
/// # Verification(s)
/// V1 | KJournalListenerList::add_listener() should panic!
#[test]
#[should_panic]
fn kjournal_listener_list_add_listener_twice() {
    todo!()


    // V1 | KJournalListenerList::add_listener() should panic!
}


/// # Test
/// kjournal_listener_list_remove_listener
/// 
/// # Description
/// Remove a listener from KJournalListenerList.
/// 
/// # Verification(s)
/// V1 | KJournalListener::remove_listener() should remove listener without error.
/// V2 | KJournalListenerList::count() should be 0 after removal.
#[test]
fn kjournal_listener_list_remove_listener() {
    todo!()

    // V1 | KJournalListener::remove_listener() should remove listener without error.
    // V2 | KJournalListenerList::count() should be 0 after removal.
}

/// # Test
/// kjournal_listener_list_remove_listener_not_added
/// 
/// # Description
/// Try to remove a listener from KJournalListenerList that was not added in the first place.
/// 
/// # Verification(s)
/// V1 | KJournalListener::remove_listener() should panic! trying to remove unknown listener.
#[test]
#[should_panic]
fn kjournal_listener_list_remove_listener_not_added() {
    todo!()

    // V1 | KJournalListener::remove_listener() should panic! trying to remove unknown listener.
}

/// # Test
/// kjournal_listener_list_clear
/// 
/// # Description
/// Clear a list of listeners.
/// 
/// # Verification(s)
/// V1 | KJournalListenerList::clear() should remove all listeners without error.
/// V2 | KJournalListenerList::count() should be 0 after clearing.
#[test]
fn kjournal_listener_list_clear() {
    todo!()

    // V1 | KJournalListenerList::clear() should remove all listeners without error.
    // V2 | KJournalListenerList::count() should be 0 after clearing.
}


/// # Test
/// kjournal_listener_list_notify
/// 
/// # Description
/// Use an implementation of KJournalListener to verify if notify is called correctly.
/// 
/// # Verification(s)
/// V1 | Create 3 customs implementation of KJournalListener.
/// V2 | Add 3 listeners to KJournalListenerList.
/// V3 | KJournalListener::notify() should notify all 3 different listeners according to listeners severity.
#[test]
fn kjournal_listener_list_notify() {
    todo!()

    // V1 | Create 3 customs implementation of KJournalListener.
    // V2 | Add 3 listeners to KJournalListenerList.
    // V3 | KJournalListener::notify() should notify all 3 different listeners according to listeners severity.
}

/// # Test
/// kjournal_listener_list_notify_empty
/// 
/// # Description
/// Notify an empty list of listeners.
/// 
/// # Verification(s)
/// V1 | KJournalListener::notify() should NOT panic! when empty and notify is called.
#[test]
fn kjournal_listener_list_notify_empty() {
    todo!()

    // V1 | KJournalListener::notify() should NOT panic! when empty and notify is called.
}

/// # Test
/// kjournal_listener_list_stress
/// 
/// # Description
/// Stress test a KJournalListenerList.
/// 
/// # Verification(s)
/// V1 | Add a great quantity of listener.
/// V2 | Remove a great quantity of listener.
/// V3 | Notify a large quantity of listeners.
/// V4 | Clear all listeners
/// V5 | Repeat a large number of time.
#[test]
fn kjournal_listener_list_stress() {
    todo!()

    // V1 | Add a great quantity of listener.
    // V2 | Remove a great quantity of listener.
    // V3 | Notify a large quantity of listeners.
    // V4 | Clear all listeners
    // V5 | Repeat a large number of time.
}
