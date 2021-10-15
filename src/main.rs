mod simulation;

use simulation::Simulation;

const GROUP_LENGTH: usize = 2;
const BATCH_LENGTH: usize = 10_000;
const MAX_GAME_LENGTH_IN_TURNS: usize = 10_000;

fn main() {
  let mut sim = Simulation::new(
    "Minnie",
    "Sam",
    MAX_GAME_LENGTH_IN_TURNS,
  );

  println!(
    "STARTING SIMULATION (MAX GAME LENGTH: {})",
    MAX_GAME_LENGTH_IN_TURNS,
  );

  // With shuffling win piles
  for _ in 0..GROUP_LENGTH {
    let (average, indeterminate) = sim.run_batch(BATCH_LENGTH, true, false);
    println!(
      "{} Games / Shuffling: YES {:.2} (Indeterminate games: {})",
      BATCH_LENGTH,
      average,
      indeterminate,
    );
  }

  // Without shuffling win piles
  for _ in 0..GROUP_LENGTH {
    let (average, indeterminate) = sim.run_batch(BATCH_LENGTH, false, false);
    println!(
      "{} Games / Shuffling: NO  {:.2} (Indeterminate games: {})",
      BATCH_LENGTH,
      average,
      indeterminate,
    );
  } 
}
