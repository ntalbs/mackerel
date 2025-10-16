use mackerel::Markerel;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let files: Vec<String> = env::args().skip(1).collect();

    for f in files {
        println!(">>> {f}");
        let source = fs::read_to_string(f)?;

        let mut mackerel = Markerel::new(&source);
        let tokens = mackerel.scan();

        for t in tokens {
            println!("{t:?}");
        }
    }
    Ok(())
}
