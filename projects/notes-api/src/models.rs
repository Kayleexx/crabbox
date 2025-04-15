use serde::{Serialize, Deserialize}; 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: u32,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub content: String,
}
