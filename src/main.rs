// Modules
mod config;
mod count_trie;
mod generator;
mod sampling;
// Includes
// Standard Library
use std::fs::{self, File};
use std::io::{self, LineWriter, Read, Write};
use std::path::PathBuf;
// External Crates
use clap::Parser;
// Local Includes
use count_trie::CountTrie;
use generator::MarkovGenerator;

// Clap CLI
#[derive(Parser)]
#[command(version, about, long_about, name = "MarCharVgen")]
/// Generate words simmilar to a provided corpus
///
/// Reads in words from a corpus, and creates words which
/// have simmilar letter and letter combination frequencies.
struct Cli {
    /// Optional file to read corpus from, if not provided
    /// will read from STDIN
    #[arg(short, long)]
    file: Option<PathBuf>,
    /// Optional file to output generated words to, if not provided
    /// will print to STDOUT
    #[arg(short, long)]
    outfile: Option<PathBuf>,
    /// Depth of Markov Generator, how far back generator looks when
    /// generating next character
    #[arg(short, long, default_value_t = 2)]
    depth: u32,
    #[arg(short, long)]
    /// Number of words to generate
    wordcount: u32,
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    // Create a generator either based on file contents or stdin
    let generator: MarkovGenerator = match cli.file {
        Some(f) => MarkovGenerator::from_file(f, cli.depth)?,
        None => {
            // read from STDIN
            let stdin = io::stdin();
            let mut buffer = String::new();
            stdin.lock().read_to_string(&mut buffer)?;
            MarkovGenerator::from_vec(&buffer.lines().collect::<Vec<&str>>(), cli.depth)?
        }
    };
    // If the output needs to go to a file, open that file and write line by line
    if let Some(f) = cli.outfile {
        // Open the file, and create a LineWriter
        let file = File::create(f)?;
        let mut writer = LineWriter::new(file);
        // Loop until number of desired words is generated
        for _ in 0..cli.wordcount {
            // Generate a new word, then remove the first and last
            // char, which will be STARTCHAR, and ENDCHAR
            let new_word = generator.generate()?;
            let mut chars = new_word.chars();
            chars.next();
            chars.next_back();
            let mut new_word = chars.collect::<String>();
            // Add a new line to the end of the string
            new_word.push('\n');
            // Write to the file
            writer.write_all(new_word.as_bytes())?;
        }
    } else {
        // Create a String which will hold all the output
        let mut output_string: String = String::new();
        // Loop until number of desired words generated
        for _ in 0..cli.wordcount {
            // Generate a new word, then remove the first and last
            // char, which will be STARTCHAR, and ENDCHAR
            let new_word = generator.generate()?;
            let mut chars = new_word.chars();
            chars.next();
            chars.next_back();
            let mut new_word = chars.collect::<String>();
            // Append a newline to the end after the word
            new_word.push('\n');
            // Add the new word to the end of the output
            output_string.push_str(&new_word);
        }
        // Write the output to the STDOUT
        print!("{}", output_string);
    }

    Ok(())
}
