use std::collections::HashMap;
use std::fmt;
use crate::card::{Card, Suit};
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

#[derive(Debug, PartialEq, PartialOrd, Eq,Ord, Hash)]
pub struct Player {
    pub(crate) hand: Vec<Card>,
    pub(crate) id: String,
    pub(crate) balance: i64,
    pub folded: bool,
}

impl Player {
    pub fn new(id: &str, balance: i64) -> Self {
        Player {
            hand: Vec::new(),
            id: id.to_string(),
            balance,
            folded: false,
        }
    }
    pub fn join_table(self,table: &mut Table){
        table.add_player(self);
    }
    pub fn draw(&mut self, deck: &mut Deck) -> Option<Card> {
        if let Some(card) = deck.draw() {
            self.hand.push(card);
            Some(card)
        } else {
            None
        }
    }
    pub fn bet(&mut self, pot: &mut i64, amount: i64){
        if self.balance >= amount {
            *pot += amount;
            self.balance -= amount;
            println!("Successfully bet {}!", amount);
        } else {
            println!("Not enough balance to bet!");
        }
    }
    pub fn fold(&mut self){
        self.folded = true;
    }

    pub fn evaluate_hand(&self, community_cards: &Vec<Card>) -> PokerHand {
        let hand_rank;
        let mut rank_counts = HashMap::new();
        let mut suit_counts = HashMap::new();
        let mut total_cards: Vec<Card> = self.hand.clone();
        total_cards.extend(community_cards.clone());
        total_cards.sort();
        // Count the occurrences of each rank and suit
        for card in &total_cards {
            *rank_counts.entry(card.value).or_insert(0) += 1;
            *suit_counts.entry(card.suit).or_insert(0) += 1;
        }

        // Check for flush
        let is_flush = self.is_flush(&total_cards);
        // Check for straight
        let is_straight = self.is_straight(&total_cards);

        // Calculate the hand rank based on the counts
        if is_flush && is_straight {
            //if there is an ace
            hand_rank = if total_cards[0].value == 1 {
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

    //something in this method is wrong
    fn is_straight(&self, total_cards: &Vec<Card>) -> bool {
        let mut count = 0;
        for i in 1.. total_cards.len(){
            if total_cards[i].value == (total_cards[i-1].value + 1) {
                count += 1;
            }
        }
        count == 4 || (count == 3 && total_cards[0].value == 1 && total_cards[4].value == 13)
    }
    fn is_flush(&self, total_cards: &Vec<Card>) -> bool{
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            let mut count:i8  = 0;
            for card in total_cards {
                if card.suit == suit {
                    count += 1;
                }
            }
            if count >= 5 {
                true;
            }
        }
        false
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hand: Vec<String> = self.hand.iter().map(|card| card.to_string()).collect();
        write!(f, "{}", hand.join(", "))
    }
}