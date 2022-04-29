pub mod server;
pub mod error;
pub mod method;
pub mod query_string;
pub mod request;
pub mod status;
pub mod response;

pub
pub use server::Server;
pub use  error::Error;
pub use method::Method;
pub use query_string::QueryString;
pub use request::Request;

pub type Result<T> = std::result::Result<T,Error>;
