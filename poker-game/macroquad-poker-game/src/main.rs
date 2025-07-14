use macroquad::prelude::*;

mod game;
use game::{Game, GameState, Round, Card, Rank};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

// Colors
const TABLE_COLOR: Color = Color::new(0.0, 0.5, 0.0, 1.0);
const CARD_COLOR: Color = WHITE;
const TEXT_COLOR: Color = WHITE;
const BUTTON_COLOR: Color = Color::new(0.2, 0.2, 0.8, 1.0);
const BUTTON_HOVER_COLOR: Color = Color::new(0.3, 0.3, 0.9, 1.0);
const BUTTON_TEXT_COLOR: Color = WHITE;

struct Button {
    rect: Rect,
    text: String,
    action: ButtonAction,
}

enum ButtonAction {
    Bet,
    Call,
    Check,
    Fold,
    Deal,
    NewGame,
}

impl Button {
    fn new(x: f32, y: f32, width: f32, height: f32, text: &str, action: ButtonAction) -> Self {
        Button {
            rect: Rect::new(x, y, width, height),
            text: text.to_string(),
            action,
        }
    }

    fn draw(&self, hovered: bool) {
        let color = if hovered { BUTTON_HOVER_COLOR } else { BUTTON_COLOR };
        
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
        draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 2.0, BLACK);
        
        let font_size = 20.0;
        let text_size = measure_text(&self.text, None, font_size as u16, 1.0);
        let text_x = self.rect.x + (self.rect.w - text_size.width) / 2.0;
        let text_y = self.rect.y + (self.rect.h + text_size.height) / 2.0;
        
        draw_text(&self.text, text_x, text_y, font_size, BUTTON_TEXT_COLOR);
    }

    fn is_hovered(&self, mouse_position: Vec2) -> bool {
        self.rect.contains(mouse_position)
    }
}

