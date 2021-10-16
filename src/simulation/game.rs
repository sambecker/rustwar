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
      player_one: Player::new(String::from(name_one)),
      player_two: Player::new(String::from(name_two)),
      should_shuffle_win_pile: should_shuffle_win_pile,
    }
  }
  pub fn deal(&mut self) {
    self.deck.deal(&mut self.player_one.pile_new, 26);
    self.deck.deal(&mut self.player_two.pile_new, 26);
  }
  pub fn tick(&mut self, debug: bool) -> bool {
    let mut does_game_continue = false;
    let mut winnings: Vec<Card> = Vec::new();
    if let Some(card_one) = self.player_one.draw_card(
      self.should_shuffle_win_pile) {
      if let Some(card_two) = self.player_two.draw_card(
        self.should_shuffle_win_pile) {
        winnings.push(card_one);
        winnings.push(card_two);
        match card_one.cmp(&card_two) {
          Ordering::Greater => {
            if debug {
              println!(
                "{} beat {} ({:?} vs {:?})",
                self.player_one.name, self.player_two.name, card_one, card_two,
              );
            }
            self.player_one.win_cards(&mut winnings);
            does_game_continue = true;
          },
          Ordering::Less => {
            if debug {
              println!(
                "{} lost to {} ({:?} vs {:?})",
                self.player_one.name, self.player_two.name, card_one, card_two,
              );
            }
            self.player_two.win_cards(&mut winnings);
            does_game_continue = true;
          },
          Ordering::Equal => {
            if debug {
              println!("WAR Start! {:?} vs {:?}", card_one, card_two);
            }
            // THE WAR ZONE
            loop {
              let mut does_war_repeat = false;
              for i in 0..=WAR_LENGTH {
                does_game_continue = false;
                if let Some(card_one) = self.player_one.draw_card(
                  self.should_shuffle_win_pile) {
                  if let Some(card_two) = self.player_two.draw_card(
                    self.should_shuffle_win_pile) {
                    winnings.push(card_one);
                    winnings.push(card_two);
                    if i == WAR_LENGTH {
                      if debug {
                        println!("WAR pot should be 8 or more: {}", winnings.len());
                      }
                      match card_one.cmp(&card_two) {
                        Ordering::Greater => {
                          if debug {
                            println!("War winner: {}", self.player_one.name);
                          }
                          self.player_one.win_cards(&mut winnings);
                        }
                        Ordering::Less => {
                          if debug {
                            println!("War winner: {}", self.player_two.name);
                          }
                          self.player_two.win_cards(&mut winnings);
                        }
                        Ordering::Equal => {
                          if debug {
                            println!("WAR tie! {:?} vs {:?}", winnings[0], winnings[1]);
                          }
                          does_war_repeat = true;
                        },
                      }
                    }
                    does_game_continue = true;
                  }
                }
                if !does_game_continue {
                  if debug {
                    println!("Returning final cards to player with more");
                  }
                  if winnings.len() > 0 {
                    match self.player_one.card_count().cmp(&self.player_two.card_count()) {
                      Ordering::Greater => self.player_one.win_cards(&mut winnings),
                      Ordering::Less => self.player_two.win_cards(&mut winnings),
                      _ => (),
                    }
                  }
                  break;
                }
              }
              if !does_war_repeat { break; }
            }
          }
        }
      }
    }
    does_game_continue
  }
}
