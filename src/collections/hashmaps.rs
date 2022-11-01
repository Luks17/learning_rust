
use std::collections::{hash_map, HashMap};

pub fn test_hash_map() {
  let mut teams_scores = HashMap::new();

  teams_scores.insert("Flamengo", 5);
  teams_scores.insert("Sao Paulo", 5);
  teams_scores.insert("Corinthians", 4);

  // if you insert again to a key that already exists, it is updated and an option with the old value is returned, 
  // alternatively an option with None is returned
  let old_value = teams_scores.insert("Sao Paulo", 6);

  match old_value {
    Some(i) => { println!("{}", i) },
    None => { () },
  }

  // sometimes we want to insert a new key,value pair only if it does not already exist, for that we can use the .entry() and .or_insert() methods
  teams_scores.entry("Palmeiras").or_insert(3);

  // if it exists or not, the .or_insert() method always returns a mutable reference to the value, so we can modify it as we wish
  let key = teams_scores.entry("Sao Paulo").or_insert(5);
  *key += 1;

  match teams_scores.get("Sao Paulo") {
    Some(i) => { println!("{}", i) },
    None => {},
  }
}
