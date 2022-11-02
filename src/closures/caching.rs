
use std::{collections::HashMap, hash::Hash, cmp::Eq};


pub struct Cacher<T, U, V> {
  calculation: T,
  value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V> 
where T: Fn(U) -> V, U: Eq + Hash + Copy, V: Copy {
  pub fn new(calculation: T) -> Cacher<T, U, V> {
    Cacher { calculation, value: HashMap::new() }
  } 

  pub fn value(self: &mut Self, arg: U) -> V {
    match self.value.get(&arg) {
      Some(x) => *x,
      None => {
        let x = (self.calculation)(arg);
        self.value.entry(arg).or_insert(x);
        x
      } 
    }
  }
}
