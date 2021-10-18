mod card;
mod card_set;
mod player;
mod game;
mod progress;

use game::Game;
use progress::Progress;

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
    let mut game = Game::new(
      &self.name_one,
      &self.name_two,
      should_shuffle_win_pile
    );
    game.deal_all_cards(true);

    let (turns_total, did_game_end_prematurely) = game.complete(
      self.max_game_length_in_turns,
      debug,
    );

    if did_game_end_prematurely {
      self.games_ended_prematurely += 1;
      if debug {
        println!("Game ended prematurely @ turn {}", turns_total);
      }
    } else {
      self.results.push(SimulationResult {
        length_in_turns: turns_total,
        winner_name: String::from(&game.player_one.name),
      });
      if debug {
        println!("Game finished @ turn {}", turns_total);
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
    progress: &mut Progress,
    debug: bool,
  ) -> (f32, f32) {
    for _ in 0..times {
      progress.tick();
      self.run(should_shuffle_win_pile, debug);
    }
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
      let label = "Games: ";
      let mut progress = Progress::new(
        batch_length,
        label,
        Some(80), 
        Some(250),
      );
      let (average, indeterminate) = self.run_batch(
        batch_length,
        shuffle,
        &mut progress,
        debug,
      );
      progress.finish(&format!(
        "{}{} [{:10}] {:.2} (Indeterminate games: {:.2}%)",
        label,
        batch_length,
        if shuffle { "SHUFFLED" } else { "UNSHUFFLED" },
        average,
        indeterminate,
      ));
    }
  }
}
