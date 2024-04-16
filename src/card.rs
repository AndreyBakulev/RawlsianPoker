use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct Card {
    value: u8,
    suit: Suit,
}

impl Card {
    pub fn new(value: u8, suit: Suit) -> Card {
        Card { value, suit }
    }

    pub fn name(&self) -> String {
        match self.value {
            1 => "Ace".to_string(),
            11 => "Jack".to_string(),
            12 => "Queen".to_string(),
            13 => "King".to_string(),
            _ => self.value.to_string(),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {:?}", self.name(), self.suit)
    }
}