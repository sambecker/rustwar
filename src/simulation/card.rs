use std::fmt;
use std::iter::Iterator;
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

#[derive(Debug, Clone, Copy, EnumIter, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Clone, Copy)]
pub struct Card {
  pub suit: Suit,
  pub rank: Rank,
}

impl Card {
  pub fn new(suit: Suit, rank: Rank) -> Self {
    Card { suit: suit, rank: rank }
  }
}

impl Ord for Card {
  fn cmp(&self, other: &Self) -> Ordering {
      self.rank.cmp(&other.rank)
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

impl Eq for Card {}

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

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn card_compare() {
    // Equal
    assert_eq!(
      Card { suit: Suit::Club, rank: Rank::Ace } ==
      Card { suit: Suit::Diamond, rank: Rank::Ace },
      true,
    );
    // Greater than
    assert_eq!(
      Card { suit: Suit::Club, rank: Rank::Ace } >
      Card { suit: Suit::Diamond, rank: Rank::King },
      true,
    );
    // Less than
    assert_eq!(
      Card { suit: Suit::Club, rank: Rank::Four } <
      Card { suit: Suit::Diamond, rank: Rank::Five },
      true,
    );
  }
}
