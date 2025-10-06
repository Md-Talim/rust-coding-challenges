use std::io::{self, Read};
use std::process::exit;

#[derive(Debug, PartialEq)]
enum JsonValue {
    Object,
}

fn parse_json(input: &str) -> Result<JsonValue, String> {
    let trimmed = input.trim();
    if trimmed == "{}" {
        Ok(JsonValue::Object)
    } else {
        Err("Invalid JSON".into())
    }
}

fn main() {
    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("Failed to read input: {}", err);
        exit(1);
    }

    match parse_json(&mut input) {
        Ok(_) => exit(0), // Valid JSON
        Err(msg) => {
            eprintln!("{}", msg);
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::JsonValue;
    use crate::parse_json;

    #[test]
    fn test_empty_object() {
        let result = parse_json("{}");
        assert_eq!(result, Ok(JsonValue::Object));
    }

    #[test]
    fn test_invalid_object() {
        let result = parse_json("{");
        assert_eq!(result, Err("Invalid JSON".into()));
    }
}
