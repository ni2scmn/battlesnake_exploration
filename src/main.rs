#[macro_use]
extern crate rocket;
mod flood_fill;
mod game;
mod pathfinding;
mod strategy;
mod utils;

use crate::game::{GameState, Move};
use crate::strategy::{RandomStrategy, SimpleStrategy, Strategy, StrategyState};
use crate::utils::info;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::Value;
use std::env;

#[get("/")]
fn handle_info() -> Json<Value> {
    info!("INFO");
    Json(info())
}

#[post("/start", format = "json", data = "<game_state>")]
fn handle_start(game_state: Json<GameState>) -> Status {
    // TODO
    info!("START");
    Status::Ok
}

#[post("/move", format = "json", data = "<game_state>")]
fn handle_move(game_state: Json<GameState>, strategy: &State<StrategyState>) -> Json<Move> {
    // TODO
    println!("turn: {}", game_state.turn);
    info!("MOVE");
    Json(
        strategy
            .strategy
            .make_move(&game_state.game, &game_state.board, &game_state.you),
    )
}

#[post("/end", format = "json", data = "<game_state>")]
fn handle_game_over(game_state: Json<GameState>) -> Status {
    // TODO
    info!("GAME OVER");
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    // env_logger::init();

    let strategy_choice = env::args().nth(1).expect("No strategy choice given");

    let strategy: Box<dyn Strategy + Send + Sync> = match strategy_choice.as_str() {
        "random" => Box::new(RandomStrategy),
        "simple" => Box::new(SimpleStrategy),
        _ => panic!("Strategy {} not specified", strategy_choice),
    };

    rocket::build()
        .attach(rocket::fairing::AdHoc::on_liftoff("Startup msg", |_| {
            Box::pin(async move { info!("Battlesnake server started...") })
        }))
        .manage(StrategyState { strategy })
        .mount(
            "/",
            routes![handle_info, handle_start, handle_move, handle_game_over],
        )
}
