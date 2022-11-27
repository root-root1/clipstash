use crate::domain::time::Time;
use crate::domain::clip::ClipError;
use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expires(Option<Time>);

impl Expires {
    pub fn new<T: Into<Option<Time>>>(expire:T) -> Self {
        Self(expire.into())
    }

    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}

impl Default for Expires {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Expires {
    type Err = ClipError;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.is_empty() {
            Ok(Self(None))
        }else{
            match Time::from_str(raw) {
                Ok(time) => Ok(Self::new(time)),
                Err(e) => Err(e.into())
            }
        }
    }
}
