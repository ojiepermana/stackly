//! Stackly application use cases.
//!
//! Each use case is a thin function over the domain and the platform ports.
//! Engines (runtime, instance, supervisor, provider, environment, operation,
//! diagnostics) start as modules here and move to their own crate once their
//! boundary is proven.

mod daemon;

pub use daemon::{DaemonInfo, daemon_info};
