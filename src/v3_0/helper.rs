use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Ref {
    #[serde(rename = "$ref")]
    pub ref_path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ObjectOrReference<T> {
    Object(T),
    Ref(Ref),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum BooleanObjectOrReference<T> {
    Boolean(bool),
    Object(T),
    Ref(Ref),
}