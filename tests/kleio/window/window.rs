use std::{cell::RefCell, rc::Rc};

use olympus_kleio::window::{KWindowManager, KEvent, KWindow, KEventReceiver, KWindowError, KEventController, KEventKeyboard, KEventMouse};

use crate::{assert_err, assert_ok};

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

/// Count of total event given by KWindowManagerHollow
pub const EVENT_COUNT:usize = 30;

/********
* TESTS *
********/
#[test]
/// Create a KWindow from KWindowManagerHollow
/// 
/// # Verification(s)
/// V1 | KWindow::from() create KWindow without error.
fn kwindow_from() {
    // V1 | KWindow::from() create KWindow without error.
    KWindow::from(Box::new(assert_ok!(KWindowManagerHollow::new(POSX,POSY,WIDTH,HEIGHT))));
}

#[test]
/// Get the manager ID from KWindow
/// 
/// # Verification(s)
/// V1 | KWindow::get_window_manager_id() gives KWINDOW_MANAGER_HOLLOW_ID as ID.
fn kwindow_get_window_manager_id() {

    let w =  KWindow::from(Box::new(assert_ok!(KWindowManagerHollow::new(POSX,POSY,WIDTH,HEIGHT))));

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

    let w =  KWindow::from(Box::new(assert_ok!(KWindowManagerHollow::new(POSX,POSY,WIDTH,HEIGHT))));


    // V1 | KWindow::downcast_window_manager() correctly downcast to KWindowManagerHollow.
    w.downcast_window_manager::<KWindowManagerHollow>();
    match KWindow::downcast_window_manager::<KWindowManagerHollow>(&w) {
        Some(wm) => {
            // V2 | KWindowManagerHollow::get_true() works.
            assert!(wm.get_true(), "KWindow::downcast_window_manager() error!")
        },
        None => assert!(false, "KWindow::downcast_window_manager() error!"),
    }
}

#[test]
/// Add an event receiver to KWindow
/// 
/// # Verification(s)
/// V1 | KWindow::add_event_receiver() correctly add receiver to KWindow.
/// V2 | Adding the same receiver via KWindow::add_event_receiver() should result in KWindowError::ReceiverAlreadyExists.
fn kwindow_add_event_receiver() {

    let mut w =  KWindow::from(Box::new(assert_ok!(KWindowManagerHollow::new(POSX,POSY,WIDTH,HEIGHT))));

    // V1 | KWindow::add_event_receiver() correctly add receiver to KWindow.
    let rc1 = Rc::new(KEventReceiverHollow::new(true, true, true, true));
    assert_ok!(w.add_event_receiver(rc1.clone()), 0);

    // V2 | Adding the same receiver via KWindow::add_event_receiver() should result in KWindowError::ReceiverAlreadyExists.
    assert_err!(w.add_event_receiver(rc1.clone()), KWindowError::ReceiverAlreadyExists);
}

#[test]
/// Remove an event receiver from KWindow
/// 
/// # Verification(s)
/// V1 | KWindow::remove_event_receiver() should return KWindowError::ReceiverNotFound since receiver was not added.
/// V2 | KWindow::add_event_receiver() correctly add receiver to KWindow.
/// V3 | KWindow::remove_event_receiver() should return Ok(0).
fn kwindow_remove_event_receiver() {

    let mut w =  KWindow::from(Box::new(assert_ok!(KWindowManagerHollow::new(POSX,POSY,WIDTH,HEIGHT))));
    let rc1 = Rc::new(KEventReceiverHollow::new(true, true, true, true));

    // V1 | KWindow::remove_event_receiver() should return KWindowError::ReceiverNotFound since receiver was not added.
    assert_err!(w.remove_event_receiver(rc1.clone()), KWindowError::ReceiverNotFound);

    // V2 | KWindow::add_event_receiver() correctly add receiver to KWindow.
    assert_ok!(w.add_event_receiver(rc1.clone()), 0);

    // V3 | KWindow::remove_event_receiver() should return Ok(0).
    assert_ok!(w.remove_event_receiver(rc1.clone()), 0);
}



