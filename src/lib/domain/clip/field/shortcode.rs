use std::str::FromStr;
use crate::domain::clip::ClipError;
use rocket::form::FromFormField;
use serde::{Serialize, Deserialize};
use derive_more::From;
use rocket::{UriDisplayPath, UriDisplayQuery};
use rocket::request::FromParam;

#[derive(Debug, Clone, Deserialize, Serialize, From,UriDisplayPath, UriDisplayQuery)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        use rand::prelude::*;
        let mut rng = thread_rng();
        let mut shortcode = String::with_capacity(15);
        let allowed_char = [
            'a', 'b','c','b','e','f','g','h','i','j', 'k',
            'A', 'B','C','D','E','F','G','H','I','J', 'K',
            '1','2','3','4','5','6','7','8','9','0'
        ];

        for _ in 0..15 {
            shortcode.push(
                *allowed_char.choose(&mut rng)
                .expect("Their No Character in the array")
            )
        }

        Self(shortcode)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for ShortCode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ShortCode> for String {
    fn from(shortcode: ShortCode) -> Self {
        shortcode.0
    }
}

impl From<&str> for ShortCode {
    fn from(shortcode: &str) -> Self {
        ShortCode(shortcode.to_owned())
    }
}

impl FromStr for ShortCode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

impl<'r> FromParam<'r> for ShortCode {
    type Error = &'r str;
    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(ShortCode::from(param))
    }
}

impl<'a> FromFormField<'a> for ShortCode {
    fn default() -> Option<Self> {
        Some(Self::new())
    }
}
