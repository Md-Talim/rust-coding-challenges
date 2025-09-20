use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct FileStats {
    bytes: usize,
    lines: usize,
    words: usize,
}

fn compute_stats(filename: &String) -> Result<FileStats, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut bytes = 0;
    let mut lines = 0;
    let mut words = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        bytes += line.len() + 1; // +1 for the stripped newline
        lines += 1;
        words += line.split_whitespace().count();
    }

    return Ok(FileStats {
        bytes,
        lines,
        words,
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} -c/-l <filename>", args[0]);
        std::process::exit(1);
    }

    let flag = &args[1];
    let filename = &args[2];

    match compute_stats(filename) {
        Ok(stats) => match flag.as_str() {
            "-c" => println!("{:8} {}", stats.bytes, filename),
            "-l" => println!("{:8} {}", stats.lines, filename),
            "-w" => println!("{:8} {}", stats.words, filename),
            _ => println!("Unknown option {}", flag),
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
