use super::KEvent;

/// # Dispatch [KWindow](struct.KWindow.html) [KEvent] to multiple receiver. 
/// TODO: More doc
pub struct KEventDispatcher {

}

/// # Receive [KEvent] from [KEventDispatcher] and handle them if needed. 
/// TODO: More doc
pub trait KEventReceiver {

    /// # Receive a [KEvent] from the dispatcher.
    /// 
    /// Return True if the [KEvent] has been handled, which will prevent other receiver from handling it.
    /// Return False if the [KEvent] wasn't handled, giving it to the next receiver.
    fn receive(&self, event : KEvent) -> bool;
}