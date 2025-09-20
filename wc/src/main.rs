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

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let (flag, filename) = match args.len() {
        1 => ("", None),
        2 => {
            let arg = &args[1];
            if arg.starts_with('-') {
                (arg.as_str(), None)
            } else {
                ("", Some(arg))
            }
        }
        3 => {
            let flag = &args[1];
            (flag.as_str(), Some(&args[2]))
        },
        _ => {
            eprintln!("Usage: {} [-c/-l/-w-/-m] [filename]", args[0]);
            std::process::exit(1);
        }
    };

    let stats = if let Some(fname) = filename {
        compute_stats(File::open(fname)?)?
    } else {
        compute_stats(io::stdin().lock())?
    };

    match flag {
        "-c" => print!("{:8}", stats.bytes),
        "-l" => print!("{:8}", stats.lines),
        "-w" => print!("{:8}", stats.words),
        "-m" => print!("{:8}", stats.chars),
        _ => print!("{:8} {:8} {:8}", stats.lines, stats.words, stats.bytes),
    }

    if let Some(fname) = filename {
        print!(" {}\n", fname);
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
