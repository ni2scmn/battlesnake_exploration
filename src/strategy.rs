use crate::game::{Battlesnake, Board, Direction, Game, Move};

use rand::Rng;

pub trait Strategy {
    fn make_move(&self, game: &Game, board: &Board, snake: &Battlesnake) -> Move;
}

pub struct StrategyState {
    pub strategy: Box<dyn Strategy + Send + Sync>,
}

pub struct RandomStrategy;

impl Strategy for RandomStrategy {
    fn make_move(&self, _: &Game, _: &Board, _: &Battlesnake) -> Move {
        let mut rng = rand::thread_rng();
        let num = rng.random_range(0..=3);

        let dir = match num {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Unknown direction {}", num),
        };

        Move { dir }
    }
}
