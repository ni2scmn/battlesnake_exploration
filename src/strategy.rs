use crate::game::{all_directions, Battlesnake, Board, Direction, Game, Move};

use crate::flood_fill::flood_fill;
use rand::rng;
use rand::seq::IteratorRandom;
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

        let pos_up = head.next_coord_in_dir(&Direction::Up);
        let pos_down = head.next_coord_in_dir(&Direction::Down);
        let pos_left = head.next_coord_in_dir(&Direction::Left);
        let pos_right = head.next_coord_in_dir(&Direction::Right);

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

fn flood_fill_all_directions(
    directions: &[Direction],
    board: &Board,
    snake: &Battlesnake,
) -> Vec<(Direction, usize)> {
    directions
        .iter()
        .map(|direction| {
            let blocked_pos = snake.body.clone();
            let reachable_field = flood_fill(
                snake.head.next_coord_in_dir(direction),
                &blocked_pos,
                board.height,
                board.width,
            );
            (*direction, reachable_field.len())
        })
        .collect()
}

impl Strategy for SimpleStrategy {
    fn make_move(&self, game: &Game, board: &Board, snake: &Battlesnake) -> Move {
        let mut possible_moves = all_directions();

        self.prevent_self_collision(&mut possible_moves, snake);
        self.prevent_out_of_bounds(&mut possible_moves, board, snake);

        let flood_fill_scores = flood_fill_all_directions(&possible_moves, board, snake);

        let max_ff_score = flood_fill_scores
            .iter()
            .map(|(_, v)| v)
            .max()
            .unwrap_or(&usize::MAX);

        let chosen_move = flood_fill_scores
            .iter()
            .filter(|(_, v)| *v == *max_ff_score)
            .choose(&mut rand::rng());

        if let Some((best_dir, score)) = chosen_move {
            Move { dir: *best_dir }
        } else {
            // No legal move found
            Move {
                dir: Direction::Down,
            }
        }
    }
}
