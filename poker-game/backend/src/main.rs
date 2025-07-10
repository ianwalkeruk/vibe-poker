use actix_web::{web, App, HttpServer, HttpResponse, Error};
use actix::{Actor, StreamHandler, Handler, Message};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

mod game;
use game::{Game, Player};

struct WsSession {
    game: web::Data<Arc<Mutex<Game>>>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Handle incoming messages from clients
                let message: GameMessage = serde_json::from_str(&text).unwrap();
                match message {
                    GameMessage::Join => {
                        // Handle player joining the game
                        let mut game = self.game.get_ref().lock().unwrap();
                        let player = Player {
                            name: "Player".to_string(),
                            chips: 1000,
                            hand: vec![],
                        };
                        game.players.push(player);
                        self.send_game_state(ctx);
                    }
                    GameMessage::Bet(amount) => {
                        // Handle player betting
                        let mut game = self.game.get_ref().lock().unwrap();
                        if let Some(player) = game.players.get_mut(0) {
                            if player.chips >= amount {
                                player.chips -= amount;
                                game.current_bet = amount;
                                game.pot += amount;
                                self.send_game_state(ctx);
                            }
                        }
                    }
                    GameMessage::Fold => {
                        // Handle player folding
                        let mut game = self.game.get_ref().lock().unwrap();
                        game.players.remove(0);
                        self.send_game_state(ctx);
                    }
                    GameMessage::Check => {
                        // Handle player checking
                        self.send_game_state(ctx);
                    }
                    GameMessage::Call => {
                        // Handle player calling
                        let mut game = self.game.get_ref().lock().unwrap();
                        let current_bet = game.current_bet;
                        let player = game.players.get_mut(0);
                        if let Some(player) = player {
                            if player.chips >= current_bet {
                                player.chips -= current_bet;
                                game.pot += current_bet;
                                self.send_game_state(ctx);
                            }
                        }
                    }
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            _ => (),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct Connect;

impl Handler<Connect> for WsSession {
    type Result = ();

    fn handle(&mut self, _: Connect, _ctx: &mut Self::Context) {
        // No action needed for connect
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum GameMessage {
    Join,
    Bet(u32),
    Fold,
    Check,
    Call,
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to the Poker Game Backend!")
}

async fn ws_index(
    req: actix_web::HttpRequest,
    stream: web::Payload,
    srv: web::Data<Arc<Mutex<Game>>>,
) -> Result<HttpResponse, Error> {
    let game = srv.clone();
    // No need to unwrap the game here
    // No need to clone the game here
    let resp = ws::start(WsSession { game: srv.clone() }, &req, stream);
    resp
}

impl WsSession {
    fn send_game_state(&self, ctx: &mut <WsSession as Actor>::Context) {
        let game = self.game.get_ref().lock().unwrap();
        let game_state = serde_json::to_string(&*game).unwrap();
        ctx.text(game_state);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let game = web::Data::new(Arc::new(Mutex::new(Game::new())));

    HttpServer::new(move || {
        App::new()
            .app_data(game.clone())
            .route("/", web::get().to(index))
            .route("/ws", web::get().to(ws_index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}