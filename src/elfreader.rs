use std::io::BufRead;

pub struct ElfReader<T: BufRead> {
    reader: T,
    buffer: String,
}

impl<T: BufRead> ElfReader<T> {
    pub fn new(r: T) -> ElfReader<T> {
        ElfReader {
            reader: r,
            buffer: String::with_capacity(10),
        }
    }
}

impl<T: BufRead> Iterator for ElfReader<T> {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut v = vec![];
        self.buffer.clear();

        while let Ok(z) = self.reader.read_line(&mut self.buffer) {
            if z == 0 {
                return None;
            }
            if z == 1 {
                return Some(v);
            }

            let parsed = self.buffer.trim_end().parse::<u32>();
            self.buffer.clear();
            if let Ok(next_int) = parsed {
                v.push(next_int);
            } else {
                break;
            }
        }
        Some(v)
    }
}

#[cfg(test)]
mod test {
    use std::io::BufRead;
    use std::io::BufReader;

    #[test]
    fn parse_int_from_string() {
        assert_eq!(String::from("1234").parse::<u32>(), Ok(1234u32));
    }

    #[test]
    fn blank_line_with_read_line() {
        let mut cursor = std::io::Cursor::new(b"foo\n\nbar");
        let mut buf = String::new();

        let num_bytes = cursor
            .read_line(&mut buf)
            .expect("reading from cursor won't fail");
        assert_eq!(num_bytes, 4);
        assert_eq!(buf, "foo\n");
        buf.clear();

        let num_bytes = cursor
            .read_line(&mut buf)
            .expect("reading from cursor won't fail");
        assert_eq!(num_bytes, 1);
        assert_eq!(buf, "\n");
    }
}
