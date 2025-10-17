mod markdown;
mod parser;
mod renderer;
mod scanner;

use markdown::Markdown;
use scanner::Scanner;
use parser::Parser;

pub fn parse_markdown<'a>(input: &'a str) -> Markdown {
    let tokens = Scanner::new(input).scan();
    let markdown = Parser::new(&tokens).parse();
    markdown
}
