use crate::game::{get_direction_from_to, Coord, Direction};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;

#[derive(Debug, Clone, Eq, PartialEq)]
struct DijkQueueItem {
    estimated_cost: u32,
    position: Coord,
}

#[derive(Debug, Clone)]
pub struct DijkResult {
    start: Coord,
    distances: HashMap<Coord, u32>,
    predecessors: HashMap<Coord, Coord>,
}

impl DijkResult {
    pub fn get_distance_ref(&self) -> &HashMap<Coord, u32> {
        &self.distances
    }

    pub fn get_predecessor_ref(&self) -> &HashMap<Coord, Coord> {
        &self.predecessors
    }

    pub fn get_paths_for(&self, coords: &[Coord]) -> HashMap<Coord, Vec<Coord>> {
        let mut paths = HashMap::new();
        for &coord in coords {
            let mut path = Vec::new();
            path.push(coord);
            let mut current = coord;
            while let Some(&predecessor) = self.predecessors.get(&current) {
                path.push(predecessor);
                current = predecessor;
            }
            path.reverse();
            paths.insert(coord, path);
        }
        paths
    }

    pub fn retrieve_distances_for<'a, 'b>(
        &'a self,
        coords: &'b [Coord],
    ) -> impl Iterator<Item = (&'b Coord, &'a u32)> {
        coords
            .iter()
            .map(|coord| (coord, self.distances.get(&coord).unwrap_or(&u32::MAX)))
    }

