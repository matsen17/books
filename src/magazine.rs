use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Magazine {
    pub title: String,
    pub year: i32,
    pub super_hero: String
}