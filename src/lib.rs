//! Openapi provides structures and support for serializing and deserializing [openapi](https://github.com/OAI/OpenAPI-Specification) specifications
//!
//! # Examples
//!
//! Typical use deserialing an existing to a persisted spec to rust form or
//! visa versa
//!
//! The hyper client should be configured with tls.
//!
//! ```no_run
//! extern crate openapi;
//!
//! fn main() {
//!   match openapi::from_path("path/to/openapi.yaml") {
//!     Ok(spec) => println!("spec: {:?}", spec),
//!     Err(err) => println!("error: {}", err)
//!   }
//! }
//! ```
//!
//! # Errors
//!
//! Operations typically result in a [`Result`] type, an alias for
//! [`std::result::Result`] with the `Err` type fixed to [`Error`],
//! which implements [`std::error::Error`].
//!
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path, result::Result as StdResult};

pub mod error;
mod util;
pub mod v2;
pub mod v3_0;

pub use error::Error;

const MINIMUM_OPENAPI30_VERSION: &str = ">= 3.0";

pub type Result<T> = StdResult<T, Error>;

/// Supported versions of the OpenApi.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum OpenApi {
    /// Version 2.0 of the OpenApi specification.
    ///
    /// Refer to the official
    /// [specification](https://github.com/OAI/OpenAPI-Specification/blob/0dd79f6/versions/2.0.md)
    /// for more information.
    V2(Box<v2::Spec>),

    /// Version 3.0.1 of the OpenApi specification.
    ///
    /// Refer to the official
    /// [specification](https://github.com/OAI/OpenAPI-Specification/blob/0dd79f6/versions/3.0.1.md)
    /// for more information.
    #[allow(non_camel_case_types)]
    V3_0(Box<v3_0::Spec>),
}

/// deserialize an open api spec from a path
pub fn from_path<P>(path: P) -> Result<OpenApi>
where
    P: AsRef<Path>,
{
    from_reader(File::open(path)?)
}

/// deserialize an open api spec from type which implements Read
pub fn from_reader<R>(read: R) -> Result<OpenApi>
where
    R: Read,
{
    Ok(serde_yaml::from_reader::<R, OpenApi>(read)?)
}

/// serialize to a yaml string
pub fn to_yaml(spec: &OpenApi) -> Result<String> {
    Ok(serde_yaml::to_string(spec)?)
}

/// serialize to a json string
pub fn to_json(spec: &OpenApi) -> Result<String> {
    Ok(serde_json::to_string_pretty(spec)?)
}
