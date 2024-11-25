mod client;
mod error;
pub mod operation;
mod stream_connection;

pub use client::CompaniesHouseStreamingClient;
pub use error::{CompaniesHouseStreamingConnectionError, CompaniesHouseStreamingNextError};
pub use stream_connection::StreamConnection;
