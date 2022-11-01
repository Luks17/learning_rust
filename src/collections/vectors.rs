

pub fn test_vectors() {
  let mut v: Vec<i32> = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);
  v.push(4);

  // you can declare a vector directly using the vec! macro
  // let v = vec![1, 2, 3, 4]; <- this is equivalent to all the above

  // by default, even while being stored in the heap, vectors do not lose ownership of their elements when you assign them to another variable
  // because what happens actually is that the Index::index trait method is called with a refere to self, returning a copy
  let x = v[2];
  // let x = *Index::index(&v, 2); <- this is equivalent to the above

  fn does_not_take_ownership(x: i32) {
    println!("Took ownership of {}", x);
  }

  does_not_take_ownership(x);
  println!("{} {}", x, v[2]);

  // when you iterate through the elements in your vector you DO lose ownership, so you better use the & operator
  for i in &mut v {
    *i += 1;
  }

  // even though all the above works, usign the index to access the value can lead to error when that index does not actually exist
  // so it's better to use the .get method
  let y = v.get(4);
  match y {
    Some(y) => { println!("fifth value: {}", y) },
    None => { println!("There is no fifth value") },
  }
}






