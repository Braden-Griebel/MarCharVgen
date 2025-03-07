// Modules
mod config;
mod count_trie;
mod generator;
mod sampling;
// Includes
// Standard Library
use std::io::{self, BufRead, Read};
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

    Ok(())
}
