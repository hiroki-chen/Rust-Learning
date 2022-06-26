use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::ops::Deref;
use crate::node::ListNode;

pub fn merge_two_lists<T>(list1: Option<Box<ListNode<T>>>, list2: Option<Box<ListNode<T>>>) -> Option<Box<ListNode<T>>>
  where T: PartialEq + PartialOrd + Debug
{
  match (list1, list2) {
    (None, None) => None,
    (None, l2) => l2,
    (l1, None) => l1,
    (Some(mut l1), Some(mut l2)) => {
      if l1.val >= l2.val {
        l1.next = merge_two_lists(l1.next, Some(l2));
        Some(l1)
      } else {
        l2.next = merge_two_lists(Some(l1), l2.next);
        Some(l2)
      }
    }
  }
}