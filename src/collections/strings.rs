
pub fn test_strings() {
  // Strings in rust, unlike c and c++, are collections of UTF-8 encoded bytes, so we have a long range of character we can use and
  // characters in utf-8 can actually have different sizes, they are not all the same 1 byte like it is in C and C++
  // As we already know, there are two main types of strings in rust:
  // &str, which is coded in the core language, it's stored in the stack and cannot grow
  // String, which is coded in the standard library, it's stored in the heap, mutable, owned and can grow
  let hello = "Здравствуйте";

  // when joining two Strings with the + operator, we give away the ownership of the first String, we also need to give references of the
  // next values istead of the value (which is stored in the heap) 
  let str1 = String::from("Hello ");
  let str2 = String::from("World");
  let str3 = String::from("!!!");
  let str4 = str1 + &str2 + &str3;

  // println!("{}", str1); <- this does not work because we lost ownership of it
  println!("str2: {}\nstr3: {}\nstr4: {}", str2, str3, str4);

  // if we join Strings and &str instead, we can safely join them without losing ownership of any of them
  let mut str5 = String::from("Russian lol - ") + hello;

  // we can also join &str with String using the push_str method
  str5.push_str(" - Lol");

  // As we already know, since utf-8 characters can have different sizes, we can't just access them like we would in other languages where
  // a String is as simple as an array of 1 byte chacacters.
  // We need to specify how we want to access each character, because in Unicode there is actually 3 ways we can represent them:
  // Vector of u8 bytes, scalar (diacritics, which may or may not resemble a letter as we humans see them) and graphemes (actual letters) 

  // we can see below that each character has a different number of bytes
  for char in str5.as_bytes() {
    print!("{} ", char);
  }

  println!();

  // we can iterate through them as scalars
  for char in str5.chars() {
    print!("{} ", char);
  }

  // we cannot iterate through them as graphemes using just the standard library because it's complex to do it, so rust decided to
  // not include that feature to keep the std library lean, the following crate can do it though
  // https://crates.io/crates/unicode-segmentation
}
