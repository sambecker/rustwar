use std::fmt;
// Iterate through enums
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Suit {
  Heart,
  Diamond,
  Club,
  Spade,
}

#[derive(Debug, EnumIter)]
pub enum Rank {
  Ace,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
}

pub struct Card {
  pub suit: Suit,
  pub rank: Rank,
}

impl Card {
  pub fn new(suit: Suit, rank: Rank) -> Self {
    Card { suit: suit, rank: rank }
  }
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?} of {:?}s", self.rank, self.suit)
  }
}

impl fmt::Debug for Card {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}
