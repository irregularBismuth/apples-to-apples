use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct RedCard {
    id: usize,
    name: String,
    text: String,
}

impl RedCard {
    pub fn new(id: usize, name: String, text: String) -> Self {
        Self { id, name, text }
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
