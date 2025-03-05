// Modules
mod count_trie;
mod generator;
mod sampling;
// Includes
// Standard Library
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
    /// Depth of Markov Generator, how far back generator looks when
    /// generating next character
    #[arg(short, long, default_value_t = 2)]
    depth: i32,
    #[arg(short, long)]
    /// Number of words to generate
    wordcount: i32,
}

fn main() {
    let cli = Cli::parse();
}
