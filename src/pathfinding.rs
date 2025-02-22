use crate::game::Coord;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;

fn manhatten_distance(pos: Coord, targets: Vec<Coord>) -> Vec<u32> {
    targets
        .iter()
        .map(|target| ((pos.x - target.x).abs() + (pos.y - target.y).abs()) as u32)
        .collect()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct DijkQueueItem {
    estimated_cost: u32,
    position: Coord,
}

impl PartialOrd for DijkQueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkQueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.estimated_cost.cmp(&other.estimated_cost).reverse()
    }
}

pub fn dijkstra(
    start: Coord,
    goal: &[Coord],
    board_size: (u32, u32),
    blocked_pos: &[Coord],
) -> Vec<u32> {
    let mut unvisited = BinaryHeap::new();
    let mut distances = HashMap::new();

    distances.insert(start, 0);
    unvisited.push(DijkQueueItem {
        estimated_cost: 0,
        position: start,
    });

    while !unvisited.is_empty() {
        let current = unvisited.pop().unwrap();

        if blocked_pos.contains(&current.position) {
            continue;
        }

        current
            .position
            .neighbors(board_size)
            .iter()
            .for_each(|neighbor| {
                let current_dist = *distances.get(neighbor).unwrap_or(&u32::MAX);
                let alt_dist = *distances.get(&current.position).unwrap() + 1;
                if alt_dist < current_dist {
                    unvisited.push(DijkQueueItem {
                        position: *neighbor,
                        estimated_cost: alt_dist,
                    });
                    distances.insert(*neighbor, alt_dist);
                }
            })
    }

    goal.iter()
        .map(|c| *distances.get(c).unwrap_or(&u32::MAX))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_target() {
        let pos = Coord { x: 0, y: 0 };
        let targets = vec![Coord { x: 3, y: 4 }];
        assert_eq!(manhatten_distance(pos, targets), vec![7]);
    }

    #[test]
    fn test_multiple_targets() {
        let pos = Coord { x: 1, y: 1 };
        let targets = vec![Coord { x: 4, y: 5 }, Coord { x: -2, y: -3 }];
        assert_eq!(manhatten_distance(pos, targets), vec![7, 7]);
    }

    #[test]
    fn test_same_position() {
        let pos = Coord { x: 2, y: 2 };
        let targets = vec![Coord { x: 2, y: 2 }];
        assert_eq!(manhatten_distance(pos, targets), vec![0]);
    }

    #[test]
    fn test_empty_targets() {
        let pos = Coord { x: 5, y: 5 };
        let targets = vec![];
        assert_eq!(manhatten_distance(pos, targets), Vec::<u32>::new());
    }

    #[test]
    fn test_negative_coordinates() {
        let pos = Coord { x: -3, y: -3 };
        let targets = vec![Coord { x: -1, y: -6 }];
        assert_eq!(manhatten_distance(pos, targets), vec![5]);
    }

    #[test]
    fn test_large_numbers() {
        let pos = Coord {
            x: 1_000_000,
            y: 1_000_000,
        };
        let targets = vec![Coord {
            x: 2_000_000,
            y: 3_000_000,
        }];
        assert_eq!(manhatten_distance(pos, targets), vec![3_000_000]);
    }

    #[test]
    fn test_mixed_sign_coordinates() {
        let pos = Coord { x: -5, y: 5 };
        let targets = vec![Coord { x: 5, y: -5 }];
        assert_eq!(manhatten_distance(pos, targets), vec![20]);
    }

    #[test]
    fn test_direct_path() {
        let start = Coord { x: 0, y: 0 };
        let goal = vec![Coord { x: 3, y: 3 }];
        let board_size = (5, 5);
        let blocked_pos = vec![];

        assert_eq!(dijkstra(start, &goal, board_size, &blocked_pos), vec![6]);
    }

    #[test]
    fn test_with_obstacles() {
        let start = Coord { x: 0, y: 2 };
        let goal = vec![Coord { x: 2, y: 2 }];
        let board_size = (3, 3);
        let blocked_pos = vec![Coord { x: 1, y: 2 }, Coord { x: 1, y: 1 }];

        assert_eq!(dijkstra(start, &goal, board_size, &blocked_pos), vec![6]);
    }

    #[test]
    fn test_with_obstacles2() {
        let start = Coord { x: 4, y: 3 };
        // let goal = Coord { x: 2, y: 0 };
        let goal = vec![Coord { x: 2, y: 0 }];
        let board_size = (6, 6);

        let blocked_pos = vec![
            Coord { x: 1, y: 4 },
            Coord { x: 1, y: 3 },
            Coord { x: 1, y: 2 },
            Coord { x: 2, y: 2 },
            Coord { x: 3, y: 2 },
            Coord { x: 4, y: 2 },
            Coord { x: 5, y: 2 },
        ];

        assert_eq!(dijkstra(start, &goal, board_size, &blocked_pos), vec![13]);
    }

    #[test]
    fn test_no_path() {
        let start = Coord { x: 0, y: 0 };
        let goal = vec![Coord { x: 3, y: 3 }];
        let board_size = (5, 5);
        let blocked_pos = (0..5).map(|i| Coord { x: i, y: 2 }).collect::<Vec<_>>();

        assert_eq!(
            dijkstra(start, &goal, board_size, &blocked_pos),
            vec![u32::MAX]
        );
    }

    #[test]
    fn test_same_start_and_goal() {
        let start = Coord { x: 2, y: 2 };
        let goal = vec![Coord { x: 2, y: 2 }];
        let board_size = (5, 5);
        let blocked_pos = vec![];

        assert_eq!(dijkstra(start, &goal, board_size, &blocked_pos), vec![0]);
    }

    #[test]
    fn test_forced_longer_path() {
        let start = Coord { x: 0, y: 0 };
        let goal = vec![Coord { x: 4, y: 4 }];
        let board_size = (5, 5);
        let blocked_pos = vec![
            Coord { x: 2, y: 0 },
            Coord { x: 2, y: 1 },
            Coord { x: 2, y: 2 },
        ];

        assert_eq!(dijkstra(start, &goal, board_size, &blocked_pos), vec![8]);
    }

    #[test]
    fn test_corner_to_corner_with_blockage() {
        let start = Coord { x: 0, y: 0 };
        let goal = vec![Coord { x: 4, y: 4 }];
        let board_size = (5, 5);
        let blocked_pos = vec![
            Coord { x: 1, y: 0 },
            Coord { x: 1, y: 1 },
            Coord { x: 1, y: 2 },
            Coord { x: 1, y: 3 },
            Coord { x: 3, y: 4 },
        ];

        assert_eq!(dijkstra(start, &goal, board_size, &blocked_pos), vec![10]);
    }
}
