use std::fs::File;
use std::io::{self, BufReader};
mod elfreader;

fn main() -> io::Result<()> {
    let f = File::open("inputs/1.txt")?;
    let r = elfreader::ElfReader::new(BufReader::new(f));

    let mut all: Vec<u32> = r.map(|x| x.iter().sum()).collect();
    all.sort();
    all.reverse();
    let top3_total = all.iter().take(3).copied();


    //println!("{:?}", top3_total.collect::<Vec<u32>>());
    println!("{:?}", top3_total.sum::<u32>());
    Ok(())
}