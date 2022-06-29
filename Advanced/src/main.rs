use std::borrow::Borrow;
use std::ops::Add;
use std::fmt;
use std::sync;
use std::thread;
use std::time::Duration;

mod generator;

static mut COUNTER: u32 = 0;

// Unsafe static mutable thing...
fn increment(step: u32) {
  unsafe {
    COUNTER += step;
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point<T = i32> {
  x: T,
  y: T,
}

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl<T> Point<T> {
  fn new(x: T, y: T) -> Self {
    Point { x, y }
  }
}

trait MyPrint: fmt::Display {
  fn print(&self) {
    let output = self.to_string();
    let len = output.len();
    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {} *", output);
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl MyPrint for Point {}

impl Add for Point {
  type Output = Point;

  fn add(self, rhs: Self) -> Self::Output {
    Self { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}

/// Add<Rhs=Type> -> Millimeters + Meters => Millimeters.
impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, rhs: Meters) -> Self::Output {
    Millimeters(self.0 + (rhs.0 * 1000))
  }
}

fn main() {
  println!("{:?}", Millimeters(19) + Meters(20));
  let point = Point { x: 12, y: 34 };
  point.print();

  let content = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let (tx, rx) = sync::mpsc::channel();

  // Let us create a thread.
  let t = thread::spawn(move || {
    for i in 0..10 {
      println!("{:?}: num = {}", thread::current().id(), &content[i]);
      thread::sleep(Duration::from_millis(100));
    }

    tx.send(format!("Hello! I am {:?}", thread::current().id())).unwrap();
  });

  for i in 1..=5 {
    println!("{:?}: num = {}", thread::current().id(), i);
    thread::sleep(Duration::from_millis(100));
  }

  println!("Got {}", rx.recv().unwrap());

  t.join().unwrap_or_else(|e| panic!("Error: {:?}", e));

  generator::generate(false);
}
