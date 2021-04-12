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
    /// Returns Result type object denoting if the books were displayed.

    pub fn display_all_books(&self) -> Result<i32, i32> {
        if self.accession_no.is_empty() {
            warn!("Empty collection");
            return Err(0);
        }
        for index in 0..self.accession_no.len() {
            info!("Book {} details ", index + 1);
            info!("Accession No  - {}", self.accession_no[index]);
            info!("Author  - {}", self.author[index].to_owned());
            info!("Title  - {}", self.title[index].to_owned());
            info!("Issue status  - {}", self.issue[index]);
        }
        Ok(1)
    }

    /// add_book method add a book in Books structure.
    ///
    /// #Arguments
    ///
    /// 'book' - a Book type object containing book to add.
    ///
    /// #Return
    ///
    /// Returns Result type object denoting if the book was added.

    pub fn add_book(&mut self, book: Books) -> Result<i32, i32> {
        if self.accession_no.contains(&book.accession_no[0]) {
            error!("Book already exists");
            return Err(0);
        }
        self.accession_no.extend(book.accession_no.iter().cloned());
        self.author.extend(book.author.iter().cloned());
        self.title.extend(book.title.iter().cloned());
        self.issue.extend(book.issue.iter().cloned());
        info!("Book added");
        Ok(1)
    }

    /// display_books_by_author method displays all books in Books structure that have a particular author.
    ///
    /// #Arguments
    ///
    /// 'author' - a string variable containing author name.
    ///
    /// #Return
    ///
    /// Returns Result type object denoting if the book of author is present.

    pub fn display_books_by_author(&self, author: String) -> Result<i32, i32> {
        if !self.author.contains(&author) {
            warn!("No books by author {}", author);
            return Err(0);
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
        Ok(1)
    }

    /// display_books_by_title method displays all books in Books structure that have a particular title.
    ///
    /// #Arguments
    ///
    /// 'title' - a string variable containing title of book.
    ///
    /// #Return
    ///
    /// Returns Result type object denoting if the book of given title is present.

    pub fn display_books_by_title(&self, title: String) -> Result<i32, i32> {
        if !self.title.contains(&title) {
            warn!("No books by title {}", title);
            return Err(0);
        }
        for index in 0..self.accession_no.len() {
            if self.title[index] == title {
                info!("Accession No  - {}", self.accession_no[index]);
                info!("Author  - {}", self.author[index]);
                info!("Title  - {}", self.title[index]);
                info!("Issue status  - {}", self.issue[index]);
            }
        }
        Ok(1)
    }

    /// count_books method counts all books in Books structure.
    ///
    /// #Arguments
    ///
    /// No arguments.
    ///
    /// #Return
    ///
    /// Returns an Option enum containing the number of books in collection.

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
    /// Returns Result type object denoting if the book was issued or not.

    pub fn issue_book(&mut self, title: String) -> Result<i32, i32> {
        for index in 0..self.accession_no.len() {
            if self.issue[index] == 1 && self.title[index] == title {
                self.issue[index] = 0;
                info!("Book issued");
                return Ok(1);
            }
        }
        error!("Book not available");
        Err(0)
    }
}
