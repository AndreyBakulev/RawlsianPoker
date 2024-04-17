use crate::deck::Deck;
use crate::player::Player;

#[derive(Debug, PartialEq, Eq,PartialOrd, Ord, Hash)]
pub struct Table {
    table_id: String,
    pub(crate) deck: Deck,
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
    pub fn remove_player(&mut self, player_id: &str) {
        self.players.retain(|player| player.id != player_id);
    }
    pub fn start_game(&mut self) {
        self.deck.shuffle();
        // Deal initial cards to players
    }
}