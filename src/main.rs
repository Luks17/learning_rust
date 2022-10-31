
#![allow(unused)]

mod game;
mod printing;
mod ownership;
mod structs;

use structs::create::User;
use structs::instantiate::new_user;
use structs::create::Point;
use game::guessing_game;


fn main() {
  guessing_game();
  // testing_ownership();
  // testing_structs()
}

fn testing_structs() {
  let mut user: User = new_user("Lucas", "lucas@email.com");

  // user implements Debug trait, so you can print it with :? or :#? using println
  println!("{:?}", user);

  // user is mutable, so you can replace its values
  user.email = user.email.replace("email", "gmail");

  println!("{:#?}\n{}", user, user.uses_gmail());


  let another_user = new_user("Thiago", "thiago@hotmail.com");

  // struct implements Clone trait, so you can clone the struct instead of moving it to the same_registration_year method
  let has_same_registration_year = user.same_registration_year(another_user.clone());

  println!("{:?}\nHas same registration year: {}", another_user, has_same_registration_year);


  let point_1 = Point::from(7.0, 4.0);
  let point_2 = Point {
    x: 5.0,
    y: 4.0,
  };

  println!("Distance between {:?} and {:?} is {}", point_1, point_2, point_1.distance(&point_2));
}

fn testing_ownership() {
  // Variable that allocates memory into the heap
  let x = String::from("a");
  printing::print_in_terminal(&x);

  ownership::takes_ownership(x);

  // Does not work because you gave away ownership of x to the takes_ownership() function
  // println!("{}", x);

  let y = String::from("My variable");

  // Does not take ownership because it uses a reference
  ownership::does_not_take_ownership(&y);

  // makes a mutable copy of y instead of taking ownership of it
  let mut z = y.clone();
  z.push_str(" Copy");

  println!("{}\n{}", y, z);

  ownership::edits_mutable_reference(&mut z);

  // does not work because x was not declared as mut
  // ownership::edits_mutable_reference(&mut x);

  println!("{}", z);

  // Showcasing string slices, they are references to a subpart of the String, so they do not take ownership
  let a = String::from("Will be sliced");
  let b = ownership::first_word_of_string(&a);

  println!("Full string: {}\nSlice: {}", a, b);
}