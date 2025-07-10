use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    // Define game state structure here
    pub players: Vec<Player>,
    pub deck: Deck,
    pub community_cards: Vec<Card>,
    pub current_bet: u32,
    pub pot: u32,
    pub current_player_index: usize,
}

impl Game {
    pub fn new() -> Self {
        // Initialize a new game
        Game {
            players: vec![],
            deck: Deck::new(),
            community_cards: vec![],
            current_bet: 0,
            pot: 0,
            current_player_index: 0,
        }
    }

    // Add methods for game logic here
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub chips: u32,
    pub hand: Vec<Card>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}