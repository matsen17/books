use serde::{Serialize, Deserialize};
use crate::book::Book;
use crate::magazine::Magazine;

#[derive(Clone, Serialize, Deserialize)]
pub struct Reader {
    pub name: String,
    pub books_borrowed: Vec<Book>,
    pub magazines_borrowed: Vec<Magazine>
}

//Borrow move to books and magazine
pub trait Borrow {
    fn borrow_book(&mut self, book: &Book);
    fn borrow_magazine(&mut self, magazine: &Magazine);
}

impl Borrow for Reader {
    fn borrow_book(&mut self, book: &Book) {
        self.books_borrowed.push(book.clone())
    }

    fn borrow_magazine(&mut self, magazine:  &Magazine) {
        self.magazines_borrowed.push(magazine.clone())
    }
}