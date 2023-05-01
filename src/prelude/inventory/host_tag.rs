use crate::prelude::{Error, Result};

use regex::Regex;
use serde::de::{Deserialize, Deserializer};
use serde_derive::Serialize;

const HOST_TAG_REGEX: &str = r"^[^!\&\|\t\n\r\f\(\) ]+$";

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct HostTag(String);

impl HostTag {
    /// Create a new host identifier from a string.
    ///
    /// Example:
    ///
    /// ```rust
    /// use tricorder::prelude::HostTag;
    ///
    /// let tag = HostTag::new("example").unwrap();
    /// # assert_eq!(tag.to_string(), String::from("example"));
    /// ```
    pub fn new(src: &str) -> Result<Self> {
        let re = Regex::new(HOST_TAG_REGEX)?;
        if !re.is_match(src) {
            Err(Box::new(Error::InvalidHostTag(format!(
                "Tag {} does not match regex {}",
                src, HOST_TAG_REGEX
            ))))
        } else {
            Ok(Self(src.to_string()))
        }
    }

    pub fn to_string(self) -> String {
        let Self(s) = self;
        s
    }
}

impl<'de> Deserialize<'de> for HostTag {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let src = String::deserialize(deserializer)?;
        HostTag::new(src.as_str()).map_err(serde::de::Error::custom)
    }
}
