use std::fmt;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::card::{Card, Suit};

lazy_static! {
    pub static ref DECK: Mutex<Deck> = Mutex::new(Deck::new());
}
pub struct Deck {
    pub card_array: Vec<Card>,
}
impl Deck {
    fn new() -> Self {
        let mut card_array = Vec::new();
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for value in 1..=13 {
                card_array.push(Card::new(value, suit));
            }
        }
        Deck { card_array }
    }
    pub fn shuffle() {
        let mut deck = DECK.lock().unwrap();
        let mut rng = thread_rng();
        deck.card_array.shuffle(&mut rng);
    }
    pub fn draw() -> Option<Card> {
        let mut deck = DECK.lock().unwrap();
        deck.card_array.pop()
    }
    pub fn is_empty() -> bool {
        let deck = DECK.lock().unwrap();
        deck.card_array.is_empty()
    }
    pub fn remaining_cards() -> usize {
        let deck = DECK.lock().unwrap();
        deck.card_array.len()
    }
}
impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let card_strings: Vec<String> = self.card_array.iter().map(|card| card.to_string()).collect();
        write!(f, "{}", card_strings.join("\n"))
    }
}