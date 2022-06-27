use std::mem;
use std::slice;

extern "C" {
  fn pass_to_c(vec_ptr: *mut i32, size: usize);
}

/// A simple function that only prints the string to the console.
#[no_mangle]
pub extern "C" fn some_function() {
  println!("Rust: Hi C++!");
}

/// Array transfer example.
#[no_mangle]
pub extern "C" fn do_something_with_array(array_p: *mut i32, size: usize) {
  let slice = unsafe {
    assert!(!array_p.is_null());
    slice::from_raw_parts_mut(array_p, size)
  };

  slice[0] = 123;

  println!("Rust: Slice is {:?}", slice);
}

#[no_mangle]
pub extern "C" fn create_array_and_forget() {
  let mut vec = vec![1, 2, 3, 4, 5, 6];
  vec.shrink_to_fit();
  let vec_ptr = vec.as_mut_ptr();
  let len = vec.len();
  assert_eq!(vec.len(), vec.capacity());

  // Give the ownership to the C code and forget it.
  println!("Rust: I created the vector and passed to you!");
  mem::forget(vec);
  unsafe {
    pass_to_c(vec_ptr, len);
  }
}

#[no_mangle]
pub extern "C" fn print_array(vec_ptr: *const i32, size: usize) {
  let slice = unsafe {
    assert!(!vec_ptr.is_null() && size >= 0 as usize);
    slice::from_raw_parts(vec_ptr, size)
  };

  println!("Rust: Slice is {:?}", slice);
}

#[no_mangle]
pub extern "C" fn drop_array(vec_ptr: *mut i32, size: usize) {
  println!("Rust: I received your request and will drop the array pointer.");
  assert!(!vec_ptr.is_null() && size >= 0 as usize);

  unsafe {
    drop(Vec::from_raw_parts(vec_ptr, size, size));
  }
  println!("Rust: I have dropped the vector.");
}