use std::io::{self, Read, Write};
use serde_json::json;
use serde_json::Value;

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Parse JSON input
    let data: Value = serde_json::from_str(&input).expect("Failed to parse JSON");

    // Perform computation (eg - calculate mean of X values)
    let x_values: Vec<f64> = data.as_array().unwrap().iter().map(|v| v["x"].as_f64().unwrap()).collect();
    let sum: f64 = x_values.iter().sum();
    let mean_x: f64 = sum / x_values.len() as f64;

    //Serialise result to JSON
    let result = json!({ "mean_x": mean_x });
    let result_str = serde_json::to_string(&result).expect("Failed to serialise JSON");
    
}
