mod card;
mod deck;
mod player;
use crate::player::Player;

fn main() {
    let mut p1 = Player::new();
    deck::Deck::shuffle();
    //println!("Shuffled Deck:\n{}", *DECK.lock().unwrap());
    while p1.hand.len() < 5{
        p1.draw();
    }
    println!("Player 1's Hand:\n{}",p1);
}
/*
TODO:
add a player class
each player has an array of Traits
each trait affects the total money
make a card class that has a shuffle method and yea

GAME LOOP{
user joins a table
in the table, there are up to _ users
each player has an array that holds 5 cards + a traits arrayList
each player draws a card and sees if they have a combination (if statements?)

}
STEPS {
1. normal poker
2. adding traits
3. GUI
4. multiplayer?
}
 */