pub mod server;
pub mod error;
pub mod me

pub use server::Server;
pub use  error::Error;

pub type Result<T> = std::result::Result<T,Error>;
