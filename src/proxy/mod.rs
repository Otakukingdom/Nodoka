//! In-memory cache layer that wraps database query results.
//!
//! The proxy module provides handle types ([`audiobook_handle::AudiobookHandle`],
//! [`audiobook_file_handle::AudiobookFileHandle`]) that cache the most recently
//! loaded audiobook data to avoid repeated database round-trips during UI rendering.
//!
//! All types in this module use [`std::rc::Rc`] and are therefore single-threaded.
//! This module is crate-internal by design and must not be used across thread
//! boundaries.

pub(crate) mod audiobook_file_handle;
pub(crate) mod audiobook_handle;
pub(crate) mod manager;

pub use audiobook_file_handle::AudiobookFileHandle;
pub use audiobook_handle::AudiobookHandle;
pub use manager::Cache;
