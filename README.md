# MarCharVgen

A Markov word generator for creating new words based on the frequency of character 
sequences in a provided compendia. MarCharVgen reads a compendia of words (provided as a 
file with a single word per line), and constructs a generator based on the empirical 
frequency of different character sequences in the words present in the compendia. 
  
## Background
A Markov chain is a sequence of random variables without memory, meaning that the next 
random variable in the sequence has it's probability function only depend on the 
current state. To use this for generating words, the compendia is used to build up 
an empirical character distribution, with probabilities based on looking at the 
previous characters in the sequence (this maintains the memory-less property, because the 
state includes more than a single character). To build up this empirical distribution 
a trie is used, which in addition to recording characters, also maintains a count 
of how many character sequences pass through a node, these counts are used as the 
empirical distribution of the characters (given the sequence of characters leading to 
a node). 

## Installation
In order to use MarCharVgen, you will need git and a rust compiler. The rust compiler can be easily obtained 
by installing [Rustup](https://www.rust-lang.org/tools/install). Once a rust compiler 
is installed, clone this repository: 
```{shell}
git clone git@github.com:Braden-Griebel/MarCharVgen.git
# or 
git clone https://github.com/Braden-Griebel/MarCharVgen.git
```

then install MarCharVgen

```{shell}
cargo install --path MarCharVgen
```

Now you can use MarCharVgen! 

## Usage 
Once installed, MarCharVgen can be used just by calling the executable, i.e. 

```{shell}
marcharvgen --help
```
to show the help for marcharvgen. 
  
The most important parameters for marcharvgen, are file, the outfile, wordcount, and the depth:
- -f,--file: The file to read the corpus from, should contain a list of words (utf8 encoded) with a single word per 
    line of the file (optional, will read from STDIN if not provided)
- -o,--outfile: The file to write the generated words to (optional, if not provided will write to STDOUT)
- -w,--wordcount: Number of words to be generated
- -d,--depth: Depth of the Markov generator, essentially the number of previous characters 
    to consider when generating the next character.

