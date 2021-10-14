use super::card::{ Card };
use super::card_set::{ CardSet };

use std::mem;

#[derive(Debug)]
pub struct Player {
  pub name: String,
  pub pile_new: CardSet,
  pub pile_won: CardSet,
}

impl Player {
  pub fn new(name: String) -> Self {
    Player {
      name: name,
      pile_new: CardSet::new(),
      pile_won: CardSet::new(),
    }
  }
  pub fn card_count(&self) -> usize {
    self.pile_new.cards.len() + self.pile_won.cards.len()
  }
  pub fn draw_card(&mut self) -> Option<Card> {
    if let Some(card) = self.pile_new.cards.pop() {
      // Card exists in new pile
      Some(card)
    } else {
      if self.pile_won.cards.len() > 0 {
        // Card exists in won pile
        mem::swap(&mut self.pile_new, &mut self.pile_won);
        self.pile_won = CardSet::new();
        self.pile_new.shuffle();
        self.pile_new.cards.pop()
      } else {
        None
      }
    }
  }
  pub fn win_cards(&mut self, cards: &mut Vec<Card>) {
    self.pile_won.cards.append(cards);
  }
}
