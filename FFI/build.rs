extern crate cc;

fn main() {
  cc::Build::new()
    .file("src/lib.cc")
    .compile("libmycpp.a");
}