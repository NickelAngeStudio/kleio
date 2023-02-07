use super::KEvent;

/// # Dispatch [KWindow](struct.KWindow.html) [KEvent] to multiple receiver.
/// 
/// This struct is automatically created for each [KWindow](struct.KWindow.html).
pub struct KEventDispatcher {

}

impl KEventDispatcher {
    pub(crate) fn new() -> KEventDispatcher {
        KEventDispatcher{}
    }

    pub(crate) fn add_receiver() {
        todo!()
    }

    pub(crate) fn remove_receiver() {
        todo!()
    }
}

/// # Receive [KEvent] from [KWindow](struct.KWindow.html) and handle them if needed. 
/// TODO: More doc
pub trait KEventReceiver {

    /// # Receive a [KEvent] from the dispatcher.
    /// 
    /// Return True if the [KEvent] has been handled, which will prevent other receiver from handling it.
    /// Return False if the [KEvent] wasn't handled, giving it to the next receiver.
    fn receive(&self, event : KEvent) -> bool;
}