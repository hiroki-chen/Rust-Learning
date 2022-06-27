# Rust Foreign Function Interface Build Hints (C / C++ Only)
1. Declare all the external C functions:
```rust
extern "C" {
  fn some_func(ptr: *mut i32, size: usize);
}

fn call_some_func() {
  let mut vec = vec![1, 2, 3];
  vec.shrink_to_fit();
  
  // Call the function.
  let mut vec_ptr = vec.as_mut_ptr();
  assert!(!vec_ptr.is_null());
  unsafe {
    some_func(vec_ptr, vec.len());
  }
}
```
along with C definition:
```c++
extern "C" {
  void some_func(int32_t* ptr, size_t size);
}

void some_func(int32_t* ptr, size_t size) {
  // do something with the pointer.
}
```

2. Write the build script in `build.rs`:
```rust
extern crate cc;

fn main() {
  cc::Build::new()
    .file("C source file")
    .compile("library name");
}
```

3. Configure the `Cargo.toml`:
```toml
[build-dependencies]
cc = "*"

[package]
# --snip--
build = "build.rs"
```

4. Sometimes the C library will require the Rust's function (mutual deps); we need to leave unresolved symbols to be
    searched at **runtime**.
```shell
# For macOS.
$ cargo rustc -- -C link-args="-undefined dynamic_lookup";
# For Linux GCC
$ cargo rustc -- -C link-args="-Wl,--as-needed";
```

5. Finally, create a binary (either in Rust or C) and invoke all the interfaces defined.