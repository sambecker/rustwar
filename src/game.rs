pub mod card;
pub mod card_set;

pub use card::{ Card, Suit, Rank };
pub use card_set::{ CardSet };

use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Game {
  pub deck: CardSet,
  pub hand_one: CardSet,
  pub hand_two: CardSet,
}

impl Game {
  pub fn new() -> Self {
    let mut deck = CardSet::new();
  
    for suit in Suit::iter() {
      for rank in Rank::iter() {
        deck.add(Card::new(suit, rank))
      }
    }
    deck.shuffle();

    Game {
      deck: deck,
      hand_one: CardSet::new(),
      hand_two: CardSet::new(),
    }
  }
  pub fn deal(&mut self) {
    self.deck.deal(&mut self.hand_one, 26);
    self.deck.deal(&mut self.hand_two, 26);
  }
}
