use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Markdown {
    pub(crate) front_matter: BTreeMap<String, String>,
    pub(crate) content: Vec<Block>,
}

#[derive(Debug)]
pub(crate) enum Block {
    Heading { level: u8, runs: Vec<Run> },
    Paragraph(Vec<Run>),
    List { ordered: bool, items: Vec<ListItem> },
    Table { thead: TRow, tbody: Vec<TRow> },
    Code,
    Quote,
    HorizontalRule,
}

#[derive(Debug)]
pub(crate) struct ListItem {
    runs: Vec<Run>,
    nested: Option<Block>,
}

#[derive(Debug)]
pub(crate) struct TRow {
    pub(crate) heading: bool,
    pub(crate) cells: Vec<TCell>,
}

#[derive(Debug)]
pub(crate) struct TCell(Vec<Run>);

#[derive(Debug)]
pub(crate) enum Run {
    Text(String),
    Bold(Vec<Run>),
    Italic(Vec<Run>),
    Link { url: String, inner: Vec<Run> },
    Image { url: String, alt: String },
    Code(String),
    LineBreak,
}
