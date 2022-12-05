use std::fs::File;
use std::io::{self, BufRead, BufReader};
mod rockpaperscissors;

fn main() -> io::Result<()> {
    let f = File::open("inputs/2.txt")?;
    let r = BufReader::new(f);
    let mut total_score: usize = 0;

    for l in r.lines() {
        if let Ok(line) = l {
            let letters = line.splitn(2, " ").take(2).collect::<Vec<&str>>();
            let their_choice: rockpaperscissors::RPS = letters[0].into();
            let your_choice: rockpaperscissors::RPS = letters[1].into();

            total_score += your_choice.score() + your_choice.result(&their_choice);
        }
    }

    println!("{total_score}");
    Ok(())
}
