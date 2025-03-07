// Standard Library Uses
use std::{fs::File, io::BufRead, io::BufReader, path::Path};
// External Create Uses

// Local Uses
use crate::CountTrie;
use crate::config::{ENDCHAR, STARTCHAR};
use crate::sampling::sample_map;

/// Markov text generator
pub struct MarkovGenerator {
    /// Depth of the text generation (how many previous chars to use to generate the next)
    depth: u32,
    /// Trie representing char frequency in corpus
    trie: CountTrie,
    /// String of current word being generated
    current_word: String,
}

impl MarkovGenerator {
    // Public Methods
    /// Create a new Markov word generator
    pub fn new(depth: u32) -> MarkovGenerator {
        MarkovGenerator {
            depth,
            trie: CountTrie::new(STARTCHAR),
            current_word: String::new(),
        }
    }

    /// Read words from a file to seed the generator
    pub fn from_file<P: AsRef<Path>>(
        file: P,
        depth: u32,
    ) -> Result<MarkovGenerator, anyhow::Error> {
        // Create a new generator with the appropriate depth
        let mut generator = Self::new(depth);
        // Open the file for reading
        let file = File::open(file)?;
        for line in BufReader::new(file).lines() {
            generator.insert_word(&(line?));
        }
        // Return the generator
        Ok(generator)
    }

    /// Read words from a Vec of String to seed the generator
    pub fn from_vec(in_vec: &[&str], depth: u32) -> Result<MarkovGenerator, anyhow::Error> {
        // Create a new generator with appropriate depth
        let mut generator = Self::new(depth);
        // Iterate through the Vec, adding words to trie
        in_vec.iter().for_each(|word| generator.insert_word(word));
        // Return the created generator
        Ok(generator)
    }

    /// Insert a word into the CountTrie
    pub fn insert_word(&mut self, word: &str) {
        // Append the ENDCHAR to the word
        let mut word_full = word.to_string();
        word_full.push(ENDCHAR);
        // If the word is shorter than the depth, insert the entire word
        if (word.len() as u32) < self.depth {
            self.trie.insert(&word_full);
        }
        // Otherwise, iterate through all the windows in the word,
        // and add those to the CountTrie
        word_full
            .chars()
            .collect::<Vec<char>>()
            .windows(self.depth as usize)
            .for_each(|w| {
                self.trie.insert(&w.iter().collect::<String>());
            });
    }

    /// Generate a new wornd
    pub fn generate(&self) -> Result<String, anyhow::Error> {
        // Create string to hold the word being generated
        let mut new_word = String::new();
        new_word.push(STARTCHAR);
        // Loop until the last character is ENDCHAR
        while !new_word.ends_with(ENDCHAR) {
            // Until the the new_word is longer than depth, pass in
            // whole prefix
            // Subtract one for the STARTCHAR
            let d = self.depth as usize;
            if new_word.len() - 1 <= d {
                // Get counts for next letter
                let counts = match self.trie.get_next_counts(&new_word) {
                    Ok(c) => c,
                    Err(_) => {
                        // Issue with current word and trie, append $ and return
                        new_word.push('$');
                        continue;
                    }
                };
                // Use the counts to get the next character
                new_word.push(sample_map(counts)?);
            } else {
                // The word is long enough it needs to have a substring taken
                let prefix = new_word.get((new_word.len() - d)..new_word.len()).unwrap();
                let counts = match self.trie.get_next_counts(prefix) {
                    Ok(c) => c,
                    Err(_) => {
                        // Issue with current word and trie, append $ and return
                        new_word.push('$');
                        continue;
                    }
                };
                // Use the counts to get the next character
                new_word.push(sample_map(counts)?);
            }
        }
        Ok(new_word)
    }
}
