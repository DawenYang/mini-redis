pub use connection::Connection;
pub use frame::Frame;

pub mod frame;

pub mod connection;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;