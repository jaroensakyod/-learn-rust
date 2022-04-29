pub mod server;
pub mod error;
pub mod method;
pub mod query_string;

pub use server::Server;
pub use  error::Error;
pub use method::Method;
pub use 

pub type Result<T> = std::result::Result<T,Error>;
