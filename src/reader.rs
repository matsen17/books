use serde::{Serialize, Deserialize};
use crate::book::Book;
use crate::book::Magazine;

#[derive(Clone, Serialize, Deserialize)]
pub struct Reader {
    pub name: String,
    pub books_borrowed: Vec<Book>,
    pub magazines_borrowed: Vec<Magazine>
}

impl Reader {
    pub fn create(name: &str) -> Self {
        Self { 
            name: String::from(name),
            books_borrowed: Vec::new(),
            magazines_borrowed: Vec::new()
        }
    }
}