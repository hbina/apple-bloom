//! Support for OpenApi version 3.0.1 specification.
//!
//! See the
//! [specification](https://github.com/OAI/OpenAPI-Specification/blob/0dd79f6/versions/3.0.1.md)
//! for more information.

mod components;
mod extension;
mod helper;
mod schema;
mod schema_object;

pub use crate::v3_0::{components::*, extension::*, helper::*, schema::*, schema_object::*};

// Yet OpenAPI dont have an implemented representation
// the `serde_json::Value` is used in place of a custom enum
// We re-expose the `serde_json::Value`, this way users does not have to include the dependency.
pub use serde_json::Value;

#[test]
pub fn test_v3_json() {
    // TODO: Test that YAML and JSON produces the same result?
    use crate::util::test_util::{get_all_file_paths_in_directory, parse_from_file};
    let files = get_all_file_paths_in_directory("./data/v3.0").unwrap();
    for file in files {
        let _: Spec = parse_from_file(file).unwrap();
    }
}
