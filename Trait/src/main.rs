struct Foo {
  content: String,
}

struct Bar {
  content: String,
}

pub trait Log {
  fn print_log(&self, verbose: bool) -> String {
    if verbose {
      String::from("...")
    } else {
      String::from("")
    }
  }
}

use std::io;

pub trait DiskIO {
  fn write_to_disk(&self, dest: &String) -> Result<usize, io::Error>;
}

impl Log for Foo {
  fn print_log(&self, verbose: bool) -> String {
    if verbose {
      format!("{}: {}", self.content, "OK")
    } else {
      format!("{}:", self.content)
    }
  }
}

impl Log for Bar {}

/// This function is defined with where clause.
pub fn do_something_version_a<T, U>(object: &T, target: &U)
  where
    T: Log,
    U: DiskIO,
{
  // Do nothing!
}

pub fn do_something_version_b<T: Log + DiskIO>(object: &mut T) {
  // Do nothing!
}

pub fn do_something_version_c(object: &mut (impl Log + DiskIO)) {
  // Do nothing!
}

pub fn do_something_version_d(object: &mut impl Log) -> &mut impl Log {
  object
  // Looks very strange but it works :)))
}

pub fn find_largest<T>(items: &Vec<T>) -> &T
  where
    T: PartialOrd,
{
  let mut max = &items[0];

  for item in items {
    if max < &item {
      max = &item;
    }
  }

  &max
}

use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}

// Life time annotation.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

pub struct Excerpt<'a> {
  content: &'a str,
}
fn display<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
  println!("Announcement: {}", ann);

  if x.len() > y.len() {
    x
  } else {
    y
  }
}

pub fn main() {
  println!(
    "{}",
    Foo {
      content: String::from("OK")
    }
      .print_log(true)
  );

  let mut my_dict: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
  my_dict.entry(32).or_insert(100);

  let novel: &str = "Give me that thing I need. Call me by your name!";
  let a = novel.split('.').next().expect("Cannot find any dot");

  let excerpt = Excerpt {
    content: a,
  };

  println!("{}", excerpt.content);
}
