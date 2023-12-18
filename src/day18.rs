use std::collections::VecDeque;

use super::util::{file_lines, print_grid, Direction, Position};
use grid::Grid;
use itertools::Itertools;

#[derive(Debug, Default, PartialEq, Eq)]
struct Tile {
    dug: bool,
}

fn tile_print_dug(t: &Tile) -> char {
    match t.dug {
        true => '#',
        false => '.',
    }
}

pub fn part1() {
    let lines = file_lines("inp18_2.txt");
    const SIZE: usize = 700;

    let mut grid: Grid<Tile> = Grid::new(SIZE, SIZE);

    let start_pos = Position {
        x: SIZE / 2,
        y: SIZE / 2,
    };
    {
        let mut pos = start_pos;
        grid[pos.as_grid_pos()].dug = true;

        for instruction in lines {
            let instr_parts = instruction.split_whitespace().collect_vec();
            let dir = match instr_parts[0] {
                "U" => Direction::N,
                "R" => Direction::E,
                "D" => Direction::S,
                "L" => Direction::W,
                _ => panic!(),
            };
            let dist: u32 = instr_parts[1].parse().unwrap();

            for _ in 0..dist {
                pos = pos.advance_in_grid(dir, &grid).unwrap();
                grid[pos.as_grid_pos()].dug = true;
            }
        }
    }

    print_grid(&grid, tile_print_dug);

    // Flood fill
    {
        let mut q = VecDeque::from([Position {
            // FIXME This is specific to the input
            x: start_pos.x - 1,
            y: start_pos.y - 1,
        }]);
        while let Some(pos) = q.pop_back() {
            grid[pos.as_grid_pos()].dug = true;
            for dir in Direction::all() {
                let new_pos = pos.advance_in_grid(dir, &grid).unwrap();
                if !grid[new_pos.as_grid_pos()].dug {
                    q.push_back(new_pos);
                }
            }
        }
    }

    let sum = grid.iter().filter(|t| t.dug).count();
    println!("{}", sum);
}

pub fn part2() {
    let lines = file_lines("inp18_1.txt");
}
