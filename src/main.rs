mod email;

use chrono::Local;
use clap::Parser;
use email::EmailConfig;
use serde::Deserialize;
use std::fs;

#[derive(Parser)]
#[command(name = "daily-vocab-mailer")]
#[command(about = "Sends daily vocabulary emails from a JSON file")]
struct Args {
    /// Path to the vocabulary JSON file
    #[arg(short, long)]
    file_path: String,

    /// Recipient email address
    #[arg(short, long)]
    to: String,

    /// Dry run mode - print email instead of sending
    #[arg(long)]
    dry_run: bool,
}

#[derive(Deserialize)]
struct VocabFile {
    vocab: Vec<VocabEntry>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Definition {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize)]
struct VocabEntry {
    word: String,
    definition: Definition,
    examples: Vec<Example>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Example {
    Simple(String),
    Bilingual([String; 2]),
}

fn format_vocab_body(subject: &str, vocab: &[VocabEntry]) -> String {
    let mut body = format!("{}\n\n", subject);

    for entry in vocab.iter() {
        body.push_str(&format!("{}\n", entry.word));
        match &entry.definition {
            Definition::Single(def) => body.push_str(&format!("{}\n", def)),
            Definition::Multiple(defs) => {
                for def in defs {
                    body.push_str(&format!("{}\n", def));
                }
            }
        }
        body.push_str("Examples:\n");
        for example in &entry.examples {
            match example {
                Example::Simple(s) => body.push_str(&format!("  - {}\n", s)),
                Example::Bilingual([first, second]) => {
                    body.push_str(&format!("  - {}\n", first));
                    body.push_str(&format!("    {}\n", second));
                }
            }
        }
        body.push('\n');
    }

    body
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.file_path).expect("Failed to read the JSON file");

    let vocab_file: VocabFile =
        serde_json::from_str(&contents).expect("Failed to parse vocabulary JSON file");

    let today = Local::now().format("%B %d, %Y");
    let subject = format!("📚 Daily Vocabulary - {}", today);
    let body = format_vocab_body(&subject, &vocab_file.vocab);

    if args.dry_run {
        println!("=== DRY RUN ===");
        println!("To: {}", args.to);
        println!("Subject: {}", subject);
        println!("Body:\n{}", body);
    } else {
        let config = EmailConfig::from_env().expect("Failed to load email configuration");
        match config.send_email(&args.to, &subject, &body) {
            Ok(_) => println!("Email sent successfully to {}", args.to),
            Err(e) => eprintln!("Failed to send email: {}", e),
        }
    }
}
