use crate::game::{all_directions, Battlesnake, Board, Coord, Direction, Game, Move};

use crate::flood_fill::flood_fill;
use crate::pathfinding::dijkstra;
use rand::rng;

use rand::prelude::IndexedRandom;
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
    start: &Coord,
    board_size: (i32, i32),
    blocked_pos: &[Coord],
) -> Vec<(Direction, usize)> {
    all_directions()
        .iter()
        .map(|d| (d, start.next_coord_in_dir(d)))
        .filter(|(_, c)| c.x >= 0 && c.y >= 0 && c.x < board_size.0 && c.y < board_size.1)
        .map(|(d, c)| {
            let reachable_field = flood_fill(
                c,
                &blocked_pos,
                board_size.0,
                board_size.1,
            );
            (*d, reachable_field.len())
        })
        .collect()
}

impl Strategy for SimpleStrategy {
    fn make_move(&self, game: &Game, board: &Board, snake: &Battlesnake) -> Move {
        let mut possible_moves = all_directions();
        let goals = &board.food;
        let board_size = (board.width, board.height);

        let blocked_pos = board
            .snakes
            .iter()
            .map(|s| s.body.clone())
            .flatten()
            .collect::<Vec<Coord>>();

        self.prevent_self_collision(&mut possible_moves, snake);
        self.prevent_out_of_bounds(&mut possible_moves, board, snake);

        let flood_fill_scores = flood_fill_all_directions(&snake.head, board_size, &blocked_pos);
        let dijk_res = dijkstra(
            snake.head,
            board_size,
            &blocked_pos, //&snake.body,
        );

        let max_ff_score = flood_fill_scores
            .iter()
            .map(|(_, v)| v)
            .max()
            .unwrap_or(&usize::MAX);

        let non_blocking_dirs = flood_fill_scores
            .iter()
            .filter(|(_, v)| *v == *max_ff_score)
            .map(|(k, _)| k)
            .collect::<Vec<_>>();

        let closest_food_dir = dijk_res.get_direction_for_shortest_goal(goals);

        if let Some(cfd) = closest_food_dir {
            if non_blocking_dirs.contains(&&cfd) {
                return Move { dir: cfd };
            }
        }

        if !non_blocking_dirs.is_empty() {
            return Move {
                dir: **non_blocking_dirs.choose(&mut rand::rng()).unwrap(),
            };
        }

        // No legal move found
        Move {
            dir: Direction::Down,
        }
    }
}
