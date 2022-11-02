
use std::{collections::HashMap, hash::Hash, cmp::Eq};


pub struct Cacher<T, U> {
  calculation: T,
  value: HashMap<U, U>,
}

impl<T, U> Cacher<T, U> 
where T: Fn(U) -> U, U: Eq + Hash + Copy {
  pub fn new(calculation: T) -> Cacher<T, U> {
    Cacher { calculation, value: HashMap::new() }
  } 

  pub fn value(self: &mut Self, arg: U) -> U {
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
