use serde::Deserialize;

#[derive(Deserialize)]
pub struct VocabFile {
    pub vocab: Vec<VocabEntry>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Definition {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize)]
pub struct VocabEntry {
    pub word: String,
    pub definition: Definition,
    pub examples: Vec<Example>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Example {
    Simple(String),
    Bilingual([String; 2]),
}

pub fn format_vocab_body(subject: &str, vocab: &[VocabEntry]) -> String {
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

/// Generate the full email output for a vocabulary file.
pub fn generate_email_output(vocab_file: &VocabFile, num_words: usize) -> String {
    let vocab = &vocab_file.vocab[..num_words.min(vocab_file.vocab.len())];
    let subject = "📚 Daily Vocabulary";
    format_vocab_body(subject, vocab)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn check_golden(name: &str, actual: &str) {
        let golden_path = format!("resources/golden/{}.golden.txt", name);
        if std::env::var("UPDATE_GOLDEN").is_ok() {
            let parent = Path::new(&golden_path).parent().unwrap();
            fs::create_dir_all(parent).unwrap();
            fs::write(&golden_path, actual).unwrap();
            println!("Updated golden file: {}", golden_path);
        } else {
            let expected = fs::read_to_string(&golden_path)
                .unwrap_or_else(|_| panic!("Golden file not found: {}. Run with UPDATE_GOLDEN=1 to create it.", golden_path));
            assert_eq!(actual, expected, "Output does not match golden file: {}", golden_path);
        }
    }

    fn load_and_generate(vocab_path: &str) -> String {
        let contents = fs::read_to_string(vocab_path)
            .unwrap_or_else(|_| panic!("Failed to read vocab file: {}", vocab_path));
        let vocab_file: VocabFile = serde_json::from_str(&contents)
            .unwrap_or_else(|_| panic!("Failed to parse vocab file: {}", vocab_path));
        generate_email_output(&vocab_file, vocab_file.vocab.len())
    }

    #[test]
    fn test_vocab_golden() {
        let output = load_and_generate("resources/vocab.json");
        check_golden("vocab", &output);
    }

    #[test]
    fn test_vocab2_golden() {
        let output = load_and_generate("resources/vocab2.json");
        check_golden("vocab2", &output);
    }
}