#[macroquad::main("Poker Game")]
async fn main() {
    // Initialize game
    let mut game = Game::new();
    
    // Add players
    game.add_player("Player 1".to_string(), 1000);
    game.add_player("Player 2".to_string(), 1000);
    
    // Create UI buttons
    let mut buttons = vec![
        Button::new(50.0, WINDOW_HEIGHT - 60.0, 100.0, 40.0, "Bet", ButtonAction::Bet),
        Button::new(160.0, WINDOW_HEIGHT - 60.0, 100.0, 40.0, "Call", ButtonAction::Call),
        Button::new(270.0, WINDOW_HEIGHT - 60.0, 100.0, 40.0, "Check", ButtonAction::Check),
        Button::new(380.0, WINDOW_HEIGHT - 60.0, 100.0, 40.0, "Fold", ButtonAction::Fold),
        Button::new(WINDOW_WIDTH - 150.0, WINDOW_HEIGHT - 60.0, 100.0, 40.0, "Deal", ButtonAction::Deal),
    ];
    
    // Game loop
    loop {
        // Process input
        let mouse_position = Vec2::new(mouse_position().0, mouse_position().1);
        
        // Handle button interactions
        for button in &buttons {
            if button.is_hovered(mouse_position) && is_mouse_button_pressed(MouseButton::Left) {
                match button.action {
                    ButtonAction::Bet => {
                        if game.state == GameState::PlayerTurn {
                            game.player_bet(10);
                        }
                    },
                    ButtonAction::Call => {
                        if game.state == GameState::PlayerTurn {
                            game.player_call();
                        }
                    },
                    ButtonAction::Check => {
                        if game.state == GameState::PlayerTurn {
                            game.player_check();
                        }
                    },
                    ButtonAction::Fold => {
                        if game.state == GameState::PlayerTurn {
                            game.player_fold();
                        }
                    },
                    ButtonAction::Deal => {
                        if game.state == GameState::Setup || game.state == GameState::GameOver {
                            game.deal_cards();
                        }
                    },
                    ButtonAction::NewGame => {
                        game = Game::new();
                        game.add_player("Player 1".to_string(), 1000);
                        game.add_player("Player 2".to_string(), 1000);
                    },
                }
            }
        }
        
        // Update game state
        if game.state == GameState::GameOver {
            // Replace Deal button with New Game button if not already done
            if buttons.last().unwrap().action != ButtonAction::NewGame {
                buttons.pop();
                buttons.push(Button::new(WINDOW_WIDTH - 150.0, WINDOW_HEIGHT - 60.0, 100.0, 40.0, "New Game", ButtonAction::NewGame));
            }
        } else if game.state == GameState::Setup || game.state == GameState::PlayerTurn {
            // Replace New Game button with Deal button if not already done
            if buttons.last().unwrap().action != ButtonAction::Deal {
                buttons.pop();
                buttons.push(Button::new(WINDOW_WIDTH - 150.0, WINDOW_HEIGHT - 60.0, 100.0, 40.0, "Deal", ButtonAction::Deal));
            }
        }
        
        // Render
        clear_background(DARKGRAY);
        
        // Draw poker table
        draw_rectangle(50.0, 50.0, WINDOW_WIDTH - 100.0, WINDOW_HEIGHT - 150.0, TABLE_COLOR);
        
        // Draw community cards
        let card_width = 70.0;
        let card_height = 100.0;
        let card_spacing = 10.0;
        let start_x = (WINDOW_WIDTH - (5.0 * card_width + 4.0 * card_spacing)) / 2.0;
        let community_y = 150.0;
        
        for (i, card) in game.community_cards.iter().enumerate() {
            let x = start_x + i as f32 * (card_width + card_spacing);
            draw_card(card, x, community_y, card_width, card_height);
        }
        
        // Draw player hands
        for (i, player) in game.players.iter().enumerate() {
            let player_y = WINDOW_HEIGHT - 200.0 - i as f32 * 120.0;
            
            // Draw player info
            draw_text(&format!("{}: ${}", player.name, player.chips), 60.0, player_y - 10.0, 20.0, TEXT_COLOR);
            
            // Draw player cards
            for (j, card) in player.hand.iter().enumerate() {
                let x = 60.0 + j as f32 * (card_width + card_spacing);
                draw_card(card, x, player_y, card_width, card_height);
            }
            
            // Highlight current player
            if i == game.current_player_index && game.state == GameState::PlayerTurn {
                draw_rectangle_lines(50.0, player_y - 30.0, 200.0, 120.0, 2.0, YELLOW);
            }
        }
        
        // Draw game info
        draw_text(&format!("Pot: ${}", game.pot), WINDOW_WIDTH - 200.0, 80.0, 24.0, TEXT_COLOR);
        draw_text(&format!("Current Bet: ${}", game.current_bet), WINDOW_WIDTH - 200.0, 110.0, 20.0, TEXT_COLOR);
        
        // Draw round info
        let round_text = match game.round {
            Round::PreFlop => "Pre-Flop",
            Round::Flop => "Flop",
            Round::Turn => "Turn",
            Round::River => "River",
            Round::Complete => "Complete",
        };
        draw_text(&format!("Round: {}", round_text), WINDOW_WIDTH - 200.0, 140.0, 20.0, TEXT_COLOR);
        
        // Draw game state
        let state_text = match game.state {
            GameState::Setup => "Setup",
            GameState::Dealing => "Dealing",
            GameState::Betting => "Betting",
            GameState::PlayerTurn => "Player Turn",
            GameState::ShowDown => "Show Down",
            GameState::GameOver => "Game Over",
        };
        draw_text(&format!("State: {}", state_text), WINDOW_WIDTH - 200.0, 170.0, 20.0, TEXT_COLOR);
        
        // Draw winner if game is over
        if game.state == GameState::GameOver {
            if let Some(winner_index) = game.get_winner() {
                let winner = &game.players[winner_index];
                draw_text(&format!("Winner: {}", winner.name), WINDOW_WIDTH / 2.0 - 100.0, 100.0, 30.0, GOLD);
            }
        }
        
        // Draw buttons
        for button in &buttons {
            button.draw(button.is_hovered(mouse_position));
        }
        
        next_frame().await;
    }
}

fn draw_card(card: &Card, x: f32, y: f32, width: f32, height: f32) {
    // Draw card background
    draw_rectangle(x, y, width, height, CARD_COLOR);
    draw_rectangle_lines(x, y, width, height, 2.0, BLACK);
    
    // Determine card color based on suit
    let color = match card.suit {
        '♥' | '♦' => RED,
        _ => BLACK,
    };
    
    // Draw card rank
    let rank_str = match card.rank {
        Rank::Number(n) => n.to_string(),
        Rank::Jack => "J".to_string(),
        Rank::Queen => "Q".to_string(),
        Rank::King => "K".to_string(),
        Rank::Ace => "A".to_string(),
    };
    
    draw_text(&rank_str, x + 5.0, y + 20.0, 20.0, color);
    
    // Draw card suit
    draw_text(&card.suit.to_string(), x + 5.0, y + 45.0, 30.0, color);
}
