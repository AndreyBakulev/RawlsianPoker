use std::{cmp, fmt, io};
use crate::card::Card;
use crate::deck::Deck;
use crate::player::Player;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct Table {
    pub table_id: String,
    pub(crate) deck: Deck,
    pub players: Vec<Player>,
    pub pot: i64,
    pub community_cards: Vec<Card>,
    pub button: i16,
    pub min_bet: i64,
}

impl Table {
    pub fn new(table_id: &str) -> Self {
        Table {
            table_id: table_id.to_string(),
            deck: Deck::new(),
            players: Vec::new(),
            pot: 0,
            community_cards: Vec::new(),
            button: 0,
            min_bet: 10,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player_id: &str) {
        self.players.retain(|player| player.id != player_id);
    }

    pub fn play_game(&mut self) {
        loop {
            self.play_round();
            // Check if there is only one player left
            if self.players.len() == 1 {
                println!("Game over! {} wins the game!", self.players[0].id);
                break;
            }
            // Ask if players want to continue playing
            println!("Do you want to play another round? (y/n)");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            if input.trim().to_lowercase() != "y" {
                break;
            }
        }
    }

    fn play_round(&mut self) {
        self.pot = 0;
        self.community_cards.clear();

        // Determine the small blind and big blind positions
        let small_blind_index = self.wrapped_index(self.button + 1);
        let big_blind_index = self.wrapped_index(self.button + 2);

        // Reset folded state for all players
        for player in &mut self.players {
            player.folded = false;
            player.hand.clear();
        }
        self.deck.shuffle();
        println!("Small blind is: {} and big blind is {}", &self.players[small_blind_index].id, &self.players[big_blind_index].id);
        println!("Min bet this round is: {}", self.min_bet);
        &self.players[small_blind_index].bet(&mut self.pot, self.min_bet / 2);
        &self.players[big_blind_index].bet(&mut self.pot, self.min_bet);

        // Deal cards to each player
        for _ in 0..2 {
            for player in &mut self.players {
                player.draw(&mut self.deck);
            }
        }

        // Pre-flop betting round
        self.betting_round();
        // Flop
        for _ in 0..3 {
            self.community_cards.push(self.deck.draw().unwrap());
        }
        println!("Flop: {:?}", &self.community_cards);
        self.betting_round();

        // Turn
        self.community_cards.push(self.deck.draw().unwrap());
        println!("Turn: {:?}", &self.community_cards);
        self.betting_round();

        // River
        self.community_cards.push(self.deck.draw().unwrap());
        println!("River: {:?}", &self.community_cards);
        self.betting_round();

        // Showdown and determine the winner
        let mut poker_hands: Vec<(usize, HandRank)> = self.players
            .iter()
            .enumerate()
            .filter(|(_, player)| !player.folded)
            .map(|(index, player)| (index, player.evaluate_hand(&self.community_cards)))
            .collect();

        poker_hands.sort_by(|(_, a), (_, b)| b.cmp(a));

        let (winner_index, winning_hand) = poker_hands[0];
        println!("The winner is {} with a {:?}", self.players[winner_index].id, winning_hand);

        // Award the pot to the winner
        self.players[winner_index].balance += self.pot;
        println!("{} has been added to {}'s balance!", self.pot, self.players[winner_index].id);
        println!("{}'s balance is now {}!", self.players[winner_index].id, self.players[winner_index].balance);

        self.pot = 0;
        self.min_bet *= 2;
        self.button = self.wrapped_index(self.button + 1) as i16;

        // Remove players with zero balance
        self.players.retain(|player| player.balance > 0);
    }

    fn betting_round(&mut self) {
        let mut last_raise = 0;
        let mut current_player_index = self.wrapped_index(self.button + 3);

        loop {
            if &self.community_cards.len() <= &0 {
                last_raise = self.min_bet;
            }
                let player = &mut self.players[current_player_index];
                if player.folded {
                    current_player_index = self.wrapped_index((current_player_index + 1) as i16);
                    continue;
                }
                println!("\nCommunity Cards: {:?}\nPot: {}\n{}'s turn:\nBalance: {}\nHand: {}",
                         self.community_cards, self.pot, player.id, player.balance, player);
                println!("Enter your action (call, raise, fold):");
                let mut action = String::new();
                io::stdin().read_line(&mut action).expect("Failed to read line");
                let action = action.trim();
            match action {
                "check" => {
                    if last_raise > 0 {
                        println!("Cannot check when there is a bet to call!");
                    }
                }
                "call" => {
                    let call_amount = cmp::min(last_raise, player.balance);
                    player.bet(&mut self.pot, call_amount);
                }
                "raise" => {
                    println!("Enter the raise amount(must raise min 2x big blind):");
                    let mut amount = String::new();
                    io::stdin().read_line(&mut amount).expect("Failed to read line");
                    let amount: i64 = amount.trim().parse().expect("Invalid raise amount");
                    if amount > last_raise*2 {
                        player.bet(&mut self.pot, amount);
                        last_raise = amount;
                    } else {
                        println!("Raise amount must 2x be greater than the last raise!");
                    }
                }
                "fold" => {
                    player.folded = true;
                }
                _ => {
                    println!("Invalid action. Please enter a valid action.");
                    continue;
                }
            }
            current_player_index = self.wrapped_index((current_player_index + 1) as i16);
            // Check if all players have either folded or called the last raise
            if self.players.iter().all(|player| player.folded || player.balance == 0) {
                break;
            }
            // Check if the betting round is complete
            if current_player_index == self.wrapped_index(self.button + 3) {
                break;
            }
        }
    }

    fn wrapped_index(&self, index: i16) -> usize {
        ((index % self.players.len() as i16 + self.players.len() as i16) % self.players.len() as i16) as usize
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let community_cards: Vec<String> = self.community_cards.iter().map(|card| card.to_string()).collect();
        let mut output = String::new();
        output.push_str("Community Cards:\n");

        for (i, card) in community_cards.iter().enumerate() {
            output.push_str(&format!("{}", card));
            if i < community_cards.len() - 1 {
                output.push_str(", ");
            }
            if (i + 1) % 3 == 0 {
                output.push_str("\n");
            }
        }

        write!(f, "{}", output.trim())
    }
}