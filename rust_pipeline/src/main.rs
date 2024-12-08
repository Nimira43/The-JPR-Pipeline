use std::io::{self, Read, Write};
use serde_json::json;
use serde_json::Value;

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Parse JSON input
    let data: Value = serde_json::from_str(&input).expect("Failed to parse JSON");
}
