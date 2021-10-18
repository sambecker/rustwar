use std::cmp::{ Ordering };

use super::card::{ Card, Suit, Rank };
use super::card_set::{ CardSet };
use super::player::{ Player };

use strum::IntoEnumIterator;

const WAR_LENGTH:usize = 3;

#[derive(Debug)]
pub struct Game {
  pub deck: CardSet,
  pub player_one: Player,
  pub player_two: Player,
  pub should_shuffle_win_pile: bool,
}

impl Game {
  pub fn new(
    name_one: &str,
    name_two: &str,
    should_shuffle_win_pile: bool,
  ) -> Self {
    let mut deck = CardSet::new();

    for suit in Suit::iter() {
      for rank in Rank::iter() {
        deck.add(Card::new(suit, rank))
      }
    }

    deck.shuffle();

    Game {
      deck: deck,
      player_one: Player::new(name_one),
      player_two: Player::new(name_two),
      should_shuffle_win_pile: should_shuffle_win_pile,
    }
  }
  pub fn deal_all_cards(&mut self, all_at_once: bool) {
    if all_at_once {
      let half_deck_length = self.deck.cards.len() / 2;
      self.deck.deal_cards(&mut self.player_one.pile_new, half_deck_length);
      self.deck.deal_cards(&mut self.player_two.pile_new, half_deck_length);
    } else {
      while self.deck.cards.len() > 0 {
        self.deck.deal_card(&mut self.player_one.pile_new);
        self.deck.deal_card(&mut self.player_two.pile_new);
      }
    }
  }
  fn both_players_have_cards(&self) -> bool {
    self.player_one.has_cards() && self.player_two.has_cards()
  }
  fn take_turns_recursive(
    &mut self,
    number_of_cards_to_skip: usize,
    number_of_turns_taken: &mut usize,
    is_game_over: &mut bool,
    winnings: &mut Vec<Card>,
    debug: bool,
  ) {
    for i in 0..=number_of_cards_to_skip {
      if self.both_players_have_cards() {
        if let Some(card_one) = self.player_one.draw_card(
          self.should_shuffle_win_pile) {
          winnings.push(card_one);
          if let Some(card_two) = self.player_two.draw_card(
            self.should_shuffle_win_pile) {
            winnings.push(card_two);
            if i == number_of_cards_to_skip {
              match card_one.cmp(&card_two) {
                Ordering::Greater => {
                  if debug {
                    println!(
                      "{} beat {} ({:?} vs {:?})",
                      self.player_one.name,
                      self.player_two.name,
                      card_one,
                      card_two,
                    );
                  }
                  self.player_one.win_cards(winnings);
                  *number_of_turns_taken += 1;
                  *is_game_over = !self.both_players_have_cards();
                },
                Ordering::Less => {
                  if debug {
                    println!(
                      "{} lost to {} ({:?} vs {:?})",
                      self.player_one.name,
                      self.player_two.name,
                      card_one,
                      card_two,
                    );
                  }
                  self.player_two.win_cards(winnings);
                  *number_of_turns_taken += 1;
                  *is_game_over = !self.both_players_have_cards();
                },
                Ordering::Equal => {
                  if debug {
                    println!("WAR Start! {:?} vs {:?}", card_one, card_two);
                  }
                  self.take_turns_recursive(
                    WAR_LENGTH,
                    number_of_turns_taken,
                    is_game_over,
                    winnings,
                    debug,
                  )
                }
              }
            }
          }
        }
      } else {
        *is_game_over = true;
        if winnings.len() > 0 {
          let player_one_wins = self.player_one.has_cards()
            && !self.player_two.has_cards();
          let player_two_wins = self.player_two.has_cards()
            && !self.player_one.has_cards();
          if player_one_wins {
            self.player_one.win_cards(winnings);
          } else if player_two_wins {
            self.player_two.win_cards(winnings);
          } else {
            if debug { println!("DRAW GAME") ;}
          }
        }
        break;
      }
    }
  }
  fn tick(&mut self, debug: bool) -> (usize, bool) {
    let mut turns = 0;
    let mut is_game_over = false;
    let mut winnings: Vec<Card> = Vec::new();

    self.take_turns_recursive(
      0,
      &mut turns,
      &mut is_game_over,
      &mut winnings,
      debug,
    );

    (turns, is_game_over)
  }
  pub fn complete(&mut self, turn_limit: usize, debug: bool) -> (usize, bool) {
    let mut turns_total = 0;
    let mut did_game_end_prematurely;
    loop {
      let (turns, is_game_over) = self.tick(debug);
      turns_total += turns;
      did_game_end_prematurely = turns_total > turn_limit;
      if is_game_over || did_game_end_prematurely { break; }
    }
    if !did_game_end_prematurely
      && self.player_one.has_cards()
      && self.player_two.has_cards() {
      panic!(
        "Game did not end properly! {} vs {}",
        self.player_one.card_count(),
        self.player_two.card_count(),
      );
    }
    (turns_total, did_game_end_prematurely)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_turn() {
    let mut deck = CardSet::new();

    // 2x Cards Per Player (dealt from the top)
    deck.add(Card { suit: Suit::Diamond, rank: Rank::Two });  // P2
    deck.add(Card { suit: Suit::Heart, rank: Rank::Jack });   // P1
    deck.add(Card { suit: Suit::Diamond, rank: Rank::Six });  // P2
    deck.add(Card { suit: Suit::Heart, rank: Rank::Ace });    // P1 (winner)

    let mut game = Game {
      deck: deck,
      player_one: Player::new("Player One"),
      player_two: Player::new("Player Two"),
      should_shuffle_win_pile: true,
    };
    game.deal_all_cards(true);

    assert_eq!(game.player_one.pile_new.cards.len(), 2);
    assert_eq!(game.player_two.pile_new.cards.len(), 2);

    let (turns_total, _) = game.complete(10_000, false);
    
    assert_eq!(turns_total, 2);

    assert_eq!(game.player_one.card_count(), 4);
    assert_eq!(game.player_two.card_count(), 0);
  }
  
  #[test]
  fn single_war() {
    let mut deck = CardSet::new();

    // 5x Cards Per Player (dealt from the top)
    deck.add(Card { suit: Suit::Diamond, rank: Rank::Six });   // P2
    deck.add(Card { suit: Suit::Heart, rank: Rank::Six });     // P1
    deck.add(Card { suit: Suit::Diamond, rank: Rank::Seven }); // P2
    deck.add(Card { suit: Suit::Heart, rank: Rank::Seven });   // P1
    deck.add(Card { suit: Suit::Diamond, rank: Rank::Eight }); // P2
    deck.add(Card { suit: Suit::Heart, rank: Rank::Eight });   // P1
    deck.add(Card { suit: Suit::Diamond, rank: Rank::Nine });  // P2
    deck.add(Card { suit: Suit::Heart, rank: Rank::Nine });    // P1
    deck.add(Card { suit: Suit::Diamond, rank: Rank::Ten });   // P2
    deck.add(Card { suit: Suit::Heart, rank: Rank::Jack });    // P1 (winner)

    let mut game = Game {
      deck: deck,
      player_one: Player::new("Player One"),
      player_two: Player::new("Player Two"),
      should_shuffle_win_pile: true,
    };
    game.deal_all_cards(false);

    assert_eq!(game.player_one.pile_new.cards.len(), 5);
    assert_eq!(game.player_two.pile_new.cards.len(), 5);

    let mut turns_total = 0;
    loop {
      let (turns, is_game_over) = game.tick(false);
      turns_total += turns;
      if is_game_over { break; }
    }
    assert_eq!(turns_total, 1);

    assert_eq!(game.player_one.card_count(), 10);
    assert_eq!(game.player_two.card_count(), 0);
  }
  
  #[test]
  fn double_war() {
    let mut player_one = Player::new("Player One");
    let mut cards = vec![
      Card { suit: Suit::Diamond, rank: Rank::Jack },
      Card { suit: Suit::Diamond, rank: Rank::Ten },
      Card { suit: Suit::Diamond, rank: Rank::Nine },
      Card { suit: Suit::Diamond, rank: Rank::Eight },
      Card { suit: Suit::Diamond, rank: Rank::Seven },
      Card { suit: Suit::Diamond, rank: Rank::Six },
      Card { suit: Suit::Diamond, rank: Rank::Five },
      Card { suit: Suit::Diamond, rank: Rank::Four },
      Card { suit: Suit::Diamond, rank: Rank::Three },
    ];
    player_one.pile_new.cards.append(&mut cards);

    let mut player_two = Player::new("Player Two");
    cards = vec![
      Card { suit: Suit::Heart, rank: Rank::Queen },
      Card { suit: Suit::Heart, rank: Rank::Ten },
      Card { suit: Suit::Heart, rank: Rank::Nine },
      Card { suit: Suit::Heart, rank: Rank::Eight },
      Card { suit: Suit::Heart, rank: Rank::Seven },
      Card { suit: Suit::Heart, rank: Rank::Six },
      Card { suit: Suit::Heart, rank: Rank::Five },
      Card { suit: Suit::Heart, rank: Rank::Four },
      Card { suit: Suit::Heart, rank: Rank::Three },
    ];
    player_two.pile_new.cards.append(&mut cards);

    let mut game = Game {
      deck: CardSet::new(),
      player_one: player_one,
      player_two: player_two,
      should_shuffle_win_pile: true,
    };

    let mut turns_total = 0;
    loop {
      let (turns, is_game_over) = game.tick(false);
      turns_total += turns;
      if is_game_over { break; }
    }
    assert_eq!(turns_total, 1);

    assert_eq!(game.player_one.card_count(), 0);
    assert_eq!(game.player_two.card_count(), 18);
  }
}
