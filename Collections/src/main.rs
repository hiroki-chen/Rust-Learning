use std::char;

fn main() {
  let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];

  // Update the vector.
  vec.push(1);

  // Access the element.
  let val: &i32 = &vec[0];
  println!("{}", val);

  if let Some(val) = vec.get(0) {
    println!("{}", val);
  } else {
    println!("Non-existing!");
  }

  let mut a = vec![1, 2, 3, 4, 5];
  let mut b = vec![6, 7, 8, 9, 10];
  a.append(&mut b);
  assert_eq!(a, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
  let first = &a[0];
  // v.push(6); // Won't compile here.
  println!("The first element is: {}", first);

  let mut s: String = String::from("Hello, World!");

  for mut c in &mut s.chars() {
    // Do something.
    if let Some(c_num) = c.to_digit(10) {
      let incremented: u32 = c_num + 1;
      c = char::from_u32(incremented).unwrap();
    } else {
      eprintln!("NO!");
    }
  }

  s = s.replace("!", "?");

  println!("{}", s);

  let mut bytes: Vec<u8> = s.into_bytes();
  bytes[0] = 'A' as u8;

  unsafe {
    s = String::from_utf8_unchecked(bytes);
  }

  println!("{}", s);

  use std::collections::HashMap;

  let countries: Vec<String> = vec![String::from("CN"), String::from("US")];
  let id: Vec<u32> = vec![1, 2];
  let mut dict: HashMap<String, u32> = countries.into_iter().zip(id.into_iter()).collect();

  let key: String = String::from("foo");
  let mut val: String = String::from("bar");

  // Error! Ownership is transferred to dict.
  // dict.insert(key, val);
  // println!("{}: {}", key, val);

  // OK! Map stores the reference to the object.
  let mut map: HashMap<&String, &mut String> = HashMap::new();
  map.insert(&key, &mut val);

  if let Some(val) = dict.get(&String::from("foo")) {
    println!("{}", val);
  } else {
    println!("No such thing!");
  }

  let mut baz: String = String::from("baz");
  let mut baz_clone: String = baz.clone();
  map.insert(&key, &mut baz);
  map.entry(&key).or_insert(&mut baz_clone);

  println!("{:?}", map);

  let mut word_count: HashMap<String, u32> = HashMap::new();
  let sentence: &str = "Oh my lord! What a beautiful world it is is is!";
  let words: std::str::SplitWhitespace = sentence.split_whitespace();

  for word in words {
    let count = word_count.entry(word.to_string()).or_insert(0);
    *count += 1;
  }

  println!("{:?}", word_count);
}
struct Server {
  addr: String,
  port: u8,
  common_name: Option<String>,
  id: u8,
}

impl Server {
  pub fn init_server(addr: &str, port: &u8, common_name: &Option<String>) -> Server {
    Server {
      addr: addr.to_string(),
      port: *port,
      common_name: common_name.clone(),
      id: 123,
    }
  }
}
