use crate::util::parse_digit;

use super::util::{build_grid, file_lines, Direction, Position};
use grid::Grid;

use itertools::Itertools;
use pathfinding::prelude::astar;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct SearchState {
    pos: Position,
    dir: Option<Direction>,
    steps_in_dir: usize,
}

impl SearchState {
    fn advance_in_grid_with_dir(&self, grid: &Grid<u8>, dir: Direction) -> Option<SearchState> {
        let mut new_steps = self.steps_in_dir + 1;
        if let Some(cur_dir) = self.dir {
            if cur_dir != dir {
                new_steps = 1;
            }
        }
        if new_steps > 3 {
            return None;
        }
        Some(SearchState {
            pos: self.pos.advance_in_grid(dir, &grid)?,
            dir: Some(dir),
            steps_in_dir: new_steps,
        })
    }

    fn advance_in_grid_with_dir_2(
        &self,
        grid: &Grid<u8>,
        new_dir: Direction,
    ) -> Option<(SearchState, usize)> {
        const MIN_STRAIGHT_DISTANCE: usize = 4;
        const MAX_STRAIGHT_DISTANCE: usize = 10;

        let direction_changes = match self.dir {
            None => true,
            Some(dir) => dir != new_dir,
        };
        let advance_by = match direction_changes {
            false => 1,
            // Minimum movement size after turning
            true => MIN_STRAIGHT_DISTANCE,
        };
        let new_steps = advance_by
            + match direction_changes {
                false => self.steps_in_dir,
                true => 0,
            };
        if new_steps > MAX_STRAIGHT_DISTANCE {
            return None;
        }

        let mut cost = 0;
        let mut new_pos = self.pos;
        for _ in 0..advance_by {
            new_pos = new_pos.advance_in_grid(new_dir, &grid)?;
            cost += grid[new_pos.as_grid_pos()] as usize;
        }
        Some((
            SearchState {
                pos: new_pos,
                dir: Some(new_dir),
                steps_in_dir: new_steps,
            },
            cost,
        ))
    }
}

pub fn part1() {
    let lines = file_lines("inp17_2.txt");

    let first_line = &lines[0];
    let grid_vec: Vec<_> = lines
        .iter()
        .flat_map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect();
    let grid = Grid::from_vec(grid_vec, first_line.len());

    let start_pos = Position { x: 0, y: 0 };
    let end_pos = Position {
        x: grid.cols() - 1,
        y: grid.rows() - 1,
    };
    fn successor_tuple(new_state: &SearchState, grid: &Grid<u8>) -> (SearchState, usize) {
        (*new_state, grid[new_state.pos.as_grid_pos()] as usize)
    }
    let result = astar(
        &SearchState {
            pos: start_pos,
            dir: None,
            steps_in_dir: 0,
        },
        |s| match s.dir {
            None => {
                // At the start: try both possible starting directions
                vec![
                    successor_tuple(
                        &s.advance_in_grid_with_dir(&grid, Direction::E).unwrap(),
                        &grid,
                    ),
                    successor_tuple(
                        &s.advance_in_grid_with_dir(&grid, Direction::S).unwrap(),
                        &grid,
                    ),
                ]
            }
            Some(dir) => {
                let mut succ = vec![];

                let mut try_dir = |dir| {
                    if let Some(next_state) = s.advance_in_grid_with_dir(&grid, dir) {
                        succ.push(successor_tuple(&next_state, &grid));
                    }
                };

                for new_dir in [Direction::N, Direction::E, Direction::S, Direction::W] {
                    if new_dir == dir.opposite() {
                        // Never go back
                        continue;
                    }
                    try_dir(new_dir);
                }

                succ
            }
        },
        |s| s.pos.manhattan_distance_to(end_pos),
        |s| s.pos == end_pos,
    );
    let cost = result.unwrap().1;
    println!("{}", cost);
}

pub fn part2() {
    let lines = file_lines("inp17_2.txt");

    let grid = build_grid(&lines, parse_digit);

    let start_pos = Position { x: 0, y: 0 };
    let end_pos = Position {
        x: grid.cols() - 1,
        y: grid.rows() - 1,
    };
    let result = astar(
        &SearchState {
            pos: start_pos,
            dir: None,
            steps_in_dir: 0,
        },
        |s| {
            Direction::all()
                .filter_map(|new_dir| {
                    if s.dir.is_some() && new_dir == s.dir.unwrap().opposite() {
                        // Never go back
                        return None;
                    }
                    s.advance_in_grid_with_dir_2(&grid, new_dir)
                })
                .collect_vec()
        },
        |s| s.pos.manhattan_distance_to(end_pos),
        |s| s.pos == end_pos,
    );
    let cost = result.unwrap().1;
    println!("{}", cost);
}
