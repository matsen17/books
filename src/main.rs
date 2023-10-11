use std::{io, str::FromStr};
use std::env;

use magazine::Magazine;
use reader::Borrow;
use reader::Reader;
use book::Book;
use command::Command;

mod book;
mod magazine;
mod reader;
mod persistence;
mod command;

fn main() {
    let mut depository_books: Vec<Book>;
    let mut depository_magazines: Vec<Magazine>;
    let mut readers: Vec<Reader>;

    match persistence::load_from_file("books_depo.json") {
        Ok(read_result, ) => {
            depository_books = read_result.0;
            depository_magazines = read_result.1;
            readers = read_result.2;  
        },
        Err(err) => {
            eprintln!("Error loading book depository data: {}", err);
            return;
        },
    }

    let args: Vec<String> = env::args().collect();
    let command_argument = &args[1];
    let command = Command::from_str(&command_argument).unwrap();

    match command {
        Command::AddBook => {
            let mut book_name = String::new();
            let mut book_year = String::new();
            let stdin = io::stdin();

            println!("Enter book name");
            stdin.read_line(&mut book_name).unwrap();
            book_name = book_name.trim().to_string();

            println!("Enter book year");
            stdin.read_line(&mut book_year).unwrap();

            let book_year_int = match book_year.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input!");
                    return;
                }
            };

            depository_books.push(add_book(&book_name, book_year_int));
        },
        Command::ListBooks => {
            list_books(&depository_books);
        },
        Command::RemoveBook => {
            let mut book_name = String::new();
            let mut book_year = String::new();
            let stdin = io::stdin();

            println!("Enter book name");
            stdin.read_line(&mut book_name).unwrap();
            book_name = book_name.trim().to_string();

            println!("Enter book year");
            stdin.read_line(&mut book_year).unwrap();

            let book_year_int = match book_year.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input!");
                    return;
                }
            };

            let book_to_remove = Book { title: book_name, year: book_year_int };
            remove_book(&mut depository_books, &book_to_remove);
        },
        Command::BorrowBookForUser => {
            let mut selected_book_id = String::new();
            let selected_book: Book;

            let mut selected_reader_name = String::new();

            let stdin = io::stdin();
            list_books(&depository_books);

            println!("Select book to borrow by index:");
            stdin.read_line(&mut selected_book_id).unwrap();

            let book_id = match selected_book_id.trim().parse::<i32>() {
                Ok(num) => num - 1,
                Err(_) => {
                    eprintln!("Invalid input!");
                    return;
                }
            };

            match find_book_by_id(&depository_books, book_id) {
                Some(found_book) => {
                    selected_book = found_book;
                },
                None => {
                    println!("No book found with ID {}", selected_book_id);
                    return;
                }
            }

            println!("Enter borrowing reader name:");
            stdin.read_line(&mut selected_reader_name)?;
            selected_reader_name = selected_reader_name.trim().to_string();

            borrow_book_for_reader(&mut readers, &selected_book, selected_reader_name, &mut depository_books);
        },
        Command::AddReader => {
            let mut reader_name = String::new();
            let stdin = io::stdin();

            println!("Enter reader name");
            stdin.read_line(&mut reader_name).unwrap();
            reader_name = reader_name.trim().to_string();

            readers.push(add_user(&reader_name));
        },
        Command::AddMagazine => {

        },
        Command::BorrowMagazineForUser => {

        }
    }

    match persistence::save_to_file("books_depo.json", &depository_books, &depository_magazines, &readers) {
        Ok(()) => println!("Data saved successfully."),
        Err(err) => eprintln!("Error saving data: {}", err),
    }
}

fn borrow_book_for_reader(readers: &mut Vec<Reader>, selected_book: &Book, selected_reader_name: String, books: &mut Vec<Book>) {
    for i in 0..readers.len() {
        if readers[i].name.trim().to_lowercase() == selected_reader_name.to_lowercase() {
            readers[i].borrow_book(&selected_book);
            remove_book(books, &selected_book);
            return;
        }
    }
    println!("No reader was found");
}

fn add_book(title: &str, year: i32) -> Book {
    println!("Adding a new book called {title} and published in {year}");
    Book {
        title: String::from(title),
        year: year
    }
}

fn add_user(name: &str) -> Reader {
    println!("Adding a new reader called {name}");
    Reader {
        name: String::from(name),
        books_borrowed: Vec::new(),
        magazines_borrowed: Vec::new(),
    }
}

fn remove_book(books: &mut Vec<Book>, book_to_remove: &Book) {
    for i in (0..books.len()).rev() {
        if books[i].title == book_to_remove.title 
        && books[i].year == book_to_remove.year {
            books.swap_remove(i);
            println!("Found and removed book");
            break;
        }
    }
}

fn list_books(books: &Vec<Book>) {
    for i in 0..books.len() {
        println!("{}. Title: {} Year {}", i + 1, books[i].title, books[i].year)
    }
}

fn find_book_by_id (books: &Vec<Book>, book_id: i32) -> Option<Book> {
    //slices, lifetimes
    //books.iter().filter(|book| {(*book).id}) 0 cost abstraction;
    for i in 0..books.len() {
        if i== book_id as usize {
            return Some(Book { title: String::from(books[i].title.clone()) , year: books[i].year.clone()});
        }
    }
    None
}
