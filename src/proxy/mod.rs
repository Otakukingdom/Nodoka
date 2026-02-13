#[allow(clippy::module_name_repetitions)] // Proxy pattern naming is idiomatic
pub mod audiobook_file_proxy;
#[allow(clippy::module_name_repetitions)] // Proxy pattern naming is idiomatic
pub mod audiobook_proxy;
pub mod manager;

#[allow(clippy::module_name_repetitions)] // Proxy pattern naming is idiomatic
pub use audiobook_file_proxy::AudiobookFileProxy;
#[allow(clippy::module_name_repetitions)] // Proxy pattern naming is idiomatic
pub use audiobook_proxy::AudiobookProxy;
#[allow(clippy::module_name_repetitions)]
// Manager pattern naming is idiomatic for manager module
pub use manager::ProxyManager;
