use olympus_kleio::window::{KWindowManager, KEvent, KWindow};

/*********
* CONSTS *
*********/
/// ID of KWindowManagerHollow
pub const KWINDOW_MANAGER_HOLLOW_ID: u8 = 123;

/// X position of KWindow
pub const POSX:isize = 100;

/// Y position of KWindow
pub const POSY:isize = 200;

/// Width of KWindow
pub const WIDTH:usize = 300;

/// Height of KWindow
pub const HEIGHT:usize = 400;


/********
* TESTS *
********/
#[test]
/// Create a KWindow from KWindowManagerHollow
/// 
/// # Verification(s)
/// V1 | KWindow::from() create KWindow without error.
fn kwindow_from() {
    let wm = KWindowManagerHollow::new(POSX,POSY,WIDTH,HEIGHT);

    match wm {
        Ok(wm) => {
            // V1 | KWindow::from() create KWindow without error.
            KWindow::from(Box::new(wm));
        },
        Err(_) => assert!(false, "Error creating KWindowManagerHollow!"),
    }
}

#[test]
/// Get the manager ID from KWindow
/// 
/// # Verification(s)
/// V1 | KWindow::get_window_manager_id() gives KWINDOW_MANAGER_HOLLOW_ID as ID.
fn kwindow_get_window_manager_id() {

    let w = KWindow::from(Box::new(KWindowManagerHollow::new(POSX,POSY,WIDTH,HEIGHT).unwrap()));

    // V1 | KWindow::get_window_manager_id() gives KWINDOW_MANAGER_HOLLOW_ID as ID.
    assert!(w.get_window_manager_id() == KWINDOW_MANAGER_HOLLOW_ID, "KWindowManager id error!");
}

#[test]
/// Downcast KWindowManager to KWindowManagerHollow.
/// 
/// # Verification(s)
/// V1 | KWindow::downcast_window_manager() correctly downcast to KWindowManagerHollow.
/// V2 | KWindowManagerHollow::get_true() works.
fn kwindow_downcast_window_manager() {

    let w = KWindow::from(Box::new(KWindowManagerHollow::new(POSX,POSY,WIDTH,HEIGHT).unwrap()));


    // V1 | KWindow::downcast_window_manager() correctly downcast to KWindowManagerHollow.
    w.downcast_window_manager::<KWindowManagerHollow>();
    match KWindow::downcast_window_manager::<KWindowManagerHollow>(&w) {
        Some(s) => todo!(),
        None => todo!(),
    }


    assert!(w.get_window_manager_id() == KWINDOW_MANAGER_HOLLOW_ID, "KWindowManager id error!");
}


/**********
* STRUCTS *
**********/
/// Hollow KWindowManager used for KWindowTest
struct KWindowManagerHollow {

    /// Contains a pre-defined list of KEvent for tests
    events : Vec<KEvent>
}

impl KWindowManagerHollow {
    /// Function implemented that always return true. Used for downcast tests.
    pub fn get_true() -> bool {
        true
    }
}

impl KWindowManager for KWindowManagerHollow {
    fn new(_pos_x:isize, _pos_y:isize, _width:usize, _height:usize) -> Result<Self, olympus_kleio::window::KWindowError> where Self: Sized {
        todo!()

        // TODO: Create events in self.events
    }

    fn get_event_count(&self) -> usize {
        todo!()
    }

    fn poll_event(&mut self) -> olympus_kleio::window::KEvent {
        todo!()
    }

    fn sync_event(&self) {
        todo!()
    }

    fn get_id(&self) -> u8 {
        KWINDOW_MANAGER_HOLLOW_ID
    }

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }
}
