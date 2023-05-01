use crate::prelude::{Error, Result};

use regex::Regex;
use serde::de::{Deserialize, Deserializer};
use serde_derive::Serialize;

const HOST_ID_REGEX: &str = r"^[a-zA-Z0-9_][a-zA-Z0-9_\-]*$";

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct HostId(String);

impl HostId {
    /// Create a new host identifier from a string.
    ///
    /// Example:
    ///
    /// ```rust
    /// use tricorder::prelude::HostId;
    ///
    /// let id = HostId::new("example").unwrap();
    /// # assert_eq!(id.to_string(), String::from("example"));
    /// ```
    pub fn new(src: &str) -> Result<Self> {
        let re = Regex::new(HOST_ID_REGEX)?;
        if !re.is_match(src) {
            Err(Box::new(Error::InvalidHostId(format!(
                "ID {} does not match regex {}",
                src, HOST_ID_REGEX
            ))))
        } else {
            Ok(Self(src.to_string()))
        }
    }

    /// Return the underlying string
    pub fn to_string(self) -> String {
        let Self(s) = self;
        s.clone()
    }
}

impl<'de> Deserialize<'de> for HostId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let src = String::deserialize(deserializer)?;
        HostId::new(src.as_str()).map_err(serde::de::Error::custom)
    }
}
