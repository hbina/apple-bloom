//! Support for OpenApi version 3.0.1 specification.
//!
//! See the
//! [specification](https://github.com/OAI/OpenAPI-Specification/blob/aa91a19c43f8a12c02efa42d64794e396473f3b1/versions/3.0.3.md)
//! for more information.

mod components;
mod extension;
mod schema;

pub use crate::v3::{components::*, extension::*, schema::*};

// Yet OpenAPI dont have an implemented representation
// the `serde_json::Value` is used in place of a custom enum
// We re-expose the `serde_json::Value`, this way users does not have to include the dependency.
pub use serde_json::Value;
