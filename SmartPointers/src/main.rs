mod node;

// A pointer is a general concept for a variable that contains an address in memory.
// This address refers to, or “points at,” some other data.
// The most common kind of pointer in Rust is a reference.
//
// Smart pointers, on the other hand, are data structures that not only act like a pointer
// but also have additional metadata and capabilities.
// The concept of smart pointers isn’t unique to Rust:
// smart pointers originated in C++ and exist in other languages as well.
// In Rust, the different smart pointers defined in the standard library provide
// functionality beyond that provided by references.

fn main() {
  // Construct two nodes with wrapped in smart pointer Box<Node<i32>>.
  let node = Box::new(node::Node::<i32>::new_with_val(&123));
  let another_node = Box::new(node::Node::<i32>::new_with_next(&123, node));
  // Dereference the smart pointer and get the normal type of Node<i32> moved.
  // another_node cannot be used anymore.
  let another_node_deref = *another_node;

  println!("{:?}", another_node_deref.next.as_ref().unwrap());
  println!("{:?}", another_node_deref.into_iter().next());

  // Let us use customized box type.
  let my_box = node::MyBox::<i32>::new(123);
  // Deref coercion
  node::deref_coercion(&my_box);
  assert_eq!(123, *my_box);
  drop(my_box);

  // Reference count.
}
