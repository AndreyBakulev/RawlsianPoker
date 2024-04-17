mod card;
mod deck;
mod player;
mod table;
use crate::player::Player;
use crate::table::Table;

fn main() {
    let mut table = Table::new("Table 1");
    let mut p1 = Player::new("Andrey");
    table.start_game();
    //println!("Shuffled Deck:\n{}", table.deck);
    while p1.hand.len() < 5{
        p1.draw(&mut table.deck);
    }
    println!("{}'s Hand:\n{}",p1.id,p1);
    let hand_rank = p1.evaluate_hand();
    println!("Hand Rank:\n{:?}", hand_rank);
}
/*

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