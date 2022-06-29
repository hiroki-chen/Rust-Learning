use rand::seq::SliceRandom;
use rand::thread_rng;
use crypto;
use crypto::digest::Digest;
use std::fmt;
use std::fmt::Formatter;
use itertools::{concat, Itertools};

const SPLIT_POINT_FOUR: usize = 56;
const SPLIT_POINT_THREE: usize = 42;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hai {
  M(u8, bool),
  S(u8, bool),
  P(u8, bool),
  Z(u8),
}

pub struct InHand<'a>(&'a Vec<Hai>);

pub struct AllHand<'a>(&'a Vec<Vec<Hai>>);

impl fmt::Display for Hai {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Hai::M(val, aka) => write!(f, "{}m", if *aka { &0 } else { val }),
      Hai::P(val, aka) => write!(f, "{}p", if *aka { &0 } else { val }),
      Hai::S(val, aka) => write!(f, "{}s", if *aka { &0 } else { val }),
      Hai::Z(val) => write!(f, "{}z", val),
    }
  }
}

impl fmt::Display for InHand<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0.iter()
      .map(|x| x.to_string())
      .collect::<Vec<String>>()
      .join(""))
  }
}

impl fmt::Display for AllHand<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let mut serial = 0;
    for hand in self.0.into_iter() {
      write!(f, "#{}: {}\n", serial, InHand(hand))?;
      serial += 1;
    }

    Ok(())
  }
}

pub fn generate(is_four_player: bool) -> (Vec<Hai>, Vec<Vec<Hai>>) {
  let mut vec: Vec<Hai> = Vec::new();
  // Handle normal hai.
  generate_initial_hai(&mut vec, is_four_player);
  vec.shuffle(&mut thread_rng());

  let md5 = generate_md5(&vec);
  println!("MD5 for this tile is {}", md5);

  // Generate all the Hai's in the hand.
  let (player_hai, tiles) = vec.split_at(
    if is_four_player {
      SPLIT_POINT_FOUR
    } else {
      SPLIT_POINT_THREE
    }
  );

  let mut players: Vec<Vec<Hai>> = player_hai
    .to_vec()
    .into_iter()
    .chunks(14)
    .into_iter()
    .map(|x| x.collect())
    .collect();

  // Finally, sort the tiles in hands.
  players.iter_mut().for_each(|x| x.sort());

  println!("Tile mountain: {}\n Player:\n{}", InHand(&tiles.to_vec()), AllHand(&players));

  (tiles.to_vec(), players)
}

fn generate_md5(vec: &Vec<Hai>) -> String {
  let mut md5_generator = crypto::md5::Md5::new();
  md5_generator.input(InHand(vec).to_string().as_bytes());
  md5_generator.result_str()
}

fn generate_initial_hai(vec: &mut Vec<Hai>, is_four_player: bool) {
  // Handle normal hai.
  for i in 1..=9 {
    if i != 5 {
      vec.append(&mut vec![Hai::P(i, false)].repeat(4));
      vec.append(&mut vec![Hai::S(i, false)].repeat(4));
    } else {
      vec.push(Hai::S(5, true));
      vec.push(Hai::P(5, true));

      // Normal 5
      vec.append(&mut vec![Hai::S(5, false)].repeat(3));
      vec.append(&mut vec![Hai::P(5, false)].repeat(3));
    }
  }

  // Add manzu.
  if is_four_player {
    for i in 1..=9 {
      if i != 5 {
        vec.append(&mut vec![Hai::M(i, false)].repeat(4));
      } else {
        vec.push(Hai::M(5, true));
        vec.append(&mut vec![Hai::M(5, false)].repeat(3));
      }
    }
  } else {
    // 1 and 9m.
    let mut manzu = [[Hai::M(1, false)].repeat(4), [Hai::M(9, false)].repeat(4)].concat();
    vec.append(&mut manzu);
  }

  for i in 1..=7 {
    vec.append(&mut vec![Hai::Z(i)].repeat(4));
  }
}