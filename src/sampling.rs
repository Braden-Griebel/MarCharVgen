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
    let mut cum_sum = 0f64;
    for (key, val) in map.iter() {
        cum_sum += *val as f64 / sum;
        if cum_sum > cutoff {
            return Ok(key.clone());
        }
    }
    // If the above doesn't return, last entry is the one that should be returned
    Ok(map.iter().last().unwrap().0.clone())
}

#[cfg(test)]
mod test_sampler {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_sample_map() {
        let mut map: OrderedHashMap<char, u32> = OrderedHashMap::new();
        map.insert('a', 2);
        map.insert('b', 1);
        map.insert('c', 1);
        map.insert('d', 1);
        let mut out_map: HashMap<char, u32> = HashMap::new();
        for _ in 0..1000{
            let sampled_char = sample_map(map.clone()).unwrap();
            let count = out_map.entry(sampled_char).or_insert(0);
            *count += 1;
        }
        // a should be the most frequently sampled
        assert!(out_map.get(&'a').unwrap() > out_map.get(&'b').unwrap());
        assert!(out_map.get(&'a').unwrap() > out_map.get(&'c').unwrap());
        assert!(out_map.get(&'a').unwrap() > out_map.get(&'d').unwrap());
    }
}
