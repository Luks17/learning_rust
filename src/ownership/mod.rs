
pub fn takes_ownership(x: String) {
  println!("Printing {} after taking ownership of it", x);
}

pub fn does_not_take_ownership(x: &String) {
  println!("Printing {} after not taking ownership of it", x);
  
  // By default, you cannot modify a reference
  // x.push_str("This does not work");
}

pub fn edits_mutable_reference(x: &mut String) {
  println!("Printing {} after not taking ownership of it", x);

  x.push_str(" - Yes, This DOES work");
}

// takes slice of string
pub fn first_word_of_string(x: &String) -> &str {
  // transforms strings to bytes because it's easier to iterate
  let bytes = x.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &x[..i];
    }
  }

  &x[..]
}
