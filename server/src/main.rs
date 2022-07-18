mod handler;
mod constant;
mod thread_pool;
mod shopping_cart;

fn main() {
  handler::run(4, &"localhost:8080".to_string()).unwrap();

  // This will be reported by MIRAI by running cargo mirai.
  // warning: OH NO!
  //   --> src/main.rs:10:3
  //   |
  //   10 |   panic!("OH NO!");
  // |   ^^^^^^^^^^^^^^^^

  let mut cart = shopping_cart::ShoppingCart::new();
  // The below causes a diagnostic because pre-condition of Item::new is violated.
  // probably needs to be modified. The diagnostic can be quite simple because there is a call stack.
  cart.add(shopping_cart::Item::new("free lunch", 0));
  panic!("OH NO!");
}

#[cfg(test)]
mod tests {
  use crate::shopping_cart;

  #[test]
  fn ok() {
    let mut cart = shopping_cart::ShoppingCart::new();
    cart.add(shopping_cart::Item::new("ipad pro", 899));
    cart.add(shopping_cart::Item::new("ipad folio", 169));
    assert_eq!(cart.checkout(), 899 + 169);
  }

  // todo: teach MIRAI about should_panic
  #[test]
  #[should_panic(expected = "Pre-condition of new violated")]
  fn fail_item_new() {
    let mut cart = shopping_cart::ShoppingCart::new();
    // Below violates precondition of Item::new
    cart.add(shopping_cart::Item::new("free lunch", 0));
  }

  #[test]
  #[should_panic(expected = "Invariant of add_broken_invariant violated")]
  fn fail_add_invariant() {
    let mut cart = shopping_cart::ShoppingCart::new();
    cart.add_broken_invariant(shopping_cart::Item::new("ipad pro", 899));
  }
}
