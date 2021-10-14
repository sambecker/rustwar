pub mod card;
pub mod card_set;
pub mod player;

use std::cmp::{ Ordering };

pub use card::{ Card, Suit, Rank };
pub use card_set::{ CardSet };
pub use player::{ Player };

use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Game {
  pub deck: CardSet,
  pub player_one: Player,
  pub player_two: Player,
}

impl Game {
  pub fn new(name_one: &str, name_two: &str) -> Self {
    let mut deck = CardSet::new();
  
    for suit in Suit::iter() {
      for rank in Rank::iter() {
        deck.add(Card::new(suit, rank))
      }
    }

    deck.shuffle();

    Game {
      deck: deck,
      player_one: Player::new(String::from(name_one)),
      player_two: Player::new(String::from(name_two)),
    }
  }
  pub fn deal(&mut self) {
    self.deck.deal(&mut self.player_one.pile_new, 26);
    self.deck.deal(&mut self.player_two.pile_new, 26);
  }
  pub fn tick(&mut self) -> bool {
    let mut result = false;
    if let Some(card_one) = self.player_one.pile_new.cards.pop() {
      if let Some(card_two) = self.player_two.pile_new.cards.pop() {
        result = true;
        match card_one.cmp(&card_two) {
          Ordering::Less => println!(
            "{} lost to {} ({:?} vs {:?})",
            self.player_one.name, self.player_two.name, card_one, card_two,
          ),
          Ordering::Greater => println!(
            "{} beat {} ({:?} vs {:?})",
            self.player_one.name, self.player_two.name, card_one, card_two,
          ),
          Ordering::Equal => println!(
            "WAR => {:?} tied {:?}",
            card_one, card_two
          ),
        }
      }
    }
    result
  }
}
