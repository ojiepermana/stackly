//! The Stackly domain model: entities, invariants, and state machines.
//!
//! This crate is the center of the architecture. It imports no other Stackly
//! crate, no async runtime, and no OS API, so it builds for every target.

mod instance_id;

pub use instance_id::{InstanceId, InvalidInstanceId};
