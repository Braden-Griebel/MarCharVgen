// Standard Library Uses
use std::{fs::File, io::BufRead, io::BufReader, path::Path};
// External Create Uses

// Local Uses
use crate::config::{ENDCHAR, STARTCHAR};
use crate::count_trie::CountTrie;
use crate::sampling::sample_map;

/// Markov text generator
pub struct MarkovGenerator {
    /// Depth of the text generation (how many previous chars to use to generate the next)
    depth: u32,
    /// Trie representing char frequency in corpus
    trie: CountTrie,
}

impl MarkovGenerator {
    // Public Methods
    /// Create a new Markov word generator
    pub fn new(depth: u32) -> MarkovGenerator {
        MarkovGenerator {
            depth,
            trie: CountTrie::new(STARTCHAR),
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
        if (word.len() as u32) <= self.depth {
            self.trie.insert(&word_full);
        }
        // Otherwise, iterate through all the windows in the word,
        // and add those to the CountTrie
        word_full
            .chars()
            .collect::<Vec<char>>()
            .windows((self.depth+1) as usize)
            .for_each(|w| {
                self.trie.insert(&w.iter().collect::<String>());
            });
    }

    /// Generate a new wornd
    pub fn generate(&self) -> Result<String, anyhow::Error> {
        // Create string to hold the word being generated
        let mut new_word = String::new();
        new_word.push(STARTCHAR);
        // Get the depth as a usize here, 
        let d = self.depth as usize;
        // Loop until the last character is ENDCHAR
        while !new_word.ends_with(ENDCHAR) {
            // Until new_word is longer than depth, pass in
            // whole prefix
            // Subtract one for the STARTCHAR
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

#[cfg(test)]
mod generator_tests{
    use super::*;

    #[test]
    fn test_word_generation()->Result<(), anyhow::Error>{
        // Create a new generator
        let mut generator = MarkovGenerator::new(3);
        // Insert some words
        generator.insert_word("cat");
        generator.insert_word("dog");
        generator.insert_word("pizza");
        generator.insert_word("car");
        generator.insert_word("tiger");
        generator.insert_word("elephant");
        generator.insert_word("capybara");
        generator.insert_word("people");
        generator.insert_word("gorilla");
        // Generate a new word
        let new_word = generator.generate()?;
        // Check first and last characters are as expected 
        assert!(new_word.starts_with(STARTCHAR));
        assert!(new_word.ends_with(ENDCHAR));
        // The length of the word should be at least 4 characters 
        // (2 for ^ and $, and at least 2 generated)
        assert!(new_word.len() >= 4);
        Ok(())
    }
}