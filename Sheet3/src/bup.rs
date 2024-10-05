use std::fmt;
use std::fmt::{format, write, Display, Formatter};

pub struct Book {
    name: String,
    code: String,
    year: i16,
    author: String,
    publisher: String,
}
pub struct Article {
    name: String,
    code: String,
    year: i16,
    orchid: String,
}
pub struct Magazine {
    name: String,
    code: String,
    year: i16,
    number: i32,
    month: i8,
}

pub struct Bup {
    books: Vec<Book>,
    articles: Vec<Article>,
    magazines: Vec<Magazine>,
}

impl Book {
    fn new(name: &str, code: &str, year: i16, author: &str, publisher: &str) -> Self {
        Self { name: String::from(name), code: String::from(code), year, author: String::from(author), publisher: String::from(publisher) }
    }
}

impl Magazine {
    fn new(name: &str, code: &str, year: i16, number: i32, month: i8) -> Self {
        Self { name: String::from(name), code: String::from(code), year, number, month }
    }
}

impl Article {
    fn new(name: &str, code: &str, year: i16, orchid: &str) -> Self {
        Self { name: String::from(name), code: String::from(code), year, orchid: String::from(orchid) }
    }
}

impl Bup {
    pub fn new() -> Self {
        Self { books: Vec::new(), articles: Vec::new(), magazines: Vec::new() }
    }

    pub fn add_book(&mut self, name: &str, code: &str, year: i16, author: &str, publisher: &str) {
        self.books.push(Book::new(name, code, year, author, publisher));
    }

    pub fn add_magazine(&mut self, name: &str, code: &str, year: i16, number: i32, month: i8) {
        self.magazines.push(Magazine::new(name, code, year, number, month));
    }

    pub fn add_article(&mut self, name: &str, code: &str, year: i16, orchid: &str) {
        self.articles.push(Article::new(name, code, year, orchid))
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write! {f, "Book: {} {} {} {} {}", self.name, self.code, self.year, self.author, self.publisher}
    }
}

impl fmt::Display for Magazine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write! {f, "Magazine: {} {} {} {} {}", self.name, self.code, self.year, self.number, self.month}
    }
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write! {f, "Article: {} {} {} {}", self.name, self.code, self.year, self.orchid}
    }
}

impl fmt::Display for Bup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::from("Books:\n");
        for book in &self.books {
            s.push_str(&format!("{}\n", book));
        }
        for book in &self.articles {
            s.push_str(&format!("{}\n", book));
        }
        for book in &self.magazines {
            s.push_str(&format!("{}\n", book));
        }
        write!(f, "{}", s)
    }
}
