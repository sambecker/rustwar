mod game;

use game::Deck;

fn main() {
  let mut deck = Deck::new();
  deck.shuffle();
  println!("{:?}", deck);
}
