use std::fmt;
use std::cmp::{ Eq, Ordering };
// Iterate through enums
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Suit {
  Heart,
  Diamond,
  Club,
  Spade,
}

#[derive(Debug, EnumIter, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
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
  Ace,
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

impl PartialOrd for Card {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.rank.cmp(&other.rank))
  }
}

impl PartialEq for Card {
  fn eq(&self, other: &Self) -> bool {
      self.rank == other.rank
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
