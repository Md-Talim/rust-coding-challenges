use std::env;

pub struct ParsedArgs {
    pub flags: Vec<char>,
    pub files: Vec<String>,
}

pub fn parse_args() -> ParsedArgs {
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
