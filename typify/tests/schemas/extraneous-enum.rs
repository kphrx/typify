#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "LetterBox"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"LetterBox\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"letter\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"a\","]
#[doc = "        \"b\","]
#[doc = "        \"cee\""]
#[doc = "      ],"]
#[doc = "      \"maxLength\": 2"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LetterBox {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub letter: Option<LetterBoxLetter>,
}
impl From<&LetterBox> for LetterBox {
    fn from(value: &LetterBox) -> Self {
        value.clone()
    }
}
#[doc = "LetterBoxLetter"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"a\","]
#[doc = "    \"b\","]
#[doc = "    \"cee\""]
#[doc = "  ],"]
#[doc = "  \"maxLength\": 2"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LetterBoxLetter {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B,
}
impl From<&LetterBoxLetter> for LetterBoxLetter {
    fn from(value: &LetterBoxLetter) -> Self {
        value.clone()
    }
}
impl ToString for LetterBoxLetter {
    fn to_string(&self) -> String {
        match *self {
            Self::A => "a".to_string(),
            Self::B => "b".to_string(),
        }
    }
}
impl std::str::FromStr for LetterBoxLetter {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LetterBoxLetter {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LetterBoxLetter {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LetterBoxLetter {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
fn main() {}
