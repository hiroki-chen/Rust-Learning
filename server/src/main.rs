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

  panic!("OH NO!");
}
