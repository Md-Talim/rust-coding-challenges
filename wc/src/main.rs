use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || args[1] != "-c" {
        eprintln!("Usage: {} -c <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[2];

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
