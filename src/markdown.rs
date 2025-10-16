use std::collections::BTreeMap;

pub(crate) struct Markdown {
    pub(crate) front_matter: BTreeMap<String, String>,
    pub(crate) content: Vec<Block>,
}

pub(crate) enum Block {
    Heading { level: u8, text: String },
    Paragraph(Vec<Run>),
    List { ordered: bool, items: Vec<Run> },
    Table { thead: TRow, tbody: Vec<TRow> },
    CodeBlock,
    BlockQuote,
    HorizontalRule,
}

pub(crate) struct TRow {
    pub(crate) heading: bool,
    pub(crate) cells: Vec<TCell>,
}

pub(crate) struct TCell(Vec<Run>);

pub(crate) enum Run {
    Text(String),
    Bold(Vec<Run>),
    Italic(Vec<Run>),
    Link { url: String, inner: Vec<Run> },
    Image { url: String, alt: String },
    Code(String),
    LineBreak,
}
