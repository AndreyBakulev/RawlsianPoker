use crate::card::Card;
use crate::deck::Deck;
use crate::player::Player;

#[derive(Debug, PartialEq,PartialOrd, Ord, Hash)]
pub struct Table {
    pub table_id: String,
    pub(crate) deck: Deck,
    pub players: Vec<Player>,
    pub pot: f64,
    pub community_card: Vec<Card>,
}
impl Table {
    pub fn new(table_id: &str) -> Self {
        Table {
            table_id: table_id.to_string(),
            deck: Deck::new(),
            players: Vec::new(),
            pot: 0.0,
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
        self.pot = 0.0;
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
        println!("community cards: {:?}",self.community_card);
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
        let mut last_raise = 0.0;
        let current_players: Vec<&Player> = self.players.iter().filter(|x| !x.folded).collect();
        for i in 0.. current_players.len(){
            let player = &mut self.players[i];
            println!("{}'s turn", player.id);
            println!("Current pot: {}", self.pot);
            println!("Your balance: {}", player.balance);
            println!("Enter your action (bet, call, raise, fold):");

            let mut action = String::new();
            std::io::stdin().read_line(&mut action).expect("Failed to read line");
            let action = action.trim();
            match action {
                "bet" => {
                    println!("Enter the bet amount:");
                    let mut amount = String::new();
                    std::io::stdin().read_line(&mut amount).expect("Failed to read line");
                    let amount: f64 = amount.trim().parse().expect("Invalid bet amount");
                    player.bet(self,amount);
                    last_raise = amount;
                }
                "call" => {
                    //bet the same as the person before u
                    player.bet(self,last_raise);
                }
                "raise" => {
                    println!("Enter the raise amount:");
                    let mut amount = String::new();
                    std::io::stdin().read_line(&mut amount).expect("Failed to read line");
                    let amount: f64 = amount.trim().parse().expect("Invalid raise amount");
                    if amount > last_raise {
                        player.bet(self,amount);
                        last_raise = amount;
                    } else {
                    println!("Cannot raise to a lower value!");
                        }
                }
                "fold" => {
                    player.fold();
                }
                _ => {
                    println!("Invalid action. Please enter a valid action.");
                    continue;
                }
            }
        }
        println!("Everyone has made an action this turn!");
    }
}