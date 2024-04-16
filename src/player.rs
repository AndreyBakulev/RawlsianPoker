use crate::card::Card;
use crate::deck::DECK;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Player {
    pub(crate) hand: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            hand: Vec::new(),
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

