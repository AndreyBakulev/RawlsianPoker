mod card;
mod deck;
mod player;
mod table;

use crate::player::Player;
use crate::table::Table;

fn main() {
    let mut table = Table::new("Table 1");
    let mut players: Vec<Player> = Vec::new();
    &players.push(Player::new("Andrey", 10000));
    &players.push(Player::new("Irina", 10000));
    &players.push(Player::new("Bot 3", 10000));
    &players.push(Player::new("Bot 4", 10000));
    &players.push(Player::new("Bot 5", 10000));
    for i in 0.. players.len(){
        table.add_player(players[i].clone());
    }
    table.play_game();
}
/*

FOR NORMAL POKER:
you have to add a pre-flop round and add a button(dealer), small blind + big blind
big blind bets x value, and small blind bets .5 of big blind
then each player can call, raise, or fold (min raise is 2x big blind)
then flop
POST FLOP:
each user can either check (bet 0 but continue), fold, or bet
AFTER BET, each user has to call raise or fold.
then turn
then once again check or bet
then river
then once again check or bet
then show

TODO{

    add trait system with money and stuff
    let user type in terminal to redraw / discard
    add multiple players and turns

    Add more games (blackjack, chess)
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