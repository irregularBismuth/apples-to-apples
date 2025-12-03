use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct RedCard {
    id: u32,
    name: Box<str>,
    text: Box<str>,
}

impl RedCard {
    pub fn new(id: u32, name: impl Into<Box<str>>, text: impl Into<Box<str>>) -> Self {
        Self {
            id,
            name: name.into(),
            text: text.into(),
        }
    }
}
