use crate::card::{Card};
use crate::deck::Deck;
use crate::player::Player;

pub struct Table {
    table_id: String,
    deck: Deck,
    players: Vec<Player>,
}

impl Table {
    pub fn new(table_id: &str) -> Self {
        Table {
            table_id: table_id.to_string(),
            deck: Deck::new(),
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player_id: usize) {
        // Remove player from the table
    }

    pub fn start_game(&mut self) {
        // Initialize the game state
        self.deck.shuffle();
        // Deal initial cards to players
    }

    pub fn player_draw(&mut self, player_id: usize) -> Option<Card> {
        if let Some(card) = self.deck.draw() {
            if let Some(player) = self.players.get_mut(player_id) {
                player.add_card(card);
                Some(card)
            } else {
                // Player not found
                None
            }
        } else {
            // Deck is empty
            None
        }
    }

    // Other table-related methods
}