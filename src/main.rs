mod game;

use game::Game;

fn main() {
  let mut game = Game::new("Sam", "Minnie");
  game.deal();
  for i in 1..10_000 {
    match game.tick(false) {
      true => {
        let player_one_total = game.player_one.card_count();
        let player_two_total = game.player_two.card_count();
        println!(
          "TICK: {}({:02}) + {}({:02}) = {}",
          game.player_one.name,
          player_one_total,
          game.player_two.name,
          player_two_total,
          player_one_total + player_two_total,
        )
      },
      false => {
        println!("No more cards left (Round {})", i);
        break;
      },
    }
  }
  println!("{:#?}", game);
}
