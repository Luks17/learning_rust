
// without the Debug trait, you cannot print the struct using println!
#[derive(Debug, Clone)]
pub struct User {
  pub name: String,
  pub email: String,
  pub registration_year: i16,
}

impl User {
  pub fn uses_gmail(self: &Self) -> bool {
    if self.email.contains("gmail") {
      return true;
    }
    false
  }

  // this method takes ownership of any other pointer
  pub fn same_registration_year(self: &Self, other: User) -> bool {
    if self.registration_year == other.registration_year {
      return true;
    }
    false
  }
}

#[derive(Debug, Clone)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

impl Point {
  // methods that do not use self in their arguments are actually known as associated functions, the from associated function is usually used as a constructor
  pub fn from(x: f64, y: f64) -> Self {
    Self {
      x,
      y,
    }
  }

  pub fn distance(&self, other: &Point) -> f64 {
    let x_squared = f64::powi(other.x - self.x, 2);
    let y_squared = f64::powi(other.y - self.y, 2);

    f64::sqrt(x_squared + y_squared)
  }
}
