use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]

// Add ISBN code for book and mag
pub struct Book {
    pub title: String,
    pub year: i32
}