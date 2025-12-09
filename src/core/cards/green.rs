use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct GreenCard {
    id: usize,
    name: Box<str>,
    text: Box<str>,
}

impl GreenCard {
    pub fn new(id: usize, name: impl Into<Box<str>>, text: impl Into<Box<str>>) -> Self {
        Self {
            id,
            name: name.into(),
            text: text.into(),
        }
    }

    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }
}