#[test]
/// Dispatch event of KWindow to multiple receivers.
/// 
/// # Verification(s)
/// V1 | KWindow::dispatch_events() should return KWindowError::DispatchNoReceiver since no receivers added.
/// V2 | Add 6 different receiver with different handling configuration. 
/// V3 | KWindow::dispatch_events() should now work without issue and return Ok(EVENT_COUNT).
/// V4 | Compare different receiver notification count with control.
/// V5 | KWindow::dispatch_events() should return Ok(0).
/// V6 | KWindow::remove_event_receiver() for each receiver should return Ok(index).
fn kwindow_dispatch_events() {
    let mut w = KWindow::from(Box::new(KWindowManagerHollow::new(POSX,POSY,WIDTH,HEIGHT).unwrap()));

    // V1 | KWindow::dispatch_events() should return KWindowError::DispatchNoReceiver since no receivers added.
    match w.dispatch_events() {
        Ok(_) => assert!(false, "KWindow::dispatch_events() should give an error!"),
        Err(err) => match err {
            KWindowError::DispatchNoReceiver => {},
            _ => assert!(false, "KWindow::dispatch_events() wrong error!")
        },
    }

    // V2 | Add 6 different receiver with different handling configuration. 
    let rc1 = Rc::new(KEventReceiverHollow::new(true, true, true, true));
    let rc2 = Rc::new(KEventReceiverHollow::new(true, false, false, false));
    let rc3 = Rc::new(KEventReceiverHollow::new(false, true, false, false));
    let rc4 = Rc::new(KEventReceiverHollow::new(false, false, true, false));
    let rc5 = Rc::new(KEventReceiverHollow::new(false, false, false, true));
    let rc6 = Rc::new(KEventReceiverHollow::new(false, true, true, true));

    assert_ok!(w.add_event_receiver(rc1.clone()), 0);
    assert_ok!(w.add_event_receiver(rc2.clone()), 1);
    assert_ok!(w.add_event_receiver(rc3.clone()), 2);
    assert_ok!(w.add_event_receiver(rc4.clone()), 3);
    assert_ok!(w.add_event_receiver(rc5.clone()), 4);
    assert_ok!(w.add_event_receiver(rc6.clone()), 5);

    // V3 | KWindow::dispatch_events() should now work without issue and return Ok(EVENT_COUNT).
    assert_ok!(w.dispatch_events(), EVENT_COUNT);

    // TODO:V4 | Compare different receiver notification count with control.


    // V5 | KWindow::dispatch_events() should return Ok(0).
    assert_ok!(w.dispatch_events(), 0);


    // V6 | KWindow::remove_event_receiver() for each receiver should return Ok(index).
    assert_ok!(w.add_event_receiver(rc6.clone()), 5);
    assert_ok!(w.add_event_receiver(rc5.clone()), 4);
    assert_ok!(w.add_event_receiver(rc4.clone()), 3);
    assert_ok!(w.add_event_receiver(rc3.clone()), 2);
    assert_ok!(w.add_event_receiver(rc2.clone()), 1);
    assert_ok!(w.add_event_receiver(rc1.clone()), 0);
    
    
    
    
    

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
    pub fn get_true(&self) -> bool {
        true
    }
}

impl KWindowManager for KWindowManagerHollow {
    fn new(_pos_x:isize, _pos_y:isize, _width:usize, _height:usize) -> Result<Self, olympus_kleio::window::KWindowError> where Self: Sized {
        //Create events in self.events up to EVENT_COUNT
        let events : Vec<KEvent> = Vec::new();

        // Controller events
        events.push(KEvent::Controller(KEventController::Connected(0)));
        events.push(KEvent::Controller(KEventController::ButtonDown(0, 1)));
        events.push(KEvent::Controller(KEventController::ButtonUp(0, 1)));
        events.push(KEvent::Controller(KEventController::Axis(0, 0, -250)));
        events.push(KEvent::Controller(KEventController::Disconnected(0)));

        // Keyboard events
        events.push(KEvent::Keyboard(KEventKeyboard::KeyDown(123)));
        events.push(KEvent::Keyboard(KEventKeyboard::KeyUp(123)));

        // Mouse events
        events.push(KEvent::Mouse(KEventMouse::Moved(10,10,0,0)));
        events.push(KEvent::Mouse(KEventMouse::ButtonDown(1,10,10)));
        events.push(KEvent::Mouse(KEventMouse::ButtonUp(1,10,10)));
        events.push(KEvent::Mouse(KEventMouse::Wheel(-255,255)));

        // Window events
        


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



/// ##### Custom receiver that count how many time it was notified.
struct KEventReceiverHollow {

    /// Is the receiver enabled or not.
    enabled : bool,

    /// Flag returned when handling window event received.
    handle_window : bool,

    /// Flag returned when handling keyboard event received.
    handle_keyboard : bool,

    /// Flag returned when handling mouse event received.
    handle_mouse : bool,

    /// Flag returned when handling controller event received.
    handle_controller : bool,

    // Was listener notified.
    notification_count: Rc<RefCell<usize>>,
}

impl KEventReceiverHollow {
    /// Create a new instance of KEventReceiverHollow which handle event or not.
    /// 
    /// Returns new KEventReceiverHollow created.
    pub fn new(handle_window : bool, handle_keyboard : bool, handle_mouse : bool, handle_controller : bool) -> KEventReceiverHollow {
        KEventReceiverHollow { enabled: true, handle_window, handle_keyboard, handle_mouse, handle_controller, notification_count : Rc::new(RefCell::new(0)) }
    }

    /// Get the count of notifications.
    pub fn get_notification_count(&self) -> usize {
        let a = self.notification_count.clone();
        a.as_ref().take()
    }

    /// Set if the receiver is enabled or not.
    pub fn set_enabled(&mut self, enabled : bool){
        self.enabled = enabled;
    }
}

impl KEventReceiver for KEventReceiverHollow {
    fn receive(&self, event : &KEvent) -> bool {
        
        // Increment notifications
        let a = self.notification_count.clone();
        let b = a.as_ref();
        b.replace(b.take() + 1);
        
        match event {
            KEvent::Unknown => panic!("Error : Unknown event received!"),

            KEvent::Window(_) => self.handle_window,
            KEvent::Keyboard(_) => self.handle_keyboard,
            KEvent::Mouse(_) => self.handle_mouse,
            KEvent::Controller(_) => self.handle_controller,            
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}