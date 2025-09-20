use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

struct FileStats {
    bytes: u64,
    lines: usize,
    words: usize,
    chars: usize,
}

fn compute_stats(filename: &String) -> Result<FileStats, std::io::Error> {
    let file = File::open(filename)?;
    let bytes = file.metadata()?.len();

    let mut reader = BufReader::new(file);
    let mut buffer = [0; 4096]; // read in 4KB chunks

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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} -c/-l/-w/-m <filename>", args[0]);
        std::process::exit(1);
    }

    let flag = &args[1];
    let filename = &args[2];

    match compute_stats(filename) {
        Ok(stats) => match flag.as_str() {
            "-c" => println!("{:8} {}", stats.bytes, filename),
            "-l" => println!("{:8} {}", stats.lines, filename),
            "-w" => println!("{:8} {}", stats.words, filename),
            "-m" => println!("{:8} {}", stats.chars, filename),
            _ => println!("Unknown option {}", flag),
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_challenge_file() {
        let filename = "test.txt".to_string();
        let stats = compute_stats(&filename).unwrap();

        assert_eq!(stats.bytes, 342190);
        assert_eq!(stats.lines, 7145);
        assert_eq!(stats.words, 58164);
        assert_eq!(stats.chars, 339292);
    }
}
