
use std::{fs::File, io::{self, ErrorKind, Write}};


pub fn open_file_with_result_using_matches() {
  let f = File::open("hello.txt");

  // unwraps file from result, if the result is and Err it will try to create the file or crash the program
  let mut f = match f {
    Ok(file) => file,
    Err(err) => match err.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(file_created) => file_created,
        Err(err) => panic!("Problem creating the file: {:?}", err),
      },
      other => panic!("Problem opening the file: {:?}", other),
    },
  };
}

pub fn create_file() -> File {
  let f = File::create("hello.txt").expect("Something went wrong creating file");

  f
}

pub fn write_username(username: &str, file: &mut File) -> Result<(), io::Error> {
  // the ? operator is equivalent of using a match expression to unwrap Ok and Err
  // in this case, if it is succesful the .write_all() method returns ()
  // in case of failure, the function exites prematurely and returns io::Error
  let f = file.write_all(username.as_bytes())?;
  Ok(f)
}
