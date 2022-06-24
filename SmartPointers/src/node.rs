use std::error::Error;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub struct MyBox<T>
  where T: Display {
  val: T,
}

impl<T> MyBox<T>
  where T: Display {
  pub fn new(val: T) -> Self {
    MyBox {
      val,
    }
  }
}

impl<T> Drop for MyBox<T>
  where T: Display {
  fn drop(&mut self) {
    println!("Goodbye! {}", self.val);
  }
}

impl<T> Deref for MyBox<T>
  where T: Display {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.val
  }
}

#[derive(Debug)]
pub struct Vertex<T>
  where T: Display {
  pub val: Option<T>,
  pub adj: Vec<Rc<Vertex<T>>>,
  pub id: u32,
}

#[derive(Debug)]
pub struct Node<T> {
  pub next: Option<Box<Node<T>>>,
  pub val: T,
}

#[derive(Debug)]
pub struct DAGNode<T, U>
  where U: Copy + Clone + Display
{
  pub children: Vec<Option<Box<DAGNode<T, U>>>>,
  pub val: (T, U),
}

impl<T> Iterator for Node<T>
  where T: Clone + Copy {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    // Because the ownership is limited, we cannot directly move
    // the Option object. We can therefore only access it by reference.
    if let Some(val) = self.next.as_ref() {
      Some(val.val)
    } else {
      None
    }
  }
}

impl<T> Vertex<T>
  where T: Display + Clone + PartialEq {
  pub fn new(val: &T, id: u32) -> Self {
    Vertex {
      val: Some(val.clone()),
      adj: Vec::new(),
      id,
    }
  }

  pub fn add(&mut self, other: &Rc<Vertex<T>>) -> Result<(), Box<dyn Error>> {
    if let Some(_) = self.adj.iter().find(|x| x.id == other.id) {
      Err("Repeated item!".into())
    } else {
      self.adj.push(Rc::clone(other));
      Ok(())
    }
  }
}

impl<T> Node<T> {
  pub fn new_with_val(val: &T) -> Self
    where T: Clone + Copy
  {
    Node {
      next: None,
      val: val.clone(),
    }
  }

  pub fn new_with_next(val: &T, next: Box<Node<T>>) -> Self
    where T: Clone + Copy
  {
    Node {
      next: Some(next),
      val: val.clone(),
    }
  }
}

pub fn deref_coercion(val: &i32) {
  println!("{}", val);
}