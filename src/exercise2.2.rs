use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::rockpaperscissors::{RPS, WinDrawLose};

mod rockpaperscissors;

fn main() -> io::Result<()> {
    let f = File::open("inputs/2.txt")?;
    let r = BufReader::new(f);
    let mut total_score: usize = 0;

    for l in r.lines() {
        if let Ok(line) = l {
            let letters = line.splitn(2, " ").take(2).collect::<Vec<&str>>();
            let their_choice: RPS = letters[0].into();
            let desired_result: WinDrawLose = letters[1].into();
            let your_choice: RPS = their_choice.choose_for_result(&desired_result);
            total_score += your_choice.score() + desired_result.score();
        }
    }

    println!("{total_score}");
    Ok(())
}
