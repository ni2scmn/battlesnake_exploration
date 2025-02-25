use crate::game::{all_directions, Coord};
use std::collections::VecDeque;

pub fn flood_fill(
    start: Coord,
    blocked_coords: &[Coord],
    board_width: i32,
    board_height: i32,
) -> Vec<Coord> {
    if blocked_coords.iter().any(|c| *c == start) {
        return vec![];
    }
    let idx = |x: i32, y: i32| -> usize { (y * board_width + x) as usize };

    let mut visited = vec![false; (board_width * board_height) as usize];
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    queue.push_back(start);
    visited[idx(start.x, start.y)] = true;

    while let Some(current) = queue.pop_front() {
        result.push(current);

        for direction in &all_directions() {
            let next = current.next_coord_in_dir(direction);

            if next.y >= 0
                && next.x >= 0
                && next.x < board_width
                && next.y < board_height
                && !blocked_coords.iter().any(|c| *c == next)
                && !visited[idx(next.x, next.y)]
            {
                visited[idx(next.x, next.y)] = true;
                queue.push_back(next);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unblocked_flood_fill() {
        let blocked_coords: Vec<Coord> = vec![];
        let start = Coord { x: 0, y: 0 };
        for h in (1..100).step_by(10) {
            for w in (1..100).step_by(10) {
                let filled_area = flood_fill(start, &blocked_coords, h, w);
                assert_eq!(filled_area.len(), (w * h) as usize);
            }
        }
    }

    // 4x4
    // OOOO
    // XXOO
    // OOXO
    // SOOX
    // S: (0,0)  B: (3,0);(2,1);(0,2);(1,2)
    #[test]
    fn test_simple_wall_flood_fill() {
        let blocked_coords: Vec<Coord> = vec![
            Coord { x: 0, y: 2 },
            Coord { x: 1, y: 2 },
            Coord { x: 2, y: 1 },
            Coord { x: 3, y: 0 },
        ];
        let start = Coord { x: 0, y: 0 };
        let filled_area = flood_fill(start, &blocked_coords, 4, 4);

        assert_eq!(filled_area.len(), 5);
    }

    // 5x5
    // OOOOO
    // OOOOO
    // OXOSO
    // OXXXX
    // OXOOO
    // S: (3,2)  B: (1,0);(1,1);(2,1);(3,1);(4,1);(1,2)
    #[test]
    fn test_basic_flood_fill() {
        let blocked_coords = vec![
            Coord { x: 1, y: 0 },
            Coord { x: 1, y: 1 },
            Coord { x: 2, y: 1 },
            Coord { x: 3, y: 1 },
            Coord { x: 4, y: 1 },
            Coord { x: 1, y: 2 },
        ];
        let start = Coord { x: 3, y: 2 };
        let max_x = 5;
        let max_y = 5;

        let mut filled_area = flood_fill(start, &blocked_coords, max_x, max_y);

        // RRRRR
        // RXRSR
        // RXXXX
        // RXOOO
        let mut expected = vec![
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 1 },
            Coord { x: 0, y: 2 },
            Coord { x: 2, y: 2 },
            Coord { x: 3, y: 2 },
            Coord { x: 4, y: 2 },
            Coord { x: 0, y: 3 },
            Coord { x: 1, y: 3 },
            Coord { x: 2, y: 3 },
            Coord { x: 3, y: 3 },
            Coord { x: 4, y: 3 },
            Coord { x: 0, y: 4 },
            Coord { x: 1, y: 4 },
            Coord { x: 2, y: 4 },
            Coord { x: 3, y: 4 },
            Coord { x: 4, y: 4 },
        ];

        filled_area.sort();
        expected.sort();

        assert_eq!(filled_area.len(), expected.len());
        assert_eq!(filled_area, expected);
    }

    #[test]
    fn test_all_cells_blocked() {
        let blocked_coords = vec![
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 1 },
            Coord { x: 0, y: 2 },
            Coord { x: 0, y: 3 },
            Coord { x: 0, y: 4 },
            Coord { x: 1, y: 0 },
            Coord { x: 1, y: 1 },
            Coord { x: 1, y: 2 },
            Coord { x: 1, y: 3 },
            Coord { x: 1, y: 4 },
            Coord { x: 2, y: 0 },
            Coord { x: 2, y: 1 },
            Coord { x: 2, y: 2 },
            Coord { x: 2, y: 3 },
            Coord { x: 2, y: 4 },
            Coord { x: 3, y: 0 },
            Coord { x: 3, y: 1 },
            Coord { x: 3, y: 2 },
            Coord { x: 3, y: 3 },
            Coord { x: 3, y: 4 },
            Coord { x: 4, y: 0 },
            Coord { x: 4, y: 1 },
            Coord { x: 4, y: 2 },
            Coord { x: 4, y: 3 },
            Coord { x: 4, y: 4 },
        ];
        let start = Coord { x: 2, y: 2 };
        let max_x = 5;
        let max_y = 5;

        let filled_area = flood_fill(start, &blocked_coords, max_x, max_y);

        // Expected result: No cells are filled since everything is blocked
        let expected: Vec<Coord> = Vec::new();

        assert_eq!(filled_area, expected);
    }

    #[test]
    fn test_flood_fill_with_isolated_blocked_cells() {
        let blocked_coords = vec![Coord { x: 1, y: 0 }, Coord { x: 0, y: 1 }];
        let start = Coord { x: 0, y: 0 };
        let max_x = 5;
        let max_y = 5;

        let filled_area = flood_fill(start, &blocked_coords, max_x, max_y);

        let expected = vec![Coord { x: 0, y: 0 }];

        assert_eq!(filled_area, expected);
    }

    #[test]
    fn test_edge_case_with_single_cell() {
        let blocked_coords = vec![];
        let start = Coord { x: 0, y: 0 };
        let max_x = 1;
        let max_y = 1;

        let filled_area = flood_fill(start, &blocked_coords, max_x, max_y);

        // Expected result: only the starting cell should be filled
        let expected = vec![Coord { x: 0, y: 0 }];
        assert_eq!(filled_area, expected);
    }
}
