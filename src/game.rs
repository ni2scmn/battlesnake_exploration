use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Display;

// API and Response Objects
// See https://docs.battlesnake.com/api
#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    pub id: String,
    pub ruleset: HashMap<String, Value>,
    pub timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    pub height: i32,
    pub width: i32,
    pub food: Vec<Coord>,
    pub snakes: Vec<Battlesnake>,
    pub hazards: Vec<Coord>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Battlesnake {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub body: Vec<Coord>,
    pub head: Coord,
    pub length: i32,
    pub latency: String,
    pub shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn next_coord_in_dir(&self, dir: &Direction) -> Coord {
        match dir {
            Direction::Left => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Up => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Coord {
                x: self.x,
                y: self.y - 1,
            },
        }
    }

    pub fn neighbors(&self, board_size: (u32, u32)) -> Vec<Coord> {
        [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ]
        .iter()
        .map(|c| self.next_coord_in_dir(c))
        .filter(|n| n.x >= 0 && n.y >= 0 && n.x < board_size.0 as i32 && n.y < board_size.1 as i32)
        .collect()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    pub game: Game,
    pub turn: i32,
    pub board: Board,
    pub you: Battlesnake,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn all_directions() -> Vec<Direction> {
    vec![
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ]
}

impl Serialize for Direction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Direction::Left => serializer.serialize_str("left"),
            Direction::Right => serializer.serialize_str("right"),
            Direction::Up => serializer.serialize_str("up"),
            Direction::Down => serializer.serialize_str("down"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Direction::Left => write!(f, "left"),
            Direction::Right => write!(f, "right"),
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Move {
    #[serde(rename = "move")]
    pub dir: Direction,
}
