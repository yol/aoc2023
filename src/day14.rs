use std::{
    collections::BTreeMap,
    hash::{DefaultHasher, Hasher},
};

use super::util;
use grid::Grid;

const CUBE: u8 = 1;
const EMPTY: u8 = 0;
const ROUND: u8 = 2;

// 00:24:32
pub fn part1() {
    let lines = util::file_lines("inp14_2.txt");
    let first_line = &lines[0];
    let grid_vec: Vec<_> = lines
        .iter()
        .flat_map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => CUBE,
                    'O' => ROUND,
                    '.' => EMPTY,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let mut grid = Grid::from_vec(grid_vec, first_line.len());

    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            if grid[(y, x)] == ROUND {
                let mut move_by_y: Option<usize> = None;
                for y2 in 1..=y {
                    match grid[(y - y2, x)] {
                        EMPTY => {
                            move_by_y = Some(y2);
                        }
                        ROUND | CUBE => break,
                        _ => panic!(),
                    }
                }
                if let Some(move_by_y) = move_by_y {
                    grid[(y, x)] = EMPTY;
                    grid[(y - move_by_y, x)] = ROUND;
                }
            }
        }
    }

    let sum: usize = grid
        .indexed_iter()
        .map(|((y, _), &c)| if c == ROUND { grid.rows() - y } else { 0 })
        .sum();
    println!("{}", sum);
}

// 00:51:13
pub fn part2() {
    let lines = util::file_lines("inp14_2.txt");
    let first_line = &lines[0];
    let grid_vec: Vec<_> = lines
        .iter()
        .flat_map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => CUBE,
                    'O' => ROUND,
                    '.' => EMPTY,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let mut grid = Grid::from_vec(grid_vec, first_line.len());

    fn fall_north(grid: &mut Grid<u8>) {
        for y in 1..grid.rows() {
            for x in 0..grid.cols() {
                if grid[(y, x)] != ROUND {
                    continue;
                }
                let move_to_y = num::range_step_inclusive((y - 1) as isize, 0, -1)
                    .take_while(|&y2| grid[(y2 as usize, x)] == EMPTY)
                    .last();
                if let Some(move_to_y) = move_to_y {
                    grid[(y, x)] = EMPTY;
                    grid[(move_to_y as usize, x)] = ROUND;
                }
            }
        }
    }

    fn print_grid(grid: &Grid<u8>) {
        println!(
            "{}",
            grid.iter_rows()
                .map(|row| String::from_iter(row.map(|&c| match c {
                    CUBE => '#',
                    ROUND => 'O',
                    EMPTY => '.',
                    _ => panic!(),
                })))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
    fn grid_hash(grid: &Grid<u8>) -> u64 {
        let mut hasher = DefaultHasher::new();
        for ((y, x), _) in grid.indexed_iter().filter(|(_, &c)| c == ROUND) {
            hasher.write_usize(y);
            hasher.write_usize(x);
        }
        hasher.finish()
    }
    let mut grid_hashes_map = BTreeMap::new();
    let mut i = 0;
    while i < 1000000000 {
        println!("- {}", i);
        let i_hash = grid_hash(&grid);
        if let Err(insert_err) = grid_hashes_map.try_insert(i_hash, i) {
            let cycle_begin = insert_err.entry.get();
            let cycle_length = i - cycle_begin;
            println!("cycle at {} to {}, length {}", cycle_begin, i, cycle_length);
            // Skip to end
            i += (1000000000 - i) / cycle_length * cycle_length;
            grid_hashes_map.clear();
            continue;
        }

        // N
        fall_north(&mut grid);
        // W
        grid.transpose();
        fall_north(&mut grid);
        grid.transpose();
        // S
        grid.flip_rows();
        fall_north(&mut grid);
        grid.flip_rows();
        // E
        grid.flip_cols();
        grid.transpose();
        fall_north(&mut grid);
        grid.transpose();
        grid.flip_cols();

        i += 1;
    }

    let sum: usize = grid
        .indexed_iter()
        .map(|((y, _), &c)| if c == ROUND { grid.rows() - y } else { 0 })
        .sum();
    println!("{}", sum);
}
