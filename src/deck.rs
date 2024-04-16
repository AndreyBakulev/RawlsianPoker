use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::card::{Card, Suit};

pub struct Deck {
    pub card_array: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut card_array = Vec::new();
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for value in 1..=13 {
                card_array.push(Card::new(value, suit));
            }
        }
        Deck { card_array }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.card_array.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.card_array.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.card_array.is_empty()
    }

    pub fn remaining_cards(&self) -> usize {
        self.card_array.len()
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let card_strings: Vec<String> = self.card_array.iter().map(|card| card.to_string()).collect();
        write!(f, "{}", card_strings.join("\n"))
    }
}