
pub mod caching;

use std::{thread, time::Duration};

use caching::Cacher;

pub fn test_closures() {
  let mut expensive_closure = Cacher::new(|num| {
    println!("calculating lots of stuff...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  println!("Calculated value: {}", expensive_closure.value(5));
  println!("Cached calculated value: {}", expensive_closure.value(5));
  println!("New calculated value: {}", expensive_closure.value(-7));
  println!("Another new calculated value: {}", expensive_closure.value(-5));
  println!("Cache of another new calculated value: {}", expensive_closure.value(-5));
}
