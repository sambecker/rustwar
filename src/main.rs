mod simulation;

use std::time::{ SystemTime };
use std::thread;
use std::thread::JoinHandle;

use simulation::Simulation;
use simulation::progress_multi::ProgressMulti;

const SET_LENGTH: usize  = 3;
const BATCH_LENGTH: usize = if cfg!(debug_assertions)
  { 20_000 } else
  { 100_000 };
const MAX_GAME_LENGTH_IN_TURNS: usize = 10_000;

fn main() {
  println!(
    "STARTING SIMULATION (MAX GAME LENGTH: {} TURNS)",
    MAX_GAME_LENGTH_IN_TURNS,
  );

  let now =  SystemTime::now();

  let bars = ProgressMulti::new(None, None);

  let mut threads: Vec<JoinHandle<()>> = vec![];

  for should_shuffle in [true, false] {
    for i in 0..SET_LENGTH {
      let label = if should_shuffle
        { format!("Games [{:02} Shuffled  ]: ", i) } else
        { format!("Games [{:02} Unshuffled]: ", i) };
      let mut bar_callback = bars.new_bar(&label, BATCH_LENGTH);
      threads.push(thread::spawn(move || {
        let mut sim = Simulation::new(MAX_GAME_LENGTH_IN_TURNS);
        sim.run_batch_with_bars(
          &mut bar_callback,
          &label,
          BATCH_LENGTH,
          should_shuffle,
          false,
        );
      }));
    }
  }

  bars.start();

  while let Some(thread) = threads.pop() {
    thread.join().unwrap();
  }

  if let Ok(elapsed) = now.elapsed() {
    println!(
      "SIMULATION FINISHED ({:.1} SECONDS)",
      elapsed.as_millis() as f32 / 1000_f32,
    );
  }
}
