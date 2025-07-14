use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GameState {
    Setup,
    Dealing,
    Betting,
    PlayerTurn,
    ShowDown,
    GameOver,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub community_cards: Vec<Card>,
    pub current_bet: u32,
    pub pot: u32,
    pub current_player_index: usize,
    pub state: GameState,
    pub round: Round,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Round {
    PreFlop,
    Flop,
    Turn,
    River,
    Complete,
}

impl Game {
    pub fn new() -> Self {
        // Initialize a new game
        let mut game = Game {
            players: vec![],
            deck: Deck::new(),
            community_cards: vec![],
            current_bet: 0,
            pot: 0,
            current_player_index: 0,
            state: GameState::Setup,
            round: Round::PreFlop,
        };
        
        // Shuffle the deck
        game.deck.shuffle();
        
        game
    }
    
    pub fn add_player(&mut self, name: String, chips: u32) -> usize {
        let player = Player {
            name,
            chips,
            hand: vec![],
            has_folded: false,
            has_acted: false,
        };
        self.players.push(player);
        self.players.len() - 1
    }
    
    pub fn deal_cards(&mut self) {
        // Clear any existing hands and community cards
        self.community_cards.clear();
        for player in &mut self.players {
            player.hand.clear();
            player.has_acted = false;
            player.has_folded = false;
        }
        
        // Reset the deck and shuffle
        self.deck = Deck::new();
        self.deck.shuffle();
        
        // Deal two cards to each player
        for _ in 0..2 {
            for player in &mut self.players {
                if let Some(card) = self.deck.draw_card() {
                    player.hand.push(card);
                }
            }
        }
        
        // Reset game state
        self.current_bet = 0;
        self.pot = 0;
        self.current_player_index = 0;
        self.state = GameState::PlayerTurn;
        self.round = Round::PreFlop;
    }
    
    pub fn next_player(&mut self) {
        // Find the next player who hasn't folded
        let start_index = self.current_player_index;
        loop {
            self.current_player_index = (self.current_player_index + 1) % self.players.len();
            
            // If we've gone all the way around, break
            if self.current_player_index == start_index {
                break;
            }
            
            // If this player hasn't folded, break
            if !self.players[self.current_player_index].has_folded {
                break;
            }
        }
        
        // Check if all players have acted
        let all_acted = self.players.iter()
            .filter(|p| !p.has_folded)
            .all(|p| p.has_acted);
            
        if all_acted {
            self.next_round();
        }
    }
    
    pub fn next_round(&mut self) {
        // Reset player actions
        for player in &mut self.players {
            player.has_acted = false;
        }
        
        // Move to the next round
        match self.round {
            Round::PreFlop => {
                // Deal the flop (3 community cards)
                for _ in 0..3 {
                    if let Some(card) = self.deck.draw_card() {
                        self.community_cards.push(card);
                    }
                }
                self.round = Round::Flop;
            },
            Round::Flop => {
                // Deal the turn (1 more community card)
                if let Some(card) = self.deck.draw_card() {
                    self.community_cards.push(card);
                }
                self.round = Round::Turn;
            },
            Round::Turn => {
                // Deal the river (1 more community card)
                if let Some(card) = self.deck.draw_card() {
                    self.community_cards.push(card);
                }
                self.round = Round::River;
            },
            Round::River => {
                // Show down
                self.state = GameState::ShowDown;
                self.round = Round::Complete;
            },
            Round::Complete => {
                // Game over
                self.state = GameState::GameOver;
            },
        }
        
        // Reset current bet for the new round
        self.current_bet = 0;
    }
    
    pub fn player_bet(&mut self, amount: u32) {
        let player = &mut self.players[self.current_player_index];
        
        // Check if player has enough chips
        if player.chips >= amount {
            player.chips -= amount;
            self.pot += amount;
            self.current_bet = amount;
            player.has_acted = true;
            
            self.next_player();
        }
    }
    
    pub fn player_call(&mut self) {
        let player = &mut self.players[self.current_player_index];
        
        // Check if player has enough chips
        if player.chips >= self.current_bet {
            player.chips -= self.current_bet;
            self.pot += self.current_bet;
            player.has_acted = true;
            
            self.next_player();
        }
    }
    
    pub fn player_check(&mut self) {
        let player = &mut self.players[self.current_player_index];
        player.has_acted = true;
        
        self.next_player();
    }
    
    pub fn player_fold(&mut self) {
        let player = &mut self.players[self.current_player_index];
        player.has_folded = true;
        player.has_acted = true;
        
        // Check if only one player remains
        let active_players = self.players.iter().filter(|p| !p.has_folded).count();
        if active_players == 1 {
            self.state = GameState::GameOver;
        } else {
            self.next_player();
        }
    }
    
    pub fn get_winner(&self) -> Option<usize> {
        // If only one player hasn't folded, they win
        let active_players: Vec<usize> = self.players.iter()
            .enumerate()
            .filter(|(_, p)| !p.has_folded)
            .map(|(i, _)| i)
            .collect();
            
        if active_players.len() == 1 {
            return Some(active_players[0]);
        }
        
        // TODO: Implement hand evaluation logic
        // For now, just return the first active player
        active_players.first().copied()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub chips: u32,
    pub hand: Vec<Card>,
    pub has_folded: bool,
    pub has_acted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        // Initialize a new deck of cards
        let mut cards = vec![];
        for suit in ['♥', '♦', '♣', '♠'] {
            for rank in 2..=10 {
                cards.push(Card { suit, rank: Rank::Number(rank as u8) });
            }
            for rank in ['J', 'Q', 'K', 'A'] {
                let card_rank = match rank {
                    'J' => Rank::Jack,
                    'Q' => Rank::Queen,
                    'K' => Rank::King,
                    'A' => Rank::Ace,
                    _ => unreachable!(),
                };
                cards.push(Card { suit, rank: card_rank });
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        // Implement deck shuffling
        let mut rng = rand::thread_rng();
        self.cards.as_mut_slice().shuffle(&mut rng);
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        // Implement card drawing
        self.cards.pop()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Card {
    pub suit: char,
    pub rank: Rank,
}

impl Card {
    pub fn to_string(&self) -> String {
        let rank_str = match self.rank {
            Rank::Number(n) => n.to_string(),
            Rank::Jack => "J".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::King => "K".to_string(),
            Rank::Ace => "A".to_string(),
        };
        
        format!("{}{}", rank_str, self.suit)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}