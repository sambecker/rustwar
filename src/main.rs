mod game;

use game::Game;

fn main() {
  let mut game = Game::new("Sam", "Minnie");
  game.deal();
  for i in 1..100 {
    match game.tick() {
      true => (),
      false => println!("Game ended (Round {})", i),
    }
  }
}
