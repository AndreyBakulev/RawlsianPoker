use std::collections::HashMap;
use std::fmt;
use crate::card::Card;
use crate::deck::Deck;
use crate::table::Table;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PokerHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Hash)]
pub struct Player {
    pub(crate) hand: Vec<Card>,
    pub(crate) id: String,
    pub(crate) balance: f64,
    pub folded: bool,
}

impl Player {
    pub fn new(id: &str, balance: f64) -> Self {
        Player {
            hand: Vec::new(),
            id: id.to_string(),
            balance,
            folded: false,
        }
    }

    pub fn draw(&mut self, deck: &mut Deck) -> Option<Card> {
        if let Some(card) = deck.draw() {
            self.hand.push(card);
            Some(card)
        } else {
            None
        }
    }
    pub fn bet(&mut self, mut table: &mut Table, amount: f64){
        if (self.balance - amount) > 0.0 {
            table.pot += amount;
            self.balance -= amount;
            println!("Successfully bet {}!", amount);
        } else {
            println!("Not enough balance to bet!");
        }
    }
    pub fn fold(&mut self){
        self.folded = true;
    }

    pub fn evaluate_hand(&self) -> PokerHand {
        let mut hand_rank = 0;
        let mut rank_counts = HashMap::new();
        let mut suit_counts = HashMap::new();

        // Count the occurrences of each rank and suit
        for card in &self.hand {
            *rank_counts.entry(card.value).or_insert(0) += 1;
            *suit_counts.entry(card.suit).or_insert(0) += 1;
        }

        // Check for flush
        let is_flush = suit_counts.len() == 1;

        // Check for straight
        let is_straight = self.is_straight();

        // Calculate the hand rank based on the counts
        if is_flush && is_straight {
            hand_rank = if self.hand.iter().map(|c| c.value).min().unwrap() == 1 {
                10 // Royal Flush
            } else {
                9 // Straight Flush
            };
        } else if rank_counts.values().any(|&count| count == 4) {
            hand_rank = 8; // Four of a Kind
        } else if rank_counts.values().any(|&count| count == 2) && rank_counts.values().any(|&count| count == 3){
            hand_rank = 7; // Full House
        } else if is_flush {
            hand_rank = 6; // Flush
        } else if is_straight {
            hand_rank = 5; // Straight
        } else if rank_counts.values().any(|&count| count == 3) {
            hand_rank = 4; // Three of a Kind
        } else if rank_counts.values().filter(|&&count| count == 2).count() == 2 {
            hand_rank = 3; // Two Pair
        } else if rank_counts.values().any(|&count| count == 2) {
            hand_rank = 2; // One Pair
        } else {
            hand_rank = 1; // High Card
        }

        // Return the corresponding PokerHand variant based on the hand rank
        match hand_rank {
            10 => PokerHand::RoyalFlush,
            9 => PokerHand::StraightFlush,
            8 => PokerHand::FourOfAKind,
            7 => PokerHand::FullHouse,
            6 => PokerHand::Flush,
            5 => PokerHand::Straight,
            4 => PokerHand::ThreeOfAKind,
            3 => PokerHand::TwoPair,
            2 => PokerHand::OnePair,
            _ => PokerHand::HighCard,
        }
    }

    fn is_straight(&self) -> bool {
        let mut sorted_values: Vec<u8> = self.hand.iter().map(|c| c.value).collect();
        sorted_values.sort();

        let mut count = 0;
        for i in 1..sorted_values.len() {
            if sorted_values[i] == sorted_values[i - 1] + 1 {
                count += 1;
            }
        }

        count == 4 || (count == 3 && sorted_values[0] == 1 && sorted_values[4] == 13)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hand: Vec<String> = self.hand.iter().map(|card| card.to_string()).collect();
        write!(f, "{}", hand.join("\n"))
    }
}