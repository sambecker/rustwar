use super::card::{ Card, Suit, Rank };

use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

pub struct Deck {
  pub cards: Vec<Card>
}

impl Deck {
  pub fn new() -> Self {
    let mut cards = Vec::new();
  
    for suit in Suit::iter() {
      for rank in Rank::iter() {
        let card = Card { suit: suit, rank: rank };
        cards.push(card)
      }
    }
  
    Deck { cards: cards }
  }

  pub fn shuffle(&mut self) {
    self.cards.shuffle(&mut thread_rng())
  }
}

impl fmt::Debug for Deck {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "{}", self.cards[0])
  }
}
