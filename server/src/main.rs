mod handler;
mod constant;
mod thread_pool;

fn main() {
  handler::run(4, &"localhost:8080".to_string()).unwrap();
}
