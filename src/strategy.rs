use crate::game::{all_directions, Battlesnake, Board, Coord, Direction, Game, Move};

use rand::rng;
use rand::seq::IteratorRandom; // for `choose` method
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
        let mut rng = rng();
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

pub struct SimpleStrategy;

impl SimpleStrategy {
    fn prevent_self_collision(&self, moves: &mut Vec<Direction>, snake: &Battlesnake) {
        let head = &snake.head;

        let pos_up = Coord {
            x: head.x,
            y: head.y + 1,
        };
        let pos_down = Coord {
            x: head.x,
            y: head.y - 1,
        };
        let pos_left = Coord {
            x: head.x - 1,
            y: head.y,
        };
        let pos_right = Coord {
            x: head.x + 1,
            y: head.y,
        };

        snake.body.iter().for_each(|bpos| {
            if *bpos == pos_left {
                moves.retain(|mv| *mv != Direction::Left);
            }
            if *bpos == pos_right {
                moves.retain(|mv| *mv != Direction::Right);
            }
            if *bpos == pos_up {
                moves.retain(|mv| *mv != Direction::Up);
            }
            if *bpos == pos_down {
                moves.retain(|mv| *mv != Direction::Down);
            }
        })
    }

    fn prevent_out_of_bounds(
        &self,
        moves: &mut Vec<Direction>,
        board: &Board,
        snake: &Battlesnake,
    ) {
        let head = &snake.head;

        let b_height = board.height;
        let b_width = board.width;


        if head.x == 0 {
            moves.retain(|mv| *mv != Direction::Left);
        }
        if head.x == b_width - 1 {
            moves.retain(|mv| *mv != Direction::Right);
        }
        if head.y == 0 {
            moves.retain(|mv| *mv != Direction::Down);
        }
        if head.y == b_height - 1 {
            moves.retain(|mv| *mv != Direction::Up);
        }
    }
}

impl Strategy for SimpleStrategy {
    fn make_move(&self, game: &Game, board: &Board, snake: &Battlesnake) -> Move {
        let mut possible_moves = all_directions();

        self.prevent_self_collision(&mut possible_moves, snake);
        self.prevent_out_of_bounds(&mut possible_moves, board, snake);

        let mut rng = rng();

        let move_dir = match possible_moves.iter().choose(&mut rng) {
            None => {
                Direction::Down // Default direction
            }
            Some(d) => *d,
        };

        println!("moving to {:?}", move_dir);

        Move { dir: move_dir }
    }
}
