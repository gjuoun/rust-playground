use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dog {
    pub id: usize,
    pub url: String,
}

impl Dog {
    pub fn new(id: usize, url: String) -> Self {
        Self { id, url }
    }

    // Convert from database row tuple to Dog
    pub fn from_row(row: (usize, String)) -> Self {
        let (id, url) = row;
        Self { id, url }
    }
}
