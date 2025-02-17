#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "TestType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TestType\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"converted_type\","]
#[doc = "    \"patched_type\","]
#[doc = "    \"replaced_type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"converted_type\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        1,"]
#[doc = "        \"one\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"patched_type\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeThatNeedsMoreDerives\""]
#[doc = "    },"]
#[doc = "    \"replaced_type\": {"]
#[doc = "      \"$ref\": \"#/definitions/HandGeneratedType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$comment\": \"validate replacement, patch, and conversion settings\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestType {
    pub converted_type: serde_json::Value,
    pub patched_type: TypeThatHasMoreDerives,
    pub replaced_type: String,
}
impl From<&TestType> for TestType {
    fn from(value: &TestType) -> Self {
        value.clone()
    }
}
#[doc = "TypeThatHasMoreDerives"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TypeThatHasMoreDerives(pub std::collections::HashMap<String, String>);
impl std::ops::Deref for TypeThatHasMoreDerives {
    type Target = std::collections::HashMap<String, String>;
    fn deref(&self) -> &std::collections::HashMap<String, String> {
        &self.0
    }
}
impl From<TypeThatHasMoreDerives> for std::collections::HashMap<String, String> {
    fn from(value: TypeThatHasMoreDerives) -> Self {
        value.0
    }
}
impl From<&TypeThatHasMoreDerives> for TypeThatHasMoreDerives {
    fn from(value: &TypeThatHasMoreDerives) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, String>> for TypeThatHasMoreDerives {
    fn from(value: std::collections::HashMap<String, String>) -> Self {
        Self(value)
    }
}
fn main() {}
