mod card;
mod deck;
use deck::Deck;
fn main() {
    println!("Hello, world!");
    let mut deck = Deck::new();
    println!("New Deck: {:?}", deck.card_array);

    deck.shuffle();
    println!("Shuffled Deck: {:?}", deck.card_array);

    let card = deck.draw();
    match card {
        Some(c) => println!("Drawn Card: {:?}", c),
        None => println!("No more cards in the deck!"),
    }
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
1. Add card class and an array of cards named Deck
give each card a name, value, and suit
}
 */