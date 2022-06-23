use utils::grep_utils;

fn main() {
  if let Err(e) = grep_utils::run() {
    panic!("{}", e);
  }
}
