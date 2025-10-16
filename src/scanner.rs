use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    Newline,
    Whitespace,
    Hash,
    Star,
    Plus,
    Underscore,
    Exclamination,
    Tilde,
    Backtick,
    Dash,
    VerticalLine,
    Caret,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    RightAngleBracket,
    Eof,
}

pub struct Scanner<'a> {
    iter: Peekable<Chars<'a>>,
    current_char: Option<char>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars().peekable(),
            current_char: Option::None,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            tokens.push(self.next_token());
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        self.current_char = self.advance();
        match self.current_char {
            Some('\n') => Token::Newline,
            Some(' ') => Token::Whitespace,
            Some('#') => Token::Hash,
            Some('*') => Token::Star,
            Some('+') => Token::Plus,
            Some('_') => Token::Underscore,
            Some('!') => Token::Exclamination,
            Some('~') => Token::Tilde,
            Some('`') => Token::Backtick,
            Some('-') => Token::Dash,
            Some('|') => Token::VerticalLine,
            Some('^') => Token::Caret,
            Some('(') => Token::LeftParen,
            Some(')') => Token::RightParen,
            Some('[') => Token::LeftBracket,
            Some(']') => Token::RightBracket,
            Some('>') => Token::RightAngleBracket,
            Some(c) => self.text(),
            None => Token::Eof,
        }
    }

    fn text(&mut self) -> Token {
        fn is_text_breaker(ch: Option<&char>) -> bool {
            if let Some(ch) = ch {
                matches!(ch, '*' | '_' | '[' | ']' | '(' | ')' | '\n')
            } else {
                false
            }
        }

        let mut text = String::new();
        loop {
            text.push(self.current_char.unwrap());
            if is_text_breaker(self.peek()) {
                break;
            }

            if let Some('\n') = self.advance() {
                break;
            }
        }
        Token::Text(text)
    }

    fn advance(&mut self) -> Option<char> {
        self.current_char = self.iter.next();
        self.current_char
    }

    fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::scanner::{Scanner, Token};

    #[test]
    fn scanner_test() {
        let markdown_text = indoc!("
            # heading 1

            First *paragraph.* [Link](https://ntalbs.github.io)
        ");

        let mut scanner = Scanner::new(markdown_text);
        let tokens = scanner.scan();
        let expected_tokens = vec![
            Token::Hash,
            Token::Whitespace,
            Token::Text("heading 1".into()),
            Token::Newline,
            Token::Newline,
            Token::Text("First ".into()),
            Token::Star,
            Token::Text("paragraph.".into()),
            Token::Star,
            Token::Whitespace,
            Token::LeftBracket,
            Token::Text("Link".into()),
            Token::RightBracket,
            Token::LeftParen,
            Token::Text("https://ntalbs.github.io".into()),
            Token::RightParen,
            Token::Newline,
        ];

        assert_eq!(tokens, expected_tokens);
    }
}
