
use std::{io::{self, Read}, fs::File};


pub fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(err) => return Err(err),
  };

  let mut string = String::new();

  // the ? operator is equivalent of using a match expression to unwrap Ok and Err
  // in this case if it is successful it will return the number of bytes that were read from file and appended to string
  // in case of failure, the function will end prematurely and return io::Error
  f.read_to_string(&mut string)?;
  Ok(string)
}
