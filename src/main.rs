mod email;

use chrono::Local;
use clap::Parser;
use email::EmailConfig;
use serde_json::Value;
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

fn format_vocab_body(subject: &str, data: &[Value]) -> String {
    let mut body = format!("{}\n\n", subject);

    for dict in data.iter() {
        if let Some(obj) = dict.as_object() {
            for (key, value) in obj {
                let value_str = match value {
                    Value::String(s) => s.to_owned(),
                    _ => value.to_string(),
                };
                body.push_str(&format!("{}: {}\n", key, value_str));
            }
        }
        body.push('\n');
    }

    body
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.file_path).expect("Failed to read the JSON file");

    let data: Vec<Value> =
        serde_json::from_str(&contents).expect("Failed to parse JSON as a list of dictionaries");

    let today = Local::now().format("%B %d, %Y");
    let subject = format!("📚 Daily Vocabulary - {}", today);
    let body = format_vocab_body(&subject, &data);

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
