use std::io::{self, BufReader, Read};

#[derive(Default)]
pub struct FileStats {
    pub bytes: u64,
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

impl FileStats {
    pub fn add(&mut self, other: &FileStats) {
        self.bytes += other.bytes;
        self.lines += other.lines;
        self.words += other.words;
        self.chars += other.chars;
    }
}

pub fn compute_stats<R: Read>(reader: R) -> io::Result<FileStats> {
    let mut reader = BufReader::new(reader);
    let mut buffer = [0; 4096]; // read in 4KB chunks

    let mut bytes: u64 = 0;
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;

    let mut in_word = false;
    let mut leftover = Vec::new();

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }

        bytes += n as u64;

        let mut chunk = leftover.clone();
        chunk.extend_from_slice(&buffer[..n]);

        let (valid_str, valid_up_to) = match std::str::from_utf8(&chunk) {
            Ok(s) => (s, chunk.len()),
            Err(e) => (
                std::str::from_utf8(&chunk[..e.valid_up_to()]).unwrap(),
                e.valid_up_to(),
            ),
        };

        for c in valid_str.chars() {
            chars += 1;

            if c == '\n' {
                lines += 1;
            }

            if c.is_whitespace() {
                in_word = false;
            } else if !in_word {
                words += 1;
                in_word = true;
            }
        }

        leftover.clear();
        leftover.extend_from_slice(&chunk[valid_up_to..]);
    }

    return Ok(FileStats {
        bytes,
        lines,
        words,
        chars,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_with_challenge_file() {
        let filename = "test.txt".to_string();
        let stats = compute_stats(File::open(filename).unwrap()).unwrap();

        assert_eq!(stats.bytes, 342190);
        assert_eq!(stats.lines, 7145);
        assert_eq!(stats.words, 58164);
        assert_eq!(stats.chars, 339292);
    }
}
