use std::str::FromStr;
pub enum Command {
    AddBook,
    ListBooks,
    RemoveBook,
    AddReader,
    BorrowBookForUser,
    AddMagazine,
    BorrowMagazineForUser
}



impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "addbook" => Ok(Command::AddBook),
            "listbooks" => Ok(Command::ListBooks),
            "removebook" => Ok(Command::RemoveBook),
            "adduser" => Ok(Command::AddReader),
            "borrowbook" => Ok(Command::BorrowBookForUser),
            "addmagazine" => Ok(Command::AddMagazine),
            "borrowmagazine" => Ok(Command::BorrowMagazineForUser),
            _ => Err(()),
        }
    }
}