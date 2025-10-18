use std::collections::BTreeMap;

use crate::{Block, Markdown, Run};
use crate::scanner::Token;

pub(crate) struct Parser<'a> {
    tokens: &'a [Token],
    current_pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current_pos: 0 }
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
        let message = "Invalid YAML delimiter.";
        self.consume(Token::Dash, message);
        self.consume(Token::Dash, message);
        self.consume(Token::Dash, message);
        self.consume(Token::Newline, message);

        while let Token::Text(text) = self.current_token() {
            let kv_pair: Vec<&str> = text.split(':').map(|s| s.trim()).collect();
            front_matter.insert(kv_pair[0].into(), kv_pair[1].into());
            self.advance();
            self.consume(Token::Newline, "Expected newline");
        }

        // end front matter
        self.consume(Token::Dash, message);
        self.consume(Token::Dash, message);
        self.consume(Token::Dash, message);
        self.consume(Token::Newline, message);

        front_matter
    }

    fn content(&mut self) -> Vec<Block> {
        let mut blocks = Vec::new();
        loop {
            let token = self.current_token();
            if *token == Token::Eof {
                break;
            }
            if *token == Token::Newline {
                self.advance();
                continue;
            }
            blocks.push(self.block());
        }
        blocks
    }

    fn block(&mut self) -> Block {
        match self.current_token() {
            Token::Hash => self.heading(),
            _ => self.paragraph(),
        }
    }

    fn heading(&mut self) -> Block {
        let mut level = 0;
        while *self.current_token() == Token::Hash {
            self.advance();
            level += 1;
        }

        let runs = self.runs();
        Block::Heading {
            level, 
            runs,
        }
    }

    fn paragraph(&mut self) -> Block {
        let runs = self.runs();
        Block::Paragraph(runs)
    }

    fn runs(&mut self) -> Vec<Run> {
        let mut runs = Vec::new();
        loop {
            match self.advance() {
                Token::Text(t) => runs.push(Run::Text(t.to_owned())),
                Token::Newline => break,

                // temporary implementation
                Token::Star => {}
                Token::Whitespace => {}
                Token::LeftBracket | Token::RightBracket | Token::LeftParen | Token::RightParen => {}


                _ => todo!()
            }
        }
        runs
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current_pos]
    }

    fn next_token(&self) -> &Token {
        if ! self.current_pos >= self.tokens.len() {
            &Token::Eof
        } else {
            &self.tokens[self.current_pos + 1]
        }
    }

    fn prev_token(&self) -> &Token {
        // assums that this method never called at the beginning
        &self.tokens[self.current_pos - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current_pos += 1;
        }
        self.prev_token()
    }

    fn consume(&mut self, token: Token, message: &str) -> &Token {
        fn matches(t1: &Token, t2: &Token) -> bool {
            t1 == t2 || matches!(t1, Token::Text(_)) && matches!(t2, Token::Text(_))
        }

        if matches(&token, self.current_token()) {
            self.advance();
            self.current_token()
        } else {
            panic!("{message}")
        }
    }

    fn is_at_end(&self) -> bool {
        *self.current_token() == Token::Eof
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, scanner::Scanner, Block, Run};
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
            Block::Heading { level: 1, runs: vec![Run::Text("heading 1".into())] },
            Block::Paragraph(vec![Run::Text("First *paragraph.* [Link](https://ntalbs.github.io)".into())])
        ];

        assert_eq!(markdown.content, expected_content);
    }
}
