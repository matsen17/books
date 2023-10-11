use serde::{Serialize, Deserialize};
use crate::book::Magazine;
use crate::book::Book;
use crate::reader::Reader;

#[derive(Serialize, Deserialize)]
struct DataToSave {
    depository_books: Vec<Book>,
    depository_magazines: Vec<Magazine>,
    readers: Vec<Reader>,
}

#[derive(Serialize, Deserialize)]
struct DataToSaveOwned {
    depository_books: Vec<Book>,
    depository_magazines: Vec<Magazine>,
    readers: Vec<Reader>,
}

pub fn save_to_file(
    filename: &str,
    depository_books: &Vec<Book>,
    depository_magazines: &Vec<Magazine>,
    readers: &Vec<Reader>,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = DataToSave {
        depository_books: depository_books.to_vec(),
        depository_magazines: depository_magazines.to_vec(),
        readers: readers.to_vec(),
    };

    let json = serde_json::to_string(&data)?;
    std::fs::write(filename, json)?;
    Ok(())
}

pub fn load_from_file(
    filename: &str,
) -> Result<(Vec<Book>, Vec<Magazine>, Vec<Reader>), Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(filename)?;
    let data: DataToSaveOwned = serde_json::from_str(&json)?;
    Ok((data.depository_books, data.depository_magazines, data.readers))
}