use serde::{Serialize, Deserialize};

use crate::reader::Reader;

pub trait DepositoryItem {
    fn get_isbn_code(&self) -> &String; 
    fn get_title(&self) -> &String;
    fn get_year(&self) -> i32;
    fn borrow_for_reader(self, reader: &mut Reader);
}

impl DepositoryItem for Book {
    fn borrow_for_reader(self, reader: &mut Reader) {
        reader.books_borrowed.push(self)
    }
    fn get_title(&self) -> &String {
        return &self.title;
    }
    fn get_year(&self) -> i32 {
        return self.year;   
    }
    fn get_isbn_code(&self) -> &String {
        return &self.isbn_code;
    }
}

impl DepositoryItem for Magazine {
    fn borrow_for_reader(self, reader: &mut Reader) {
        reader.magazines_borrowed.push(self)
    }
    fn get_title(&self) -> &String {
        return &self.title;
    }
    fn get_year(&self) -> i32 {
        return self.year;   
    }
    fn get_isbn_code(&self) -> &String {
        return &self.isbn_code;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Book {
    pub isbn_code: String,
    pub title: String,
    pub year: i32
}

impl Book {
    pub fn create(isbn_code: String, title: String, year: i32) -> Self {
        Self {
            isbn_code,
            title: title.to_string(),
            year,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Magazine {
    pub isbn_code: String,
    pub title: String,
    pub year: i32,
    pub super_hero: String
}

impl Magazine {
    pub fn create(isbn_code: String, title: String, year: i32, super_hero: String) -> Self {
        Self {
            isbn_code,
            title: title.to_string(),
            year,
            super_hero: super_hero.to_string(),
        }
    }
}