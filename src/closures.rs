
pub mod caching;

use std::{thread, time::Duration};
use caching::Cacher;

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
struct CoolStruct {
  x: i32,
  y: i32,
}

pub fn test_closures() {
  let mut expensive_closure = Cacher::new(|structure: CoolStruct| {
    println!("calculating lots of stuff...");
    thread::sleep(Duration::from_secs(2));
    structure.x * structure.y
  });

  let st1 = CoolStruct { x: 2, y: 4 };
  let st2 = CoolStruct { x: -9, y: 2 };
  let st3 = CoolStruct { x: 2, y: 1 };

  println!("Calculated value: {}", expensive_closure.value(st1));
  println!("Cached calculated value: {}", expensive_closure.value(st1));
  println!("New calculated value: {}", expensive_closure.value(st2));
  println!("Another new calculated value: {}", expensive_closure.value(st3));
  println!("Cache of another new calculated value: {}\n", expensive_closure.value(st3));


  let mut not_that_expensive_closure = Cacher::new(|value: i32| {
    println!("Calculating some stuff...");
    thread::sleep(Duration::from_millis(700));
    value * 2
  });

  println!("Calculated {} resulting in: {}", 3, not_that_expensive_closure.value(-3));
  println!("Calculated {} resulting in: {}", 7, not_that_expensive_closure.value(7));
  println!("Cached {} resulting in: {}", 3, not_that_expensive_closure.value(-3));
  println!("Cached {} resulting in: {}", 7, not_that_expensive_closure.value(7));
}
