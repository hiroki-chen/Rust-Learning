// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

// This is an example of using pre/post conditions as well as invariants
// via the contracts crate.
//
// We define a simple shopping cart with an invariant both on items in the cart
// and the overall cart.
//
use contracts::*;
use mirai_annotations::*;
use std::rc::Rc;

// Items stored in the shopping cart.
#[allow(dead_code)]
pub struct Item {
  name: Rc<str>,
  price: u64,
}

impl Item {
  // Invariant for the Item struct. We define those invariants via functions
  // so they can easily be referenced from attributes.
  // fn invariant(&self) -> bool {
  //     !self.name.is_empty() && self.price > 0
  // }

  // Creates a new Item, satisfying the invariant.
  #[requires(!name.is_empty() && price > 0)]
  //#[ensures(ret.invariant())]
  pub(crate) fn new(name: &str, price: u64) -> Item {
    Item {
      name: Rc::from(name),
      price,
    }
  }
}

// The shopping cart, consisting of a list of items, and the total price for
// all the items in the cart.
pub struct ShoppingCart {
  items: Vec<Item>,
  total: u64,
}

impl ShoppingCart {
  // Invariant for the shopping cart.
  // Note that MIRAI cannot currently prove that such invariants hold and that this is not
  // likely to change any time soon.
  // fn invariant(&self) -> bool {
  //     self.items.iter().all(|x| x.invariant())
  //         && self.items.iter().map(|x| x.price).sum::<u64>() == self.total
  // }

  //#[ensures(ret.invariant())]
  pub(crate) fn new() -> ShoppingCart {
    ShoppingCart {
      items: vec![],
      total: 0,
    }
  }
}

// Implements methods of the shopping cart. By attaching the `#[invariant]` attribute
// all methods in the impl which work on `self` get automatically injected
// the invariant both as pre and post condition, in addition to what they state
// individually.
//#[invariant(self.invariant())]
impl ShoppingCart {
  #[requires(self.total <= std::u64::MAX - item.price && self.items.len() < std::usize::MAX)]
  pub fn add(&mut self, item: Item) {
    self.total += item.price;
    self.items.push(item);
  }

  pub fn add_broken_invariant(&mut self, item: Item) {
    // The below should cause a diagnostic because the invariant is violated.
    self.items.push(item);
  }

  pub fn checkout(&mut self) -> u64 {
    let bill = self.total;
    self.total = 0;
    self.items.clear();
    bill
  }
}
