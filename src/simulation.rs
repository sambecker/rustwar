mod card;
mod card_set;
mod player;
mod game;

use game::Game;

#[derive(Debug)]
pub struct Simulation {
  name_one: String,
  name_two: String,
  max_game_length_in_turns: usize,
  games_ended_prematurely: usize,
  pub results: Vec<SimulationResult>,
}

#[derive(Debug)]
pub struct SimulationResult {
  pub length_in_turns: usize,
  pub winner_name: String,
}

impl Simulation {
  pub fn new(
    name_one: &str,
    name_two: &str,
    max_game_length_in_turns: usize,
  ) -> Self {
    Simulation {
      name_one: String::from(name_one),
      name_two: String::from(name_two),
      max_game_length_in_turns: max_game_length_in_turns,
      games_ended_prematurely: 0,
      results: Vec::new(),
    }
  }
  fn reset(&mut self) {
    self.results = Vec::new();
    self.games_ended_prematurely = 0;
  }
  // Run one simulation
  fn run(&mut self, should_shuffle_win_pile: bool, debug: bool) {
    let mut game = Game::new("Sam", "Minnie", should_shuffle_win_pile);
    game.deal();
    for i in 0..self.max_game_length_in_turns {
      let player_one_total = game.player_one.card_count();
      let player_two_total = game.player_two.card_count();
      match game.tick(debug) {
        true => {
          if debug {
            println!(
              "TICK: {}({:02}) + {}({:02}) = {}",
              game.player_one.name,
              player_one_total,
              game.player_two.name,
              player_two_total,
              player_one_total + player_two_total,
            );
          }
        },
        false => {
          self.results.push(SimulationResult {
            length_in_turns: i,
            winner_name: String::from(&game.player_one.name),
          });
          if debug {
            println!("No more cards left (Round {})", i);
          }
          break;
        },
      }
      if i == self.max_game_length_in_turns - 1 {
        self.games_ended_prematurely += 1;
        if debug {
          println!("GAME ENDED PREMATURELY @ {} TURNS", i + 1);
        }
      }
    }
    if debug {
      println!("{:#?}", game);
    }
  }
  // Run batch (of simulations)
  pub fn run_batch(
    &mut self,
    times: usize,
    should_shuffle_win_pile: bool,
    debug: bool,
  ) -> (f32, f32) {
    for _ in 0..times { self.run(should_shuffle_win_pile, debug); }
    let average: f32 = self.results
      .iter()
      .map(|result| result.length_in_turns as f32)
      .sum();
    let average_length = average / self.results.len() as f32;
    let percent_indeterminant =
      100_f32 *
      self.games_ended_prematurely as f32 /
      self.results.len() as f32;
    (average_length, percent_indeterminant)
  }
  // Run set (of batches of simulations)
  pub fn run_batch_set(
    &mut self,
    set_length: usize,
    batch_length: usize,
    shuffle: bool,
    debug: bool,
  ) {
    for _ in 0..set_length {
      self.reset();
      let (average, indeterminate) = self.run_batch(
        batch_length,
        shuffle,
        debug,
      );
      println!(
        "{} Games / Shuffling: {:3} {:.2} (Indeterminate games: {:.2}%)",
        batch_length,
        if shuffle { "YES" } else { "NO" },
        average,
        indeterminate,
      );
    }
  }
}
