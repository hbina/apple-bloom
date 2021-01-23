use crate::v3_0::helper::ObjectOrReference;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::BTreeMap;

use super::helper::BooleanObjectOrReference;

// FIXME: Verify against OpenAPI 3.0
/// The Schema Object allows the definition of input and output data types.
/// These types can be objects, but also primitives and arrays.
/// This object is an extended subset of the
/// [JSON Schema Specification Wright Draft 00](http://json-schema.org/).
/// For more information about the properties, see
/// [JSON Schema Core](https://tools.ietf.org/html/draft-wright-json-schema-00) and
/// [JSON Schema Validation](https://tools.ietf.org/html/draft-wright-json-schema-validation-00).
/// Unless stated otherwise, the property definitions follow the JSON Schema.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#schemaObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SchemaObject {
    /// Properties.
    /// The following properties are taken directly from the [JSON Schema](https://tools.ietf.org/html/draft-wright-json-schema-00) definition and follow the same specification.
    /// TODO(hbina): Extend support to all of this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "multipleOf", skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<Value>>,
    /// OpenAPI Specific Properties.
    /// The following properties are taken from the JSON Schema definition but their definitions were adjusted to the OpenAPI Specification.
    // TODO(hbina): Extend support to all of this.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<String>,
    // Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard
    // JSON Schema.
    // [oneOf-anyOf-](https://swagger.io/docs/specification/data-models/oneof-anyof-allof-not/#oneof)
    #[serde(rename = "oneOf", skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<ObjectOrReference<SchemaObject>>>,
    #[serde(rename = "allOf", skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<ObjectOrReference<SchemaObject>>>,
    #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<ObjectOrReference<SchemaObject>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Vec<ObjectOrReference<SchemaObject>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<ObjectOrReference<SchemaObject>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, ObjectOrReference<SchemaObject>>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "additionalProperties"
    )]
    pub additional_properties: Option<BooleanObjectOrReference<Box<SchemaObject>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    // UNCHECKED
    // #[serde(skip_serializing_if = "Option::is_none", rename = "readOnly")]
    // pub read_only: Option<bool>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub nullable: Option<bool>,
    // Value can be boolean or object. Inline or referenced schema MUST be of a
    // [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#schemaObject)
    // and not a standard JSON Schema.
    //
    // See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#properties>.
    // A free-form property to include an example of an instance for this schema.
    // To represent examples that cannot be naturally represented in JSON or YAML,
    // a string value can be used to contain the example with escaping where necessary.
    // NOTE: According to [spec], _Primitive data types in the OAS are based on the
    //       types supported by the JSON Schema Specification Wright Draft 00._
    //       This suggest using
    //       [`serde_json::Value`](https://docs.serde.rs/serde_json/value/enum.Value.html). [spec][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#data-types]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub example: Option<serde_json::value::Value>,
    // The following properties are taken directly from the JSON Schema definition and
    // follow the same specifications:
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub maximum: Option<i32>,
    // #[serde(skip_serializing_if = "Option::is_none", rename = "exclusiveMaximum")]
    // pub exclusive_maximum: Option<bool>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub minimum: Option<i32>,
    // #[serde(skip_serializing_if = "Option::is_none", rename = "exclusiveMinimum")]
    // pub exclusive_minimum: Option<bool>,
    // #[serde(skip_serializing_if = "Option::is_none", rename = "maxLength")]
    // pub max_length: Option<u32>,
    // #[serde(skip_serializing_if = "Option::is_none", rename = "minLength")]
    // pub min_length: Option<u32>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub pattern: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none", rename = "maxItems")]
    // pub max_items: Option<u32>,
    // #[serde(skip_serializing_if = "Option::is_none", rename = "minItems")]
    // pub min_items: Option<u32>,
    // #[serde(skip_serializing_if = "Option::is_none", rename = "uniqueItems")]
    // pub unique_items: Option<bool>,
    // #[serde(skip_serializing_if = "Option::is_none", rename = "maxProperties")]
    // pub max_properties: Option<u32>,
    // #[serde(skip_serializing_if = "Option::is_none", rename = "minProperties")]
    // pub min_properties: Option<u32>,
    // The following properties are taken from the JSON Schema definition but their
    // definitions were adjusted to the OpenAPI Specification.
    // - type - Value MUST be a string. Multiple types via an array are not supported.
    // - allOf - Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - oneOf - Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - anyOf - Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - not - Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - items - Value MUST be an object and not an array. Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema. `items` MUST be present if the `type` is `array`.
    // - properties - Property definitions MUST be a [Schema Object](#schemaObject) and not a standard JSON Schema (inline or referenced).
    // - additionalProperties - Value can be boolean or object. Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - description - [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    // - format - See [Data Type Formats](#dataTypeFormat) for further details. While relying on JSON Schema's defined formats, the OAS offers a few additional predefined formats.
    // - default - The default value represents what would be assumed by the consumer of the input as the value of the schema if one is not provided. Unlike JSON Schema, the value MUST conform to the defined type for the Schema Object defined at the same level. For example, if `type` is `string`, then `default` can be `"foo"` but cannot be `1`.
    // The default value represents what would be assumed by the consumer of the input as the value
    // of the schema if one is not provided. Unlike JSON Schema, the value MUST conform to the
    // defined type for the Schema Object defined at the same level. For example, if type is
    // `string`, then `default` can be `"foo"` but cannot be `1`.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub default: Option<serde_json::Value>,
    // Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard
    // JSON Schema.
    // [allOf](https://swagger.io/docs/specification/data-models/oneof-anyof-allof-not/#allof)
    // Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard
    // JSON Schema.
    // [anyOf](https://swagger.io/docs/specification/data-models/oneof-anyof-allof-not/#anyof)
    // #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
    // pub any_of: Option<Vec<ObjectOrReference<Schema>>>,
    // Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard
    // JSON Schema.
    // [not](https://swagger.io/docs/specification/data-models/oneof-anyof-allof-not/#not)
    // #[serde(rename = "not", skip_serializing_if = "Option::is_none")]
    // pub not: Option<Vec<ObjectOrReference<Schema>>>,
    // [Specification extensions](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.2.md#specificationExtensions)
    // #[serde(flatten)]
    // pub extensions: HashMap<String, String>,
}
