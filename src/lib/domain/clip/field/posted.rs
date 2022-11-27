use crate::domain::time::Time;
use serde::{Serialize, Deserialize};
use derive_more::Constructor;

#[derive(Debug, Constructor, Clone, Serialize, Deserialize)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time {
        self.0
    } 
}
