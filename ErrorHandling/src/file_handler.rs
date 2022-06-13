use std::fs::File;
use std::io::{self, Read};

pub fn read_from_file(file_name: &String) -> Result<String, io::Error> {
  let mut file = match File::open(file_name) {
    Ok(f) => f,
    Err(e) => return Err(e),
  };

  let mut buf: String = String::new();

  match file.read_to_string(&mut buf) {
    Ok(_) => Ok(buf),
    Err(e) => Err(e),
  }
}