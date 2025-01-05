mod auth;
mod core; // Core server implementation.
mod handlers; // Request handlers.
mod models; // Data models. // AuthN/Z middleware.

// Re-export the main server function and any other public interfaces.
pub use core::run_server;
