mod core; // Core server implementation.
mod handlers; // Request handlers.
mod middleware;
mod models; // Data models. // Middleware components.

// Re-export the main server function and any other public interfaces.
pub use core::run_server;
pub use models::{BuilderError, LanguageTranslation, Translation, TranslationResponse};
