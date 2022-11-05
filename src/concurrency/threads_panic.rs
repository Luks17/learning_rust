
use std::thread;

pub fn test_thread_panic() {

  let join_handle = thread::spawn(move || {
    panic!();
  });

  println!("Code will gets here even though other thread panicked");
}
