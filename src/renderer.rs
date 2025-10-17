use crate::{Block, Run};

fn render_runs(runs: &[Run]) -> String {
    let mut s = String::new();
    for r in runs {
        s.push_str(&r.render());
    }
    s
}

trait Render {
    fn render(&self) -> String;
}

impl Render for Run {
    fn render(&self) -> String {
        match self {
            Run::Text(s) => s.clone(),
            Run::Bold(runs) => format!("<b>{}</b>", render_runs(runs)),
            Run::Italic(runs) => format!("<i>{}</i>", render_runs(runs)),
            Run::Link { inner, url } => format!("<a href=\"{url}\">{}</a>", render_runs(inner)),
            Run::Image { url, alt } => format!("<img src=\"{url}\" alt=\"{alt}\">"),
            Run::Code(code) => format!("<code>{code}</code>"),
            Run::LineBreak => "<br>".into(),
        }
    }
}

impl Render for Block {
    fn render(&self) -> String {
        match self {
            Block::Heading { level, runs } => {
                format!("<h{level}>{}</h{level}", render_runs(runs))
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Run, renderer::Render};
    use p_test::p_test;

    /// macros for runs

    macro_rules! t {
        ($input:literal) => {
            Run::Text($input.into())
        };
    }

    macro_rules! b {
        ( $($r:expr), + ) => {
            Run::Bold(vec![ $($r), + ])
        }
    }

    macro_rules! i {
        ( $($r:expr), + ) => {
            Run::Italic(vec![ $($r), + ])
        }
    }

    macro_rules! ln {
        ($url:literal, $inner:expr) => {
            Run::Link {
                url: $url.into(),
                inner: vec![$inner.into()],
            }
        };
        ($url:literal, ($inner:expr), +) => {
            Run::Link {
                url: $url.into(),
                inner: vec![$($inner.into()), +],
            }
        };
    }

    macro_rules! img {
        ($url:literal, $alt:literal) => {
            Run::Image {
                url: $url.into(),
                alt: $alt.into(),
            }
        };
    }

    macro_rules! code {
        ($input:literal) => {
            Run::Code($input.into())
        };
    }

    macro_rules! lineBreak {
        () => {
            Run::LineBreak
        };
    }

    #[p_test(
        (t!("text"), "text"),
        (b!(t!("bold")), "<b>bold</b>"),
        (i!(t!("italic")), "<i>italic</i>"),
        (ln!("https://local.host", t!("target")), "<a href=\"https://local.host\">target</a>"),
        (img!("https://local.host/img", "alt text"), "<img src=\"https://local.host/img\" alt=\"alt text\">"),
        (code!("code"), "<code>code</code>"),
        (lineBreak!(), "<br>"),
    )]
    fn test_render_run(run: Run, expected: &str) {
        assert_eq!(run.render(), expected);
    }
}
