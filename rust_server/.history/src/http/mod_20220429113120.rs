pub mod server;
pub mod error;

pub use server::Server;
pub use  error::Error;

pub type Result<T> = std::result::Result<T,>
