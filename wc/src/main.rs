use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn count_bytes(filename: &String) {
    match fs::read(filename) {
        Ok(data) => {
            println!("{:8} {}", data.len(), filename);
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", e, filename);
            std::process::exit(1);
        }
    }
}

fn count_lines(filename: &String) {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file {}: {}", filename, e);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let mut line_count = 0;
    for line in reader.lines() {
        match line {
            Ok(_) => line_count += 1,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                std::process::exit(1);
            }
        }
    }
    println!("{:8} {}", line_count, filename);
}

fn count_words(filename: &String) {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file {}: {}", filename, e);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let mut word_count = 0;
    for line in reader.lines() {
        match line {
            Ok(content) => word_count += content.split_whitespace().count(),
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                std::process::exit(1);
            }
        }
    }
    println!("{:8} {}", word_count, filename);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} -c/-l <filename>", args[0]);
        std::process::exit(1);
    }

    let flag = &args[1];
    let filename = &args[2];

    match flag.as_str() {
        "-c" => count_bytes(filename),
        "-l" => count_lines(filename),
        "-w" => count_words(filename),
        _ => println!("Unknown option {}", flag),
    }
}
