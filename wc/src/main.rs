mod args;
mod stats;

use args::parse_args;
use stats::{FileStats, compute_stats};
use std::{
    collections::HashSet, fs::File, io::{self}
};

fn print_stats(stats: &FileStats, flags: &HashSet<char>, filename: Option<&str>) {
    let print_order = ['l', 'w', 'm', 'c'];

    if flags.is_empty() {
        print!("{:8} {:8} {:8}", stats.lines, stats.words, stats.bytes);
    } else {
        for flag in print_order {
            if flags.contains(&flag) {
                match flag {
                    'c' => print!("{:8}", stats.bytes),
                    'l' => print!("{:8}", stats.lines),
                    'w' => print!("{:8}", stats.words),
                    'm' => print!("{:8}", stats.chars),
                    _ => {
                        eprintln!("wc: invalid option -- {}", flag);
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    if let Some(fname) = filename {
        print!(" {}", fname);
    }
    println!();
}

fn main() -> io::Result<()> {
    let parsed = parse_args();

    if parsed.files.len() == 0 {
        let stats = compute_stats(io::stdin().lock())?;
        print_stats(&stats, &parsed.flags, None);
        return Ok(());
    }

    let mut total_stats = FileStats::default();

    for file in &parsed.files {
        let stats = compute_stats(File::open(&file)?)?;
        print_stats(&stats, &parsed.flags, Some(file));
        total_stats.add(&stats);
    }

    if parsed.files.len() > 1 {
        print_stats(&total_stats, &parsed.flags, Some("total"));
    }

    Ok(())
}
