pub mod node;
pub mod list_utils;

#[cfg(test)]
mod tests {
  use std::borrow::{Borrow, BorrowMut};
  use crate::list_utils::merge_two_lists;
  use crate::node;

  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }

  #[test]
  fn test_merge() {
    let a = Box::new(node::ListNode::new(1, None));
    let b = Box::new(node::ListNode::new(3, Some(a)));
    let c = Box::new(node::ListNode::new(5, Some(b)));

    let d = Box::new(node::ListNode::new(2, None));
    let e = Box::new(node::ListNode::new(4, Some(d)));
    let f = Box::new(node::ListNode::new(6, Some(e)));

    let ans = &merge_two_lists(Some(c), Some(f));
    println!("List: {:?}", ans);
    assert_eq!(vec![6,5,4,3,2,1], ans.as_ref().unwrap().to_vec());

  }
}
