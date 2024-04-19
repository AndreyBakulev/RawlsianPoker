mod card;
mod deck;
mod player;
mod table;

use crate::player::Player;
use crate::table::Table;

fn main() {
    let mut table = Table::new("Table 1");
    let mut p1 = Player::new("Andrey", 10000);
    let mut p2 = Player::new("Bot 1", 5000);
    let mut p3 = Player::new("Bot 2", 2000);
    p1.join_table(&mut table);
    p2.join_table(&mut table);
    p3.join_table(&mut table);
    table.play_round();
}
/*
TODO{

    add trait system with money and stuff
    let user type in terminal to redraw / discard
    add multiple players and turns

}
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
1. rawlisian poker
2. adding traits
3. GUI
4. multiplayer?
5. normal poker? (ez prob)
}
 */