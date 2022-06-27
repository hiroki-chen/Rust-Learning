#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Value {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

pub fn check_value<T>(lhs: &T, rhs: &T)
  where T: PartialEq + PartialOrd
{
  assert!(lhs == rhs);
  assert!(!(lhs > rhs));
}