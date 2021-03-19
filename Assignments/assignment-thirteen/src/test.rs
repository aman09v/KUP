#[cfg(test)]
mod tests {
    use crate::menu::library::Books;
    #[test]
    fn display_all_books_success() {
        env_logger::init();
        let books_collection = Books {
            accession_no: vec![1, 2, 3, 4, 5],
            author: vec![
                "Leo Tolstoy".to_owned(),
                "Toni Morrison".to_owned(),
                "James Joyce".to_owned(),
                "Carlos Ruiz Zafon".to_owned(),
                "J.R.R. Tolkien".to_owned(),
            ],
            title: vec![
                "War and Peace".to_owned(),
                "Song of Solomon".to_owned(),
                "Ulysses".to_owned(),
                "The Shadow of the Wind".to_owned(),
                "The Lord of the Rings".to_owned(),
            ],
            issue: vec![1, 1, 1, 1, 1],
        };
        assert_eq!(books_collection.display_all_books(), Ok(1));
    }
    #[test]
    fn display_all_books_fail() {
        let books_collection = Books {
            accession_no: vec![],
            author: vec![],
            title: vec![],
            issue: vec![],
        };
        assert_eq!(books_collection.display_all_books(), Err(0));
    }
    #[test]
    fn add_book_success() {
        let mut books_collection = Books {
            accession_no: vec![1, 2, 3, 4, 5],
            author: vec![
                "Leo Tolstoy".to_owned(),
                "Toni Morrison".to_owned(),
                "James Joyce".to_owned(),
                "Carlos Ruiz Zafon".to_owned(),
                "J.R.R. Tolkien".to_owned(),
            ],
            title: vec![
                "War and Peace".to_owned(),
                "Song of Solomon".to_owned(),
                "Ulysses".to_owned(),
                "The Shadow of the Wind".to_owned(),
                "The Lord of the Rings".to_owned(),
            ],
            issue: vec![1, 1, 1, 1, 1],
        };
        let book = Books {
            accession_no: vec![6],
            author: vec!["Salman Rushdie".to_owned()],
            title: vec!["The Satanic Verses".to_owned()],
            issue: vec![1],
        };
        assert_eq!(books_collection.add_book(book), Ok(1));
    }
    #[test]
    fn add_book_fail() {
        let mut books_collection = Books {
            accession_no: vec![1, 2, 3, 4, 5],
            author: vec![
                "Leo Tolstoy".to_owned(),
                "Toni Morrison".to_owned(),
                "James Joyce".to_owned(),
                "Carlos Ruiz Zafon".to_owned(),
                "J.R.R. Tolkien".to_owned(),
            ],
            title: vec![
                "War and Peace".to_owned(),
                "Song of Solomon".to_owned(),
                "Ulysses".to_owned(),
                "The Shadow of the Wind".to_owned(),
                "The Lord of the Rings".to_owned(),
            ],
            issue: vec![1, 1, 1, 1, 1],
        };
        let book = Books {
            accession_no: vec![2],
            author: vec!["Toni Morrison".to_owned()],
            title: vec!["Song of Solomon".to_owned()],
            issue: vec![1],
        };
        assert_eq!(books_collection.add_book(book), Err(0));
    }
    #[test]
    fn display_book_by_author_success() {
        let books_collection = Books {
            accession_no: vec![1, 2, 3, 4, 5, 6],
            author: vec![
                "Leo Tolstoy".to_owned(),
                "Leo Tolstoy".to_owned(),
                "Toni Morrison".to_owned(),
                "James Joyce".to_owned(),
                "Carlos Ruiz Zafon".to_owned(),
                "J.R.R. Tolkien".to_owned(),
            ],
            title: vec![
                "War and Peace".to_owned(),
                "Anna Karenina".to_owned(),
                "Song of Solomon".to_owned(),
                "Ulysses".to_owned(),
                "The Shadow of the Wind".to_owned(),
                "The Lord of the Rings".to_owned(),
            ],
            issue: vec![1, 1, 1, 1, 1, 1],
        };
        assert_eq!(
            books_collection.display_books_by_author("Leo Tolstoy".to_owned()),
            Ok(1)
        );
    }
    #[test]
    fn display_book_by_author_fail() {
        let books_collection = Books {
            accession_no: vec![1, 2, 3, 4, 5, 6],
            author: vec![
                "Leo Tolstoy".to_owned(),
                "Leo Tolstoy".to_owned(),
                "Toni Morrison".to_owned(),
                "James Joyce".to_owned(),
                "Carlos Ruiz Zafon".to_owned(),
                "J.R.R. Tolkien".to_owned(),
            ],
            title: vec![
                "War and Peace".to_owned(),
                "Anna Karenina".to_owned(),
                "Song of Solomon".to_owned(),
                "Ulysses".to_owned(),
                "The Shadow of the Wind".to_owned(),
                "The Lord of the Rings".to_owned(),
            ],
            issue: vec![1, 1, 1, 1, 1, 1],
        };
        assert_eq!(
            books_collection.display_books_by_author("AMAN VERMA".to_owned()),
            Err(0)
        );
    }
    #[test]
    fn display_book_by_title_success() {
        let books_collection = Books {
            accession_no: vec![1, 2, 3, 4, 5, 6],
            author: vec![
                "Leo Tolstoy".to_owned(),
                "Leo Tolstoy".to_owned(),
                "Toni Morrison".to_owned(),
                "James Joyce".to_owned(),
                "Carlos Ruiz Zafon".to_owned(),
                "J.R.R. Tolkien".to_owned(),
            ],
            title: vec![
                "War and Peace".to_owned(),
                "Anna Karenina".to_owned(),
                "Song of Solomon".to_owned(),
                "Ulysses".to_owned(),
                "The Shadow of the Wind".to_owned(),
                "The Lord of the Rings".to_owned(),
            ],
            issue: vec![1, 1, 1, 1, 1, 1],
        };
        assert_eq!(
            books_collection.display_books_by_title("Song of Solomon".to_owned()),
            Ok(1)
        );
    }
    #[test]
    fn display_book_by_title_fail() {
        let books_collection = Books {
            accession_no: vec![1, 2, 3, 4, 5, 6],
            author: vec![
                "Leo Tolstoy".to_owned(),
                "Leo Tolstoy".to_owned(),
                "Toni Morrison".to_owned(),
                "James Joyce".to_owned(),
                "Carlos Ruiz Zafon".to_owned(),
                "J.R.R. Tolkien".to_owned(),
            ],
            title: vec![
                "War and Peace".to_owned(),
                "Anna Karenina".to_owned(),
                "Song of Solomon".to_owned(),
                "Ulysses".to_owned(),
                "The Shadow of the Wind".to_owned(),
                "The Lord of the Rings".to_owned(),
            ],
            issue: vec![1, 1, 1, 1, 1, 1],
        };
        assert_eq!(
            books_collection.display_books_by_title("Rich Dad poor Dad".to_owned()),
            Err(0)
        );
    }
    #[test]
    fn count_books_success() {
        let books_collection = Books {
            accession_no: vec![1, 2, 3, 4, 5, 6],
            author: vec![
                "Leo Tolstoy".to_owned(),
                "Leo Tolstoy".to_owned(),
                "Toni Morrison".to_owned(),
                "James Joyce".to_owned(),
                "Carlos Ruiz Zafon".to_owned(),
                "J.R.R. Tolkien".to_owned(),
            ],
            title: vec![
                "War and Peace".to_owned(),
                "Anna Karenina".to_owned(),
                "Song of Solomon".to_owned(),
                "Ulysses".to_owned(),
                "The Shadow of the Wind".to_owned(),
                "The Lord of the Rings".to_owned(),
            ],
            issue: vec![1, 0, 1, 0, 1, 1],
        };
        assert_eq!(books_collection.count_books(), Some(4));
    }
    #[test]
    fn count_books_fail() {
        let books_collection = Books {
            accession_no: vec![],
            author: vec![],
            title: vec![],
            issue: vec![],
        };
        assert_eq!(books_collection.count_books(), None);
    }
    #[test]
    fn issue_book_success() {
        let mut books_collection = Books {
            accession_no: vec![1, 2, 3, 4, 5, 6],
            author: vec![
                "Leo Tolstoy".to_owned(),
                "Leo Tolstoy".to_owned(),
                "Toni Morrison".to_owned(),
                "James Joyce".to_owned(),
                "Carlos Ruiz Zafon".to_owned(),
                "J.R.R. Tolkien".to_owned(),
            ],
            title: vec![
                "War and Peace".to_owned(),
                "Anna Karenina".to_owned(),
                "Song of Solomon".to_owned(),
                "Ulysses".to_owned(),
                "The Shadow of the Wind".to_owned(),
                "The Lord of the Rings".to_owned(),
            ],
            issue: vec![1, 1, 1, 0, 1, 1],
        };
        assert_eq!(
            books_collection.issue_book("Anna Karenina".to_owned()),
            Ok(1)
        );
    }
    #[test]
    fn issue_book_fail() {
        let mut books_collection = Books {
            accession_no: vec![1, 2, 3, 4, 5, 6],
            author: vec![
                "Leo Tolstoy".to_owned(),
                "Leo Tolstoy".to_owned(),
                "Toni Morrison".to_owned(),
                "James Joyce".to_owned(),
                "Carlos Ruiz Zafon".to_owned(),
                "J.R.R. Tolkien".to_owned(),
            ],
            title: vec![
                "War and Peace".to_owned(),
                "Anna Karenina".to_owned(),
                "Song of Solomon".to_owned(),
                "Ulysses".to_owned(),
                "The Shadow of the Wind".to_owned(),
                "The Lord of the Rings".to_owned(),
            ],
            issue: vec![1, 0, 1, 0, 1, 1],
        };
        assert_eq!(
            books_collection.issue_book("Anna Karenina".to_owned()),
            Err(0)
        );
    }
}
