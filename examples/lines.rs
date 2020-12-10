use std::io::{stdin, BufRead};
use umlauts::UmlautsOwned;

fn main() -> std::io::Result<()> {
    for line in stdin().lock().lines() {
        let mut line = line?.into_bytes();
        line.make_utf8_umlauts_to_ascii();
        println!("{}", String::from_utf8(line).unwrap());
    }

    Ok(())
}