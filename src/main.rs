use clap::Parser;
use serde_json::Value;
use std::fs;

#[derive(Parser)]
#[command(name = "daily-vocab-mailer")]
#[command(about = "Reads and displays vocabulary from a JSON file")]
struct Args {
    /// Path to the vocabulary JSON file
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.file_path).expect("Failed to read the JSON file");

    let data: Vec<Value> =
        serde_json::from_str(&contents).expect("Failed to parse JSON as a list of dictionaries");

    // TODO: Eventually this will send emails
    for (index, dict) in data.iter().enumerate() {
        println!("Dictionary {}: {}", index + 1, dict.to_string());
    }
}
