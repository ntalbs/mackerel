mod parser;
mod renderer;
mod scanner;

use parser::Parser;
use scanner::Scanner;

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Markdown {
    pub(crate) front_matter: BTreeMap<String, String>,
    pub(crate) content: Vec<Block>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Block {
    Heading { level: u8, runs: Vec<Run> },
    Paragraph(Vec<Run>),
    List { ordered: bool, items: Vec<ListItem> },
    Table { thead: TRow, tbody: Vec<TRow> },
    Code,
    Quote,
    HorizontalRule,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ListItem {
    runs: Vec<Run>,
    nested: Option<Block>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct TRow {
    pub(crate) heading: bool,
    pub(crate) cells: Vec<TCell>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct TCell(Vec<Run>);

#[derive(Debug, PartialEq)]
pub(crate) enum Run {
    Text(String),
    Bold(Vec<Run>),
    Italic(Vec<Run>),
    Link { url: String, inner: Vec<Run> },
    Image { url: String, alt: String },
    Code(String),
    LineBreak,
}

pub fn parse_markdown<'a>(input: &'a str) -> Markdown {
    let tokens = Scanner::new(input).scan();
    Parser::new(&tokens).parse()
}
