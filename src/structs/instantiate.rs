
// equivalent to use crate::structs::create::User;
use super::create::User;


pub fn new_user(name: &str, email: &str) -> User {
  return User { 
    name: String::from(name),
    email: String::from(email),
    registration_year: 2022,
  }
}
