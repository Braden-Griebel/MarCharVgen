// Standard Library Uses
use std::collections::HashMap;
// External Crate Uses
use ordered_hash_map::OrderedHashMap;
// Local Uses

/// A Trie structure which includes additional information of number
/// of inserted character sequences which pass through a particular node.
#[derive(Clone, PartialEq, Debug)]
pub struct CountTrie {
    /// Character represented by this node
    node_char: char,
    /// Count of sequences passing through this node
    count: u32,
    /// Children of this node, indexed by their Characters
    children: HashMap<char, Box<CountTrie>>,
}

impl CountTrie {
    // Public Methods
    /// Create a new node from a character
    pub fn new(node_char: char) -> CountTrie {
        CountTrie {
            node_char,
            count: 0,
            children: HashMap::new(),
        }
    }

    /// Insert a string into the CountTrie
    pub fn insert(&mut self, insert_str: &str) {
        // Increment count, since insert was called on this node
        self.count += 1;
        // If received empty string, simply return
        if insert_str.len() == 0 {
            return;
        }
        // Create a new child if needed
        let new_child = self
            .children
            .entry(insert_str.chars().next().unwrap())
            .or_insert(Box::new(CountTrie::new(insert_str.chars().next().unwrap())));
        // Pass the tail of the string to the
        new_child.insert(insert_str.get(1..).unwrap());
    }

    /// Get Counts of next characters
    pub(crate) fn get_next_counts(
        &self,
        prefix: &str,
    ) -> Result<OrderedHashMap<char, u32>, CountTrieError> {
        // If the prefix is the empty string, this is the penultimate node
        if prefix.is_empty() {
            return Ok(self.get_child_counts());
        }
        // If the children map is empty, the prefix is too long for this trie
        if self.children.is_empty() {
            return Err(CountTrieError::PrefixTooLong(prefix.to_string()));
        }
        // Otherwise, pass the tail of prefix to correct child
        match self.children.get(&prefix.chars().next().unwrap()) {
            Some(val) => return val.get_next_counts(prefix.get(1..).unwrap()),
            None => Err(CountTrieError::NoNextChild(prefix.to_string())),
        }
    }

    // Private Methods
    /// Get the count of the node
    fn get_count(&self) -> u32 {
        self.count
    }

    /// Get an OrderedHashMap of the children of a node
    fn get_child_counts(&self) -> OrderedHashMap<char, u32> {
        let mut count_map: OrderedHashMap<char, u32> = OrderedHashMap::new();
        self.children.iter().for_each(|(k, v)| {
            _ = count_map.insert(*k, v.get_count());
        });
        count_map
    }
}

#[derive(Debug, thiserror::Error)]
/// Errors associated with CountTrie
enum CountTrieError {
    /// Occurs when the prefix is too long for the trie
    #[error("Prefix exceeds trie depth: {0}")]
    PrefixTooLong(String),
    /// Occurs when the first char doesn't match the children of a node
    ///
    /// Should only occur when the sequence generated wasn't seen in
    /// training, which should be impossible
    #[error("No match in node's children for next char: {0}")]
    NoNextChild(String),
}

#[cfg(test)]
mod test_counttrie {
    use super::*;

    #[test]
    fn test_insert() {
        // Create a test CountTrie
        let mut test_trie = CountTrie::new('^');
        // Insert some words into the trie
        test_trie.insert("cat$");
        test_trie.insert("car$");
        // Check that c is a child of the root, with count 2
        let c_node = test_trie.children.get(&'c').unwrap();
        assert_eq!(c_node.get_count(), 2);
        // Check that a is a child of this node with count 2
        let a_node = c_node.children.get(&'a').unwrap();
        assert_eq!(a_node.get_count(), 2);
        // Check that r and t are both children of this node, with counts 1
        let t_node = a_node.children.get(&'t').unwrap();
        assert_eq!(t_node.get_count(), 1);
        let r_node = a_node.children.get(&'r').unwrap();
        assert_eq!(r_node.get_count(), 1);
    }

    #[test]
    fn test_counts() {
        // Create a test CountTrie
        let mut test_trie = CountTrie::new('^');
        // Insert some words into the trie
        test_trie.insert("cat$");
        test_trie.insert("car$");
        // Check next counts for ca
        let ncounts = test_trie.get_next_counts("ca").unwrap();
        assert_eq!(ncounts.get(&'t').unwrap().clone(), 1u32);
        assert_eq!(ncounts.get(&'r').unwrap().clone(), 1u32);
    }
}
