
#[derive(Debug)]
pub enum UsState {
  NewYork,
  Florida,
  Ohio,
  California,
  NewJersey,
  Texas,
  Chicago,
}

#[derive(Debug)]
pub enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

// if coin quarter is from NY: count += 5 and print state, if quarter from other state: count += 3, else: count += 1 and print coin type
pub fn counts_coins(array: &[Coin]) -> i32 {
  let mut count = 0;

  for coin in array {
    match coin {
      Coin::Quarter(UsState::NewYork) => { 
        count += 5;
        println!("{:?}", UsState::NewYork);
      },

      // '_' represents an 'else' in match
      Coin::Quarter(_) => { count += 3 },

      // 'other' also represents an 'else' in match, but you can also access it's value as shown in the println!, unlike '_'
      other => { 
        println!("{:?}", other);
        count += 1;
      },
    }
  };

  count
}
