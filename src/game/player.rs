use super::card_set:: { CardSet };

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
}
