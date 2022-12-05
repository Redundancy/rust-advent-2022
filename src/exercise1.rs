use std::fs::File;
use std::io::{self, BufReader};
mod elfreader;

fn main() -> io::Result<()> {
    let f = File::open("inputs/1.txt")?;

    let r = elfreader::ElfReader::new(BufReader::new(f));

    let largest = r.map(|x| x.iter().sum()).max();

    println!("{largest}");
    Ok(())
}