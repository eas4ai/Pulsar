//! Application middleware
//!
//! Each middleware has its own dedicated file following the framework convention.

pub mod authenticate;
mod logging;
mod public_cache;

pub use logging::LoggingMiddleware;
pub use public_cache::PublicCacheCookies;
