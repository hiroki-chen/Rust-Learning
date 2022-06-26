use std::error::Error;
use std::fmt::{Debug, Formatter, Pointer};

#[derive(PartialEq)]
pub struct ListNode<T>
  where T: Debug + PartialEq + PartialOrd
{
  pub val: T,
  pub next: Option<Box<ListNode<T>>>,
}

impl<T> Debug for ListNode<T>
  where T: Debug + PartialEq + PartialOrd
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_list();
    ds.entry(&self.val);

    let mut head = self.next.as_ref();
    while let Some(node) = head.as_ref() {
      ds.entry(&node.val);
      head = head.as_ref().unwrap().next.as_ref();
    }

    ds.finish()
  }
}

impl<T> ListNode<T>
  where T: Debug + PartialEq + PartialOrd
{
  #[inline]
  pub fn new(val: T, next: Option<Box<ListNode<T>>>) -> Self {
    ListNode {
      val,
      next,
    }
  }

  pub fn to_vec(&self) -> Vec<T>
    where T: Debug + PartialEq + PartialOrd + Clone
  {
    let mut ans: Vec<T> = Vec::new();
    ans.push(self.val.clone());

    let mut head = &self.next;
    while head.is_some() {
      ans.push(head.as_ref().unwrap().val.clone());
      head = &head.as_ref().unwrap().next;
    }

    ans
  }

  pub fn add(&mut self, next: Option<Box<ListNode<T>>>) -> Result<(), Box<dyn Error>> {
    if self.next.is_some() {
      Err("Cannot change the next pointer because it is not none!".into())
    } else {
      self.next = next;
      Ok(())
    }
  }
}