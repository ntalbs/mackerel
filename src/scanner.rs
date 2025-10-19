use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    Newline(u8),
    Whitespace,
    Hash(u8),
    Star(u8),
    Plus,
    Underscore,
    Exclamination,
    Tilde,
    Backtick(u8),
    Dash(u8),
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
        tokens.push(Token::Eof);
        tokens
    }

    fn next_token(&mut self) -> Token {
        match self.advance() {
            Some('\n') | Some('#') | Some('*') | Some('`') | Some('-') => {
                self.repeatable(self.current_char.unwrap())
            }
            Some(' ') => Token::Whitespace,
            Some('+') => Token::Plus,
            Some('_') => Token::Underscore,
            Some('!') => Token::Exclamination,
            Some('~') => Token::Tilde,
            Some('|') => Token::VerticalLine,
            Some('^') => Token::Caret,
            Some('(') => Token::LeftParen,
            Some(')') => Token::RightParen,
            Some('[') => Token::LeftBracket,
            Some(']') => Token::RightBracket,
            Some('>') => Token::RightAngleBracket,
            Some(_) => self.text(),
            None => Token::Eof,
        }
    }

    fn repeatable(&mut self, ch: char) -> Token {
        let mut cnt = 1;
        while self.peek() == Some(&ch) {
            cnt += 1;
            self.advance();
        }
        match ch {
            '\n' => Token::Newline(cnt),
            '#' => Token::Hash(cnt),
            '*' => Token::Star(cnt),
            '`' => Token::Backtick(cnt),
            '-' => Token::Dash(cnt),
            _ => panic!("Invalid repeated char."),
        }
    }

    fn text(&mut self) -> Token {
        fn is_text_breaker(ch: char) -> bool {
            matches!(ch, '*' | '_' | '[' | ']' | '(' | ')' | '`' | '\n')
        }

        let mut text = String::new();
        loop {
            let Some(current_ch) = self.current_char else {
                break;
            };
            text.push(current_ch);
            
            let Some(&next_ch) = self.peek() else {
                break;
            };

            if is_text_breaker(next_ch) {
                break;
            }

            self.advance();
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
        #[rustfmt::skip]
        let markdown_text = indoc!("
            ---
            title: Test
            date: 2025-10-24
            ---
            # heading 1

            First *paragraph.* [Link](https://ntalbs.github.io)
        ");

        let mut scanner = Scanner::new(markdown_text);
        let tokens = scanner.scan();
        let expected_tokens = vec![
            Token::Dash(3),
            Token::Newline(1),
            Token::Text("title: Test".into()),
            Token::Newline(1),
            Token::Text("date: 2025-10-24".into()),
            Token::Newline(1),
            Token::Dash(3),
            Token::Newline(1),
            Token::Hash(1),
            Token::Whitespace,
            Token::Text("heading 1".into()),
            Token::Newline(2),
            Token::Text("First ".into()),
            Token::Star(1),
            Token::Text("paragraph.".into()),
            Token::Star(1),
            Token::Whitespace,
            Token::LeftBracket,
            Token::Text("Link".into()),
            Token::RightBracket,
            Token::LeftParen,
            Token::Text("https://ntalbs.github.io".into()),
            Token::RightParen,
            Token::Newline(1),
            Token::Eof,
        ];

        assert_eq!(tokens, expected_tokens);
    }
}
