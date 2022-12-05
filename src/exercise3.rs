use std::fs::File;
use std::io::{self, BufRead, BufReader};
mod rockpaperscissors;

fn main() -> io::Result<()> {
    let f = File::open("inputs/3.txt")?;
    let r = BufReader::new(f);
    let mut total: usize = 0;

    for l in r.lines() {
        if let Ok(line) = l {
            let c = find_common(split_in_half(line.as_str()));
            total += priority(&c)
        }
    }

    println!("{total}");
    Ok(())
}

fn priority(c: &char) -> usize {
    return match c {
        'a'..='z' => (*c as u8) - ('a' as u8) + 1,
        'A'..='Z' => (*c as u8) - ('A' as u8) + 27,
        _ => unreachable!("uhoh"),
    } as usize;
}

fn split_in_half(s: &str) -> (&str, &str) {
    s.split_at(s.len() / 2)
}

fn find_common(s: (&str, &str)) -> char {
    let set1: String = s.1.chars().collect();
    for c in s.0.chars() {
        if set1.contains(c) {
            return c
        }
    }
    'a'
}

#[cfg(test)]
mod test {
    #[test]
    fn test_split_in_half() {
        let s = String::from("vJrwpWtwJgWrhcsFMMfFFhFp");
        let (compartment1, compartment2) = crate::split_in_half(s.as_str());
        assert_eq!(compartment1, "vJrwpWtwJgWr");
        assert_eq!(compartment2, "hcsFMMfFFhFp");
    }
    #[test]
    fn test_find_common() {
        assert_eq!(
            crate::find_common(("vJrwpWtwJgWr", "hcsFMMfFFhFp")),
            'p',
        )
    }

    #[test]
    fn test_priority() {
        assert_eq!(crate::priority(&'A'), 27);
        assert_eq!(crate::priority(&'a'), 1);
        assert_eq!(crate::priority(&'p'), 16);
    }
}
