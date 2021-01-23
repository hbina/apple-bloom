//! Support for OpenApi version 2.0 specification.
//!
//! See the
//! [specification](https://github.com/OAI/OpenAPI-Specification/blob/0dd79f6/versions/2.0.md)
//! for more information.

mod schema;

pub use crate::v2::schema::*;

#[test]
pub fn test_v2_json() {
    use crate::util::test_util::{get_all_file_paths_in_directory, parse_from_file};
    let files = get_all_file_paths_in_directory("./data/v2.0/json").unwrap();
    for file in files {
        let _: Spec = parse_from_file(file).unwrap();
    }
}

#[test]
pub fn test_v2_yaml() {
    use crate::util::test_util::{get_all_file_paths_in_directory, parse_from_file};
    let files = get_all_file_paths_in_directory("./data/v2.0/yaml").unwrap();
    for file in files {
        let _: Spec = parse_from_file(file).unwrap();
    }
}
