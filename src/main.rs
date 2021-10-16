mod simulation;

use simulation::Simulation;

const SET_LENGTH: usize  = 2;
const BATCH_LENGTH: usize = 20_000;
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
  
  // With shuffling win piles
  sim.run_batch_set(SET_LENGTH, BATCH_LENGTH, true, false);

  // Without shuffling win piles
  sim.run_batch_set(SET_LENGTH, BATCH_LENGTH, false, false);
}