    pub fn get_direction_for_shortest_goal(&self, coords: &[Coord]) -> Option<Direction> {
        let closest_goal = self
            .retrieve_distances_for(coords)
            .min_by_key(|(c, d)| **d)?
            .0;

        let paths = self.get_paths_for(&[closest_goal.clone()]);

        let path = paths.get(closest_goal)?;

        let first_step = path.get(1)?;

        get_direction_from_to(&self.start, first_step)
    }
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

fn manhatten_distance(pos: Coord, targets: Vec<Coord>) -> Vec<u32> {
    targets
        .iter()
        .map(|target| ((pos.x - target.x).abs() + (pos.y - target.y).abs()) as u32)
        .collect()
}

pub fn dijkstra(start: Coord, board_size: (u32, u32), blocked_pos: &[Coord]) -> DijkResult {
    let mut unvisited = BinaryHeap::new();
    let mut distances = HashMap::with_capacity(board_size.0 as usize * board_size.1 as usize);
    let mut predecessors =
        HashMap::<Coord, Coord>::with_capacity(board_size.0 as usize * board_size.1 as usize);

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
                    predecessors.insert(*neighbor, current.position);
                }
            })
    }

    DijkResult {
        start,
        distances,
        predecessors,
    }
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
        let goal = Coord { x: 3, y: 3 };
        let board_size = (5, 5);
        let blocked_pos = vec![];

        assert_eq!(
            dijkstra(start, board_size, &blocked_pos)
                .get_distance_ref()
                .get(&goal)
                .unwrap()
                .clone(),
            6
        );
    }

    #[test]
    fn test_with_obstacles() {
        let start = Coord { x: 0, y: 2 };
        let goal = Coord { x: 2, y: 2 };
        let board_size = (3, 3);
        let blocked_pos = vec![Coord { x: 1, y: 2 }, Coord { x: 1, y: 1 }];

        assert_eq!(
            dijkstra(start, board_size, &blocked_pos)
                .get_distance_ref()
                .get(&goal)
                .unwrap()
                .clone(),
            6
        );
    }
    #[test]
    fn test_with_obstacles2() {
        let start = Coord { x: 4, y: 3 };
        let goal = Coord { x: 2, y: 0 };
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

        assert_eq!(
            dijkstra(start, board_size, &blocked_pos)
                .get_distance_ref()
                .get(&goal)
                .unwrap()
                .clone(),
            13
        );
    }

    #[test]
    fn test_no_path() {
        let start = Coord { x: 0, y: 0 };
        let goal = Coord { x: 3, y: 3 };
        let board_size = (5, 5);
        let blocked_pos = (0..5).map(|i| Coord { x: i, y: 2 }).collect::<Vec<_>>();

        assert_eq!(
            dijkstra(start, board_size, &blocked_pos)
                .get_distance_ref()
                .get(&goal)
                .unwrap_or(&u32::MAX)
                .clone(),
            u32::MAX
        );
    }

    #[test]
    fn test_same_start_and_goal() {
        let start = Coord { x: 2, y: 2 };
        let goal = Coord { x: 2, y: 2 };
        let board_size = (5, 5);
        let blocked_pos = vec![];

        assert_eq!(
            dijkstra(start, board_size, &blocked_pos)
                .get_distance_ref()
                .get(&goal)
                .unwrap()
                .clone(),
            0
        );
    }

    #[test]
    fn test_forced_longer_path() {
        let start = Coord { x: 0, y: 0 };
        let goal = Coord { x: 4, y: 4 };
        let board_size = (5, 5);
        let blocked_pos = vec![
            Coord { x: 2, y: 0 },
            Coord { x: 2, y: 1 },
            Coord { x: 2, y: 2 },
        ];

        assert_eq!(
            dijkstra(start, board_size, &blocked_pos)
                .get_distance_ref()
                .get(&goal)
                .unwrap()
                .clone(),
            8
        );
    }

    #[test]
    fn test_corner_to_corner_with_blockage() {
        let start = Coord { x: 0, y: 0 };
        let goal = Coord { x: 4, y: 4 };
        let board_size = (5, 5);
        let blocked_pos = vec![
            Coord { x: 1, y: 0 },
            Coord { x: 1, y: 1 },
            Coord { x: 1, y: 2 },
            Coord { x: 1, y: 3 },
            Coord { x: 3, y: 4 },
        ];

        assert_eq!(
            dijkstra(start, board_size, &blocked_pos)
                .get_distance_ref()
                .get(&goal)
                .unwrap()
                .clone(),
            10
        );
    }

    #[test]
    fn test_path_for_dijkstra() {
        let mut start = Coord { x: 2, y: 2 };
        let mut goal = Coord { x: 0, y: 0 };
        let board_size = (3, 3);

        let mut blocked_pos = vec![Coord { x: 1, y: 1 }, Coord { x: 2, y: 1 }];
        let mut dijk_result = dijkstra(start, board_size, &blocked_pos);
        let mut path = dijk_result
            .get_paths_for(&[goal])
            .get(&goal)
            .unwrap()
            .clone();
        let mut expected_path = vec![
            Coord { x: 2, y: 2 },
            Coord { x: 1, y: 2 },
            Coord { x: 0, y: 2 },
            Coord { x: 0, y: 1 },
            Coord { x: 0, y: 0 },
        ];

        assert_eq!(path, expected_path);

        start = Coord { x: 0, y: 2 };
        goal = Coord { x: 2, y: 0 };
        blocked_pos = vec![Coord { x: 1, y: 0 }, Coord { x: 1, y: 1 }];
        dijk_result = dijkstra(start, board_size, &blocked_pos);
        path = dijk_result
            .get_paths_for(&[goal])
            .get(&goal)
            .unwrap()
            .clone();
        expected_path = vec![
            Coord { x: 0, y: 2 },
            Coord { x: 1, y: 2 },
            Coord { x: 2, y: 2 },
            Coord { x: 2, y: 1 },
            Coord { x: 2, y: 0 },
        ];

        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_shortest_path_direction_for_dijkstra() {
        let mut start = Coord { x: 2, y: 2 };
        let mut goal = Coord { x: 0, y: 0 };
        let board_size = (3, 3);

        let mut blocked_pos = vec![Coord { x: 1, y: 1 }, Coord { x: 2, y: 1 }];
        let mut dijk_result = dijkstra(start, board_size, &blocked_pos);
        let mut s_path_dir = dijk_result
            .get_direction_for_shortest_goal(&[goal])
            .unwrap();

        assert_eq!(s_path_dir, Direction::Left);

        start = Coord { x: 0, y: 2 };
        goal = Coord { x: 2, y: 0 };
        blocked_pos = vec![Coord { x: 1, y: 0 }, Coord { x: 1, y: 1 }];
        dijk_result = dijkstra(start, board_size, &blocked_pos);
        s_path_dir = dijk_result
            .get_direction_for_shortest_goal(&[goal])
            .unwrap();
        assert_eq!(s_path_dir, Direction::Right);
    }
}
