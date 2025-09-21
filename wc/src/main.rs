use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
};

struct FileStats {
    bytes: u64,
    lines: usize,
    words: usize,
    chars: usize,
}

fn compute_stats<R: Read>(reader: R) -> io::Result<FileStats> {
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

struct ParsedArgs {
    flags: Vec<char>,
    files: Vec<String>,
}

fn parse_args() -> ParsedArgs {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut flags = Vec::new();
    let mut files = Vec::new();

    for arg in args {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                flags.push(c);
            }
        } else {
            files.push(arg);
        }
    }

    ParsedArgs { flags, files }
}

fn main() -> io::Result<()> {
    let parsed = parse_args();

    if parsed.files.len() == 0 {
        let stats = compute_stats(io::stdin().lock())?;

        if parsed.flags.len() == 0 {
            print!("{:8} {:8} {:8}", stats.lines, stats.words, stats.bytes);
        }

        for flag in &parsed.flags {
            match flag {
                'c' => print!("{:8}", stats.bytes),
                'l' => print!("{:8}", stats.lines),
                'w' => print!("{:8}", stats.words),
                'm' => print!("{:8}", stats.chars),
                _ => print!("{:8} {:8} {:8}", stats.lines, stats.words, stats.bytes),
            }
        }
    }

    let mut total_stats = FileStats {
        bytes: 0,
        chars: 0,
        lines: 0,
        words: 0,
    };

    for file in &parsed.files {
        let stats = compute_stats(File::open(&file)?)?;

        if parsed.flags.len() == 0 {
            print!(
                "{:8} {:8} {:8} {}\n",
                stats.lines, stats.words, stats.bytes, file
            );
            total_stats.lines += stats.lines;
            total_stats.words += stats.words;
            total_stats.bytes += stats.bytes;
            continue;
        }

        for flag in &parsed.flags {
            match flag {
                'c' => {
                    print!("{:8} ", stats.bytes);
                    total_stats.bytes += stats.bytes;
                }
                'l' => {
                    print!("{:8} ", stats.lines);
                    total_stats.lines += stats.lines;
                }
                'w' => {
                    print!("{:8} ", stats.words);
                    total_stats.words += stats.words;
                }
                'm' => {
                    print!("{:8} ", stats.chars);
                    total_stats.chars += stats.chars;
                }
                _ => {
                    eprintln!("wc: invalid option -- {}", flag);
                    std::process::exit(1);
                }
            }
        }

        print!("{}\n", file);
    }

    if parsed.flags.len() == 0 {
        print!(
            "{:8} {:8} {:8} ",
            total_stats.lines, total_stats.words, total_stats.bytes
        );
    }

    for flag in &parsed.flags {
        match flag {
            'c' => print!("{:8} ", total_stats.bytes),
            'l' => print!("{:8} ", total_stats.lines),
            'w' => print!("{:8} ", total_stats.words),
            'm' => print!("{:8} ", total_stats.chars),
            _ => {
                eprintln!("wc: invalid option -- {}", flag);
                std::process::exit(1);
            }
        }
    }

    if parsed.files.len() > 0 {
        print!("total\n");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
