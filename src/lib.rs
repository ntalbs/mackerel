mod markdown;
mod renderer;
mod scanner;

use markdown::Markdown;
use scanner::{Scanner, Token};

pub struct Markerel<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Markerel<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            scanner: Scanner::new(input),
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        self.scanner.scan()
    }

    pub fn parse(&mut self) -> Markdown {
        todo!()
    }
}
