mod email;

use chrono::Local;
use clap::Parser;
use daily_vocab_mailer::{format_vocab_body, VocabFile};
use email::EmailConfig;
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

    /// Number of vocabulary words to send
    #[arg(short = 'n', long, default_value = "5")]
    num_words: usize,

    /// Dry run mode - print email instead of sending
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.file_path).expect("Failed to read the JSON file");

    let vocab_file: VocabFile =
        serde_json::from_str(&contents).expect("Failed to parse vocabulary JSON file");

    let vocab = &vocab_file.vocab[..args.num_words.min(vocab_file.vocab.len())];

    let today = Local::now().format("%B %d, %Y");
    let subject = format!("📚 Daily Vocabulary - {}", today);
    let body = format_vocab_body(&subject, vocab);

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
