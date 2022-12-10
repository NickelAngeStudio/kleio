


/// # Re-export for Public API
#[doc(inline)]
pub use entry::KJournalEntrySeverity as KJournalEntrySeverity;
pub use entry::KJournalEntry as KJournalEntry;

// Kleio asset source
#[doc(hidden)]
pub mod journal;

// Kleio asset source implementation for file system
#[doc(hidden)]
pub mod entry;

// Kleio asset broker
#[doc(hidden)]
pub mod listener;
