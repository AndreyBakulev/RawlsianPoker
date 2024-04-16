use std::fmt;
use crate::card::Card;
use crate::deck::Deck;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Player {
    pub(crate) hand: Vec<Card>,
    id: String
}

impl Player {
    pub fn new(id: &str) -> Self {
        Player {
            hand: Vec::new(),
            id: id.to_string()
        }
    }
    pub fn draw(&mut self) -> Option<Card> {
        let mut deck = DECK.lock().unwrap();
        if let Some(card) = deck.card_array.pop() {
            self.hand.push(card);
            Some(card)
        } else {
            None
        }
    }
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hand: Vec<String> = self.hand.iter().map(|card| card.to_string()).collect();
        write!(f, "{}", hand.join("\n"))
    }
}

