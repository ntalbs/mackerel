use std::collections::BTreeMap;

use crate::{Block, Markdown};
use crate::scanner::Token;

pub(crate) struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Markdown {
        let front_matter = self.front_matter();
        let content = self.content();
        Markdown {
            front_matter,
            content,
        }
    }

    fn front_matter(&mut self) -> BTreeMap<String, String> {
        let mut front_matter = BTreeMap::new();
        // begin front matter
        self.consume(Token::Dash, "Invalid YAML delimiter.");
        self.consume(Token::Dash, "Invalid YAML delimiter.");
        self.consume(Token::Dash, "Invalid YAML delimiter.");
        self.consume(Token::Newline, "Invalid YAML delimiter");

        while let Token::Text(text) = self.peek() {
            let kv_pair: Vec<&str> = text.split(':').map(|s| s.trim()).collect();
            front_matter.insert(kv_pair[0].into(), kv_pair[1].into());
            self.advance();
            self.consume(Token::Newline, "Expected newline");
        }

        // end front matter
        self.consume(Token::Dash, "Invalid YAML delimiter.");
        self.consume(Token::Dash, "Invalid YAML delimiter.");
        self.consume(Token::Dash, "Invalid YAML delimiter.");
        self.consume(Token::Newline, "Invalid YAML delimiter");

        return front_matter;
    }

    fn content(&mut self) -> Vec<Block> {
        todo!()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if self.is_at_end() {}
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }

    fn consume(&mut self, token: Token, message: &str) -> &Token {
        if *self.peek() == token {
            self.advance()
        } else {
            panic!("{message}")
        }
    }

    fn is_at_end(&self) -> bool {
        *self.peek() == Token::Eof
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, scanner::Scanner};
    use indoc::indoc;
    use std::collections::BTreeMap;

    #[test]
    fn test() {
        #[rustfmt::skip]
        let markdown_text = indoc!("
            ---
            title: Test
            date: 2025-10-24
            ---
            # heading 1

            First *paragraph.* [Link](https://ntalbs.github.io)
        ");

        let tokens = Scanner::new(markdown_text).scan();
        let mut parser = Parser::new(&tokens);
        let markdown = parser.parse();

        let mut expected: BTreeMap<String, String> = BTreeMap::new();
        expected.insert("title".into(), "Test".into());

        assert_eq!(markdown.front_matter, expected);
    }
}
