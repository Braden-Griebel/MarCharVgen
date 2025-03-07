// Standard Library Uses

// External Crate Uses
use ordered_hash_map::OrderedHashMap;
use rand::distr::StandardUniform;
use rand::prelude::*;
// Local Uses

/// Function to sample from OrderedHashMap keys weighted by values
pub(crate) fn sample_map<K: Clone>(map: OrderedHashMap<K, u32>) -> Result<K, anyhow::Error> {
    // If the map is empty, return error
    if map.is_empty() {
        return Err(anyhow::anyhow!("Can't sample from empty map"));
    }
    // Check if map only has single item, return that
    if map.len() == 1 {
        return Ok(map.iter().next().unwrap().0.clone());
    }
    // Find the total value of the weights
    let mut sum = 0u32;
    map.iter().for_each(|(_, v)| sum += v);
    let sum = sum as f64;

    // Get a random value between 0 and 1
    let cutoff: f64 = rand::rng().sample(StandardUniform);

    // Iter through the map, checking if val/total>cutoff
    for (key, val) in map.iter() {
        if (*val as f64) / sum > cutoff {
            return Ok(key.clone());
        }
    }
    // If the above doesn't return, last entry is the one that should be returned
    Ok(map.iter().last().unwrap().0.clone())
}
