use core::panic;

use crate::util::{build_grid, Direction, Position};

use super::util::file_lines;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum TileType {
    Garden,
    Rock,
    Start,
}

#[derive(Debug)]
struct Tile {
    tile_type: TileType,
    reached: bool,
}

fn parse_tile(c: char) -> Tile {
    Tile {
        tile_type: match c {
            '.' => TileType::Garden,
            '#' => TileType::Rock,
            'S' => TileType::Start,
            _ => panic!(),
        },
        reached: false,
    }
}

pub fn part1() {
    let lines = file_lines("inp21_2.txt");
    let grid = build_grid(&lines, parse_tile);

    struct QueueEntry {
        pos: Position,
        walk_len: usize,
    }
    let start_pos = Position::from_grid_pos(
        grid.indexed_iter()
            .find(|(_, tile)| tile.tile_type == TileType::Start)
            .unwrap()
            .0,
    );

    let mut plots = vec![start_pos];
    const WALK_MAX: usize = 64;
    for _i in 0..WALK_MAX {
        let mut next_plots = Vec::new();
        for pos in plots {
            for dir in Direction::all() {
                if let Some(next_pos) = pos.advance_in_grid(dir, &grid) {
                    if grid[next_pos.as_grid_pos()].tile_type != TileType::Rock {
                        next_plots.push(next_pos);
                    }
                }
            }
        }
        plots = next_plots.iter().unique().cloned().collect_vec();
    }

    let reached = plots.len();
    println!("{}", reached);
}

pub fn part2() {
    let lines = file_lines("inp21_1.txt");
}
