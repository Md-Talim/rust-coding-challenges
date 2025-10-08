use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    String(String),
    Object(BTreeMap<String, JsonValue>),
}
