use std::fmt;
use crate::card::Card;
use crate::deck::Deck;
use crate::player::Player;

#[derive(Debug, PartialEq,PartialOrd, Eq,Ord, Hash)]
pub struct Table {
    pub table_id: String,
    pub(crate) deck: Deck,
    pub players: Vec<Player>,
    pub pot: i64,
    pub community_card: Vec<Card>,
}
impl Table {
    pub fn new(table_id: &str) -> Self {
        Table {
            table_id: table_id.to_string(),
            deck: Deck::new(),
            players: Vec::new(),
            pot: 0,
            community_card: Vec::new(),
        }
    }
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
    pub fn remove_player(&mut self, player_id: &str) {
        self.players.retain(|player| player.id != player_id);
    }
    pub fn play_round(&mut self) {
        self.pot = 0;
        self.community_card.clear();
        for player in &mut self.players {
            player.hand.clear();
        }
        self.deck.shuffle();
        for _ in 0..2 {
            for player in &mut self.players {
                player.draw(&mut self.deck);
            }
        }
        self.betting_round();
        //adds 3 to community cards
        for i in 0 .. 3{
            self.community_card.push(self.deck.draw().unwrap());

        }
        self.betting_round();
        self.community_card.push(self.deck.draw().unwrap());
        self.betting_round();
        self.community_card.push(self.deck.draw().unwrap());
        self.betting_round();

        // Showdown and determine the winner
        // TODO: Implement showdown and winner determination

        // Award the pot to the winner
        // TODO: Implement awarding the pot to the winner
    }

    fn betting_round(&mut self) {
        let mut last_raise = 0;
        let mut current_players: Vec<usize> = self.players.iter().enumerate()
            .filter(|(_, player)| !player.folded)
            .map(|(index, _)| index)
            .collect();
            for i in 0..current_players.len() {
                let player_index = current_players[i];
                let player = &mut self.players[player_index];
                println!("Community Cards:{:?}\nPot: {}\n{}'s turn:\nBalance: {}\nCards: {} ({:?})"
                         ,self.community_card,self.pot,player.id,player.balance,player,player.evaluate_hand());
                println!("Enter your action (bet, call, raise, fold):");
                let mut action = String::new();
                std::io::stdin().read_line(&mut action).expect("Failed to read line");
                let action = action.trim();

                match action {
                    "bet" => {
                        println!("Enter the bet amount:");
                        let mut amount = String::new();
                        std::io::stdin().read_line(&mut amount).expect("Failed to read line");
                        let amount: i64 = amount.trim().parse().expect("Invalid bet amount");
                        player.bet(&mut self.pot, amount);
                        last_raise = amount;
                    }
                    "call" => {
                        player.bet(&mut self.pot, last_raise);
                    }
                    "raise" => {
                        println!("Enter the raise amount:");
                        let mut amount = String::new();
                        std::io::stdin().read_line(&mut amount).expect("Failed to read line");
                        let amount: i64 = amount.trim().parse().expect("Invalid raise amount");
                        if amount > last_raise {
                            player.bet(&mut self.pot, amount);
                            last_raise = amount;
                            current_players = self.players.iter().enumerate()
                                .filter(|(_, player)| !player.folded)
                                .map(|(index, _)| index)
                                .collect();
                            break;
                        } else {
                            println!("Cannot raise to a lower value!");
                        }
                    }
                    "fold" => {
                        player.fold();
                        current_players.remove(i);
                        break;
                    }
                    _ => {
                        println!("Invalid action. Please enter a valid action.");
                    }
                }

                // println!("{}'s Hand:\n{}", player.id, player);
                // let hand_rank = player.evaluate_hand();
                // println!("Hand Rank:\n{:?}", hand_rank);
            }


        println!("Everyone has made an action this turn!");
    }
}
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let community_cards: Vec<String> = self.community_card.iter().map(|card| card.to_string()).collect();
        write!(f, "{}", community_cards.join(", "))
    }
}