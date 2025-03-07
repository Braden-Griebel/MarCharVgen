// Standard Library use
use std::path::PathBuf;
// External crates use

// Local use
use marcharvgen::generator::MarkovGenerator;

#[test]
fn test_generator() -> Result<(), anyhow::Error> {
    // Get the path to words.txt (wordle words)
    let words_path = PathBuf::new();
    let words_path = words_path
        .join(file!())
        .parent()
        .unwrap()
        .join("data")
        .join("words.txt");
    // Create a generator based on the words in the words.txt file
    let generator = MarkovGenerator::from_file(words_path, 2)?;
    // Generate a new word
    let new_word = generator.generate()?;
    // Check that the first character is the expected '^'
    assert_eq!(new_word.chars().next().unwrap(), '^');
    // Check that the last character is the expected '$'
    assert_eq!(new_word.chars().next_back().unwrap(), '$');
    Ok(())
}
