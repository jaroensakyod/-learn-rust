pub mod server;
pub mod error;

pub use server::Server;

pub type Result<T> = std::result::Result<T,>
