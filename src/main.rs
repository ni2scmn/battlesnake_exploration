mod game;
mod utils;

#[macro_use]
extern crate rocket;

use crate::game::{Battlesnake, Board, Direction, Game, GameState, Move};
use crate::utils::info;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::{json, Value};

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
fn handle_move(game_state: Json<GameState>) -> Json<Move> {
    // TODO
    info!("MOVE");
    Json(Move {
        dir: Direction::Right,
    })
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

    rocket::build()
        .attach(rocket::fairing::AdHoc::on_liftoff("Startup msg", |_| {
            Box::pin(async move { info!("Battlesnake server started...") })
        }))
        .mount(
            "/",
            routes![handle_info, handle_start, handle_move, handle_game_over],
        )
}
