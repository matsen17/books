use std::{io, str::FromStr};
use std::env;

use reader::Reader;
use book::{Book, DepositoryItem};
use book::Magazine;
use command::Command;

mod book;
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
            let mut book_code = String::new();

            let stdin = io::stdin();

            println!("Enter book name");
            stdin.read_line(&mut book_name).unwrap();
            book_name = book_name.trim().to_string();

            println!("Enter book year");
            stdin.read_line(&mut book_year).unwrap();

            println!("Enter book isbn code");
            stdin.read_line(&mut book_code).unwrap();
            book_code = book_code.trim().to_string();

            let book_year_int = match book_year.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input!");
                    return;
                }
            };

            depository_books.push(book::Book::create(book_code, book_name, book_year_int));
        },
        Command::AddMagazine => {
            let mut magazine_name = String::new();
            let mut magazine_year = String::new();
            let mut magazine_code = String::new();
            let mut magazine_hero = String::new();

            let stdin = io::stdin();

            println!("Enter magazine name");
            stdin.read_line(&mut magazine_name).unwrap();
            magazine_name = magazine_name.trim().to_string();

            println!("Enter magazine year");
            stdin.read_line(&mut magazine_year).unwrap();

            println!("Enter magazine hero");
            stdin.read_line(&mut magazine_hero).unwrap();
            magazine_hero = magazine_hero.trim().to_string();

            println!("Enter magazine isbn code");
            stdin.read_line(&mut magazine_code).unwrap();
            magazine_code = magazine_code.trim().to_string();

            let magazine_year_int = match magazine_year.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input!");
                    return;
                }
            };

            depository_magazines.push(book::Magazine::create(magazine_code, magazine_name, magazine_year_int, magazine_hero));
        },
        Command::ListBooks => {
            list_items(&depository_books);
        },
        Command::ListMagazines => {
            list_items(&depository_magazines);
        },
        Command::RemoveBook => {
            let mut book_name = String::new();
            let mut book_year = String::new();
            let mut book_code = String::new();

            let stdin = io::stdin();

            println!("Enter book name");
            stdin.read_line(&mut book_name).unwrap();
            book_name = book_name.trim().to_string();

            println!("Enter book year");
            stdin.read_line(&mut book_year).unwrap();

            println!("Enter book isbn code");
            stdin.read_line(&mut book_code).unwrap();
            book_code = book_code.trim().to_string();

            let book_year_int = match book_year.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input!");
                    return;
                }
            };

            let book_to_remove = book::Book::create(book_code, book_name, book_year_int);
            remove_depository_item(&mut depository_books, &book_to_remove);
        },
        Command::BorrowBookForUser => {
            let mut selected_book_id = String::new();
            let selected_book: Book;

            let mut selected_reader_name = String::new();

            let stdin = io::stdin();
            list_items(&depository_books);

            println!("Select book to borrow by index:");
            stdin.read_line(&mut selected_book_id).unwrap();

            let book_id = match selected_book_id.trim().parse::<usize>() {
                Ok(num) => num - 1,
                Err(_) => {
                    eprintln!("Invalid input!");
                    return;
                }
            };

            match find_item_by_id(&depository_books, book_id) {
                Some(found_book) => {
                    selected_book = found_book.clone();
                },
                None => {
                    println!("No book found with ID {}", selected_book_id);
                    return;
                }
            }

            println!("Enter borrowing reader name:");
            stdin.read_line(&mut selected_reader_name).unwrap();
            selected_reader_name = selected_reader_name.trim().to_string();

            borrow_item_for_reader(&mut readers, selected_book, selected_reader_name, &mut depository_books);
        },
        Command::BorrowMagazineForUser => {
            let mut selected_magazine_id = String::new();
            let selected_magazine: Magazine;

            let mut selected_reader_name = String::new();

            let stdin = io::stdin();
            list_items(&depository_magazines);

            println!("Select book to borrow by index:");
            stdin.read_line(&mut selected_magazine_id).unwrap();

            let magazine_id = match selected_magazine_id.trim().parse::<usize>() {
                Ok(num) => num - 1,
                Err(_) => {
                    eprintln!("Invalid input!");
                    return;
                }
            };

            match find_item_by_id(&depository_magazines, magazine_id) {
                Some(found_magazine) => {
                    selected_magazine = found_magazine.clone();
                },
                None => {
                    println!("No book found with ID {}", selected_magazine_id);
                    return;
                }
            }

            println!("Enter borrowing reader name:");
            stdin.read_line(&mut selected_reader_name).unwrap();
            selected_reader_name = selected_reader_name.trim().to_string();

            borrow_item_for_reader(&mut readers, selected_magazine, selected_reader_name, &mut depository_magazines);
        }
        Command::AddReader => {
            let mut reader_name = String::new();
            let stdin = io::stdin();

            println!("Enter reader name");
            stdin.read_line(&mut reader_name).unwrap();
            reader_name = reader_name.trim().to_string();

            readers.push(reader::Reader::create(&reader_name));
        }
    }

    match persistence::save_to_file("books_depo.json", &depository_books, &depository_magazines, &readers) {
        Ok(()) => println!("Data saved successfully."),
        Err(err) => eprintln!("Error saving data: {}", err),
    }
}

fn borrow_item_for_reader<T: DepositoryItem>(readers: &mut Vec<Reader>, selected_item: T, selected_reader_name: String, items: &mut Vec<T>) {
    for i in 0..readers.len() {
        if readers[i].name.trim().to_lowercase() == selected_reader_name.to_lowercase() {
            remove_depository_item(items, &selected_item);
            selected_item.borrow_for_reader(&mut readers[i]);
            return;
        }
    }
    println!("No reader was found");
}

fn remove_depository_item<T: DepositoryItem>(items: &mut Vec<T>, item_to_remove: &T) {
    for i in (0..items.len()).rev() {
        if items[i].get_isbn_code() == item_to_remove.get_isbn_code() {
            items.swap_remove(i);
            println!("Found and removed item");
            break;
        }
    }
}

fn list_items<T: DepositoryItem>(items: &Vec<T>) {
    for i in 0..items.len() {
        println!("{}. Title: {} Year {}", i + 1, items[i].get_title(), items[i].get_year())
    }
}

fn find_item_by_id<T: DepositoryItem>(items: &[T], item_id: usize) -> Option<&T> {
    items.get(item_id)
}

//slices, lifetimes
//books.iter().filter(|book| {(*book).id}) 0 cost abstraction;
