use log::error;
use log::info;
use log::warn;

pub struct Books {
    pub accession_no: Vec<i32>,
    pub author: Vec<String>,
    pub title: Vec<String>,
    pub issue: Vec<i32>,
}

impl Books {
    /// display_all_books method displays all books in Books structure.
    ///
    /// #Arguments
    ///
    /// No Arguments.
    ///
    /// #Return
    ///
    /// No return value.

    pub fn display_all_books(&self) {
        env_logger::init();
        if self.accession_no.is_empty() {
            warn!("Empty collection");
            return;
        }
        for index in 0..self.accession_no.len() {
            info!("Book {} details ", index + 1);
            info!("Accession No  - {}", self.accession_no[index]);
            info!("Author  - {}", self.author[index].to_owned());
            info!("Title  - {}", self.title[index].to_owned());
            info!("Issue status  - {}", self.issue[index]);
        }
    }

    /// add_book method add a book in Books structure.
    ///
    /// #Arguments
    ///
    /// 'book' - a Book type object containing book to add.
    ///
    /// #Return
    ///
    /// No return value.

    pub fn add_book(&mut self, book: Books) {
        if self.accession_no.contains(&book.accession_no[0]) {
            error!("Book already exists");
            return;
        }
        self.accession_no.extend(book.accession_no.iter().cloned());
        self.author.extend(book.author.iter().cloned());
        self.title.extend(book.title.iter().cloned());
        self.issue.extend(book.issue.iter().cloned());
        info!("Book added");
    }

    /// display_books_by_author method displays all books in Books structure that have a particular author.
    ///
    /// #Arguments
    ///
    /// 'author' - a string variable containing author name.
    ///
    /// #Return
    ///
    /// No return value.

    pub fn display_books_by_author(&self, author: String) {
        if !self.author.contains(&author) {
            warn!("No books by author {}", author);
        }
        for index in 0..self.accession_no.len() {
            if self.author[index] == author {
                info!("Book {} details ", index + 1);
                info!("Accession No  - {}", self.accession_no[index]);
                info!("Author  - {}", self.author[index].to_owned());
                info!("Title  - {}", self.title[index].to_owned());
                info!("Issue status  - {}", self.issue[index]);
            }
        }
    }

    /// display_books_by_title method displays all books in Books structure that have a particular title.
    ///
    /// #Arguments
    ///
    /// 'title' - a string variable containing title of book.
    ///
    /// #Return
    ///
    /// No return value.

    pub fn display_books_by_title(&self, title: String) {
        if !self.title.contains(&title) {
            warn!("No books by title {}", title);
        }
        for index in 0..self.accession_no.len() {
            if self.title[index] == title {
                info!("Accession No  - {}", self.accession_no[index]);
                info!("Author  - {}", self.author[index]);
                info!("Title  - {}", self.title[index]);
                info!("Issue status  - {}", self.issue[index]);
            }
        }
    }

    /// count_books method counts all books in Books structure.
    ///
    /// #Arguments
    ///
    /// No arguments.
    ///
    /// #Return
    ///
    /// an Option enum containing the number of books in collection.

    pub fn count_books(&self) -> Option<i32> {
        let mut count = 0;
        if self.accession_no.is_empty() {
            warn!("Empty collection");
            return None;
        }
        for index in 0..self.accession_no.len() {
            if self.issue[index] == 1 {
                count += 1;
            }
        }
        if count == 0 {
            None
        } else {
            Some(count)
        }
    }

    /// issue_book method issues a book from Books structure.
    ///
    /// #Arguments
    ///
    /// 'title' - a string variable containing title of book to be issued.
    ///
    /// #Return
    ///
    /// No return value.

    pub fn issue_book(&mut self, title: String) {
        let mut found = 0;
        for index in 0..self.accession_no.len() {
            if self.issue[index] == 1 && self.title[index] == title {
                self.issue[index] = 0;
                found = 1;
                info!("Book issued");
                break;
            }
        }
        if found == 0 {
            error!("Book not available");
        }
    }
}
