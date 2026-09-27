#![forbid(unsafe_code)]

pub mod apps;
pub mod prompts;
pub mod protocol;
pub mod resources;
pub mod server;
pub mod tasks;
pub mod tools;

#[cfg(test)]
mod tests;

pub use protocol::*;
pub use server::handle_mcp_request;
