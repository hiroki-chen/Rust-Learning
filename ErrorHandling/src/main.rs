use std::env;
use std::fs::File;
use std::io::ErrorKind;

mod file_handler;

fn main() {
  println!(
    "Your current directory is: {}",
    env::current_dir().unwrap().display()
  );
  let f = match File::open("./data/input.txt") {
    Ok(file) => file,
    Err(error) => panic!("{:?}", error),
  };

  // Code snippet from previous.
  let f = match File::open("./data/input.txt") {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("./data/input.txt") {
        Ok(file) => file,
        Err(e) => panic!("{:?}", e),
      },
      other_error => {
        panic!("Unknown error! {:?}", other_error)
      }
    },
  };

  let res: String = std::fs::read_to_string("./data/input.txt").expect("No such thing");
  println!("{}", res);

  let content = file_handler::read_from_file(&String::from("./data/input.txt")).unwrap();
  println!("{}", content);
}

use std::io;
use std::cmp;


fn foo<'a, T>(items: &'a Vec<T>) -> Result<&'a T, Box<dyn std::error::Error>>
  where T: PartialOrd {
  let len: usize = items.len();
  // Check if empty.
  if len == 0 {
    return Err("Vector cannot be empty!".into());
  }

  let mut maximum = &items[0];
  for item in items {
    if maximum < item {
      maximum = item;
    }
  }

  Ok(maximum)
}
