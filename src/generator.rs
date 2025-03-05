// Standard Library Uses

// External Create Uses

// Local Uses
use crate::CountTrie;

// Define the start and end characters for the produced words
const STARTCHAR: char = '^';
const ENDCHAR: char = '$';

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
}
