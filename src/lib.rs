use event::KEventLog;
use journal::KJournal;
use window::KWindowSupplier;


 /// ##### Kleio struct used for I/O operations.
/// 
/// TODO : Description
/// 
/// TODO : Example
/// # Examples
///
/// ``` 
/// 
/// ```
pub struct Kleio {
    // Main Journal
    journal: KJournal,

    // Events log
    event_log : KEventLog,

    // Window supplier
    window_supplier : Box<dyn KWindowSupplier>,
}

// Kleio journal interface
pub mod journal;

// Kleio Asset manager
pub mod asset;

// Kleio event interface
pub mod event;

// Kleio window interface
pub mod window;