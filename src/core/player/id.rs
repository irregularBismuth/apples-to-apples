use serde::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(Serialize, Deserialize, Default, Copy, Clone)]
pub struct PlayerId(u32);

impl PlayerId {
    pub fn new(id: u32) -> Self {
        Self { 0: id }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}
