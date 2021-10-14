use super::card:: { Card };

use std::fmt;
// Shuffle vectors
use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct CardSet {
  pub cards: Vec<Card>
}

impl CardSet {
  pub fn new() -> Self {
    CardSet { cards: Vec::new() }
  }
  pub fn add(&mut self, card: Card) {
    self.cards.push(card);
  }
  pub fn shuffle(&mut self) {
    self.cards.shuffle(&mut thread_rng());
  }
  pub fn deal(&mut self, to: &mut CardSet, count: u8) {
    for _ in 0..count {
      if let Some(card) = self.cards.pop() {
        to.add(card);
      }
    }
  }
}

impl fmt::Debug for CardSet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_list()
      .entries(self.cards.iter())
      .finish()
  }
}
