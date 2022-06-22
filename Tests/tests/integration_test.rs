use Tests::target;

mod common;

#[test]
fn should_work() {
  let lhs = target::Value {
    x: 123,
    y: 123,
    z: 123,
  };

  let rhs = lhs.clone();

  target::check_value(&lhs, &rhs);
}