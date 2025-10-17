use std::{env, fs, io};

use mackerel::parse_markdown;

fn main() -> io::Result<()> {
    let files: Vec<String> = env::args().skip(1).collect();

    for f in files {
        println!(">>> {f}");
        let source = fs::read_to_string(f)?;
        let markdown = parse_markdown(&source);
        println!("{markdown:?}");
    }

    Ok(())
}
