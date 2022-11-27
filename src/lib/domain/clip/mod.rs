pub mod field;

use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClipError{
    #[error("Invalid Password: {0}")]
    InvalidPassword(String),
    #[error("Invalid Title: {0}")]
    InvalidTitle(String),
    #[error("Content Can't be Empty")]
    EmptyContent,
    #[error("Invalid Date: {0}")]
    InvalidDate(String),
    #[error("Failed to Parse the Date: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("Failed to Parse Id: {0}")]
    Id(#[from] uuid::Error),
    #[error("Failed to Parse Hits: {0}")]
    Hits(#[from] std::num::TryFromIntError)
}

#[derive(Debug, Clone,Deserialize, Serialize )]
pub struct Clip {
    pub clip_id: field::ClipId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}
