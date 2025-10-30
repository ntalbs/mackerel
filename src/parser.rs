use core::panic;
use std::collections::BTreeMap;

use crate::scanner::Token;
use crate::{Block, Markdown, Run};

pub(crate) struct Parser<'a> {
    tokens: &'a [Token],
    current_pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            current_pos: 0,
        }
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
        let message = "Invalid YAML delimiter.";

        // begin front matter
        match self.consume(Token::Dash(3), message) {
            Ok(_) => {}
            Err(_) => return front_matter,
        }
        self.consume(Token::Newline(1), message);

        while let Token::Text(text) = self.current_token() {
            let kv_pair: Vec<&str> = text.split(':').map(|s| s.trim()).collect();
            front_matter.insert(kv_pair[0].into(), kv_pair[1].into());
            self.advance();
            self.consume(Token::Newline(1), "Expected newline");
        }

        // end front matter
        self.consume(Token::Dash(3), message);
        self.consume(Token::Newline(1), message);

        front_matter
    }

    fn content(&mut self) -> Vec<Block> {
        let mut blocks = Vec::new();
        loop {
            let token = self.current_token();
            if *token == Token::Eof {
                break;
            }
            if *token == Token::Newline(2) {
                self.advance();
                continue;
            }
            blocks.push(self.block());
        }
        blocks
    }

    fn block(&mut self) -> Block {
        match self.current_token() {
            Token::Hash(n) => self.heading(*n),
            Token::Star(1) | Token::Dash(1) | Token::Plus => self.list(),
            Token::RightAngleBracket => self.blockquote(),
            _ => self.paragraph(),
        }
    }

    fn heading(&mut self, level: u8) -> Block {
        self.advance();
        self.consume(Token::Whitespace, "expected space");
        let runs = self.runs(Token::Newline(1));
        Block::Heading { level, runs }
    }

    fn list(&mut self) -> Block {
        todo!()
    }

    fn blockquote(&mut self) -> Block {
        todo!()
    }

    fn paragraph(&mut self) -> Block {
        let runs = self.runs(Token::Newline(2));
        Block::Paragraph(runs)
    }

    fn runs(&mut self, until: Token) -> Vec<Run> {
        let mut runs = Vec::new();
        loop {
            let token = self.advance();
            if *token == until {
                break;
            }
            match token {
                Token::Newline(2) => break,
                Token::Text(t) => runs.push(Run::Text(t.to_owned())),

                Token::Star(1) => runs.push(Run::Italic(self.runs(Token::Star(1)))),
                Token::Star(2) => runs.push(Run::Bold(self.runs(Token::Star(2)))),
                Token::Star(3) => runs.push(Run::Bold(vec![Run::Italic(self.runs(Token::Star(3)))])),

                Token::LeftBracket => runs.push(self.link()),
                Token::Whitespace => runs.push(Run::Text(" ".into())),
                Token::Eof => break,
                // Token::LeftBracket | Token::RightBracket | Token::LeftParen | Token::RightParen => {}
                _ => {
                    println!(">>>>>>>>");
                    println!("{:?}", self.tokens);
                    println!(">>>>>>>>");
                    todo!()
                }
            }
        }
        runs
    }

    fn link(&mut self) -> Run {
        let runs = self.runs(Token::RightBracket);
        self.consume(Token::LeftParen, "expected (");
        let Token::Text(url) = self.advance() else {
            panic!("expected Token::Text(url)");
        };
        let url = url.clone();
        self.consume(Token::RightParen, "expected )");

        Run::Link {
            inner: runs,
            url: url.clone(),
        }
    }

    fn token_at(&self, offset: usize) -> &Token {
        if self.current_pos + offset < self.tokens.len() {
            &self.tokens[self.current_pos + offset]
        } else {
            &Token::Eof
        }
    }

    fn current_token(&self) -> &Token {
        self.token_at(0)
    }

    // fn next_token(&self) -> &Token {
    //     if ! self.current_pos >= self.tokens.len() {
    //         &Token::Eof
    //     } else {
    //         &self.tokens[self.current_pos + 1]
    //     }
    // }

    fn prev_token(&self) -> &Token {
        // assums that this method never called at the beginning
        &self.tokens[self.current_pos - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current_pos += 1;
            self.prev_token()
        } else {
            &Token::Eof
        }
    }

    fn consume(&mut self, token: Token, message: &'a str) -> Result<&Token, &'a str> {
        fn matches(t1: &Token, t2: &Token) -> bool {
            t1 == t2 || matches!(t1, Token::Text(_)) && matches!(t2, Token::Text(_))
        }

        if matches(&token, self.current_token()) {
            self.advance();
            Ok(self.current_token())
        } else {
            Err(message)
        }
    }

    fn is_at_end(&self) -> bool {
        *self.current_token() == Token::Eof
    }
}

#[cfg(test)]
mod tests {
    use crate::{Block, Run, parser::Parser, scanner::Scanner};
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

            ***bold italic***, **bold**

        ");

        let tokens = Scanner::new(markdown_text).scan();
        for (i, t) in tokens.iter().enumerate() {
            println!("{i}:{t:?}");
        }
        let mut parser = Parser::new(&tokens);
        let markdown = parser.parse();

        let mut expected_front_matter: BTreeMap<String, String> = BTreeMap::new();
        expected_front_matter.insert("title".into(), "Test".into());
        expected_front_matter.insert("date".into(), "2025-10-24".into());

        assert_eq!(markdown.front_matter, expected_front_matter);

        let expected_content = vec![
            Block::Heading {
                level: 1,
                runs: vec![Run::Text("heading 1".into())],
            },
            Block::Paragraph(vec![
                Run::Text("First ".into()),
                Run::Italic(vec![Run::Text("paragraph.".into())]),
                Run::Text(" ".into()),
                Run::Link {
                    inner: vec![Run::Text("Link".into())],
                    url: "https://ntalbs.github.io".into(),
                },
            ]),
            Block::Paragraph(vec![
                Run::Bold(vec![Run::Italic(vec![Run::Text("bold italic".into())])]),
                Run::Text(", ".into()),
                Run::Bold(vec![Run::Text("bold".into())]),
            ]),
        ];

        assert_eq!(markdown.content, expected_content);
    }
}
