use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    // Prompt the user to provide the file path
    println!("Please provide the path to the text file:");

    // Read the file path from user input
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read input.");
    let file_path = file_path.trim(); // Trim whitespace

    // Try to open the file
    let file = File::open(file_path);
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    // Use a buffered reader to read the file line by line
    let reader = BufReader::new(file);

    // HashMap to store word frequencies
    let mut word_count: HashMap<String, usize> = HashMap::new();

    // Read the file and count word frequencies
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Split the line into words and count each word
                for word in line.split_whitespace() {
                    // Clean up the word (remove punctuation, make lowercase)
                    let word = word
                        .to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric())
                        .collect::<String>();

                    if !word.is_empty() {
                        *word_count.entry(word).or_insert(0) += 1;
                    }
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    // Display the word frequencies
    println!("\nWord frequencies:");
    for (word, count) in word_count {
        println!("{}: {}", word, count);
    }
}
