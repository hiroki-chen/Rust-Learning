#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn should_work() {
    let x = 2;
    let y = 4;
    assert_eq!(x + y, 6);

    let lhs = target::Value {
      x: 123,
      y: 123,
      z: 123,
    };

    let rhs = lhs.clone();

    target::check_value(&lhs, &rhs);
  }

  #[test]
  #[should_panic]
  pub fn check_panic() {
    let test: Option<i32> = None;
    println!("{}", test.expect("Should panic here!"));
  }
}

pub mod target;