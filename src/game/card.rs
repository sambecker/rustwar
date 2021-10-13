use std::fmt;
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

#[derive(Debug)]
pub struct Card {
  pub suit: Suit,
  pub rank: Rank,
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?} of {:?}s", self.rank, self.suit)
  }
}
