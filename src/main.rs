mod simulation;

use std::time::{ SystemTime };

use simulation::Simulation;

const SET_LENGTH: usize  = 2;
const BATCH_LENGTH: usize = if cfg!(debug_assertions)
  { 20_000 } else
  { 100_000 };
const MAX_GAME_LENGTH_IN_TURNS: usize = 10_000;

fn main() {
  let mut sim = Simulation::new(
    "Player One",
    "Player Two",
    MAX_GAME_LENGTH_IN_TURNS,
  );

  println!(
    "STARTING SIMULATION (MAX GAME LENGTH: {} TURNS)",
    MAX_GAME_LENGTH_IN_TURNS,
  );

  let now =  SystemTime::now();
  
  // With shuffling win piles
  sim.run_batch_set(SET_LENGTH, BATCH_LENGTH, true, false);

  // Without shuffling win piles
  sim.run_batch_set(SET_LENGTH, BATCH_LENGTH, false, false);

  if let Ok(elapsed) = now.elapsed() {
    println!("SIMULATION FINISHED ({} SECONDS)", elapsed.as_secs());
  }
}
