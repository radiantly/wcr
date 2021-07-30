use std::io::{self, Error, Read};

fn main() -> Result<(), Error> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    let count = wcr::parse(&buf);
    println!(
        "Words: {}\nLines: {}\nBytes: {}",
        count.words, count.lines, count.chars
    );
    Ok(())
}
