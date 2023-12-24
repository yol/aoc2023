use std::collections::BTreeSet;

use crate::util::{build_grid, file_lines, Direction, Position};

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

    let mut plots: BTreeSet<Position> = vec![start_pos].into_iter().collect();
    const WALK_MAX: usize = 64;
    for _i in 0..WALK_MAX {
        let mut next_plots = BTreeSet::<Position>::new();
        for pos in &plots {
            for dir in Direction::all() {
                if let Some(next_pos) = pos.advance_in_grid(dir, &grid) {
                    if grid[next_pos.as_grid_pos()].tile_type != TileType::Rock {
                        next_plots.insert(next_pos);
                    }
                }
            }
        }
        plots = next_plots;
    }

    let reached = plots.len();
    println!("{}", reached);
}

pub fn part2() {
    let lines = file_lines("inp21_1.txt");
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
    /*start_pos.x += 500 * grid.cols() as isize;
    start_pos.y += 500 * grid.rows() as isize;*/

    // Count possible steps for one fully filled grid unit
    let mut plot_count_even = 0_u64;
    let mut plot_count_odd = 0_u64;
    {
        // Start: odd step number (=1 step)
        let mut plots = BTreeSet::from([start_pos]);
        let mut done_plots = BTreeSet::new();

        for i in 1_u64.. {
            let mut next_plots = BTreeSet::new();
            for pos in &plots {
                for dir in Direction::all() {
                    if let Some(next_pos) = pos.advance_in_grid(dir, &grid) {
                        if grid[next_pos.as_grid_pos()].tile_type != TileType::Rock
                            && !done_plots.contains(&next_pos)
                        {
                            next_plots.insert(next_pos);
                            done_plots.insert(next_pos);
                            if i % 2 == 0 {
                                plot_count_even += 1;
                            } else {
                                plot_count_odd += 1;
                            }
                        }
                    }
                }
            }
            plots = next_plots;

            if plots.is_empty() {
                // Discovered all plots in the atomic garden unit
                break;
            }
        }
    }

    // Start: odd step number (=1 step)
    let mut plots = BTreeSet::from([start_pos]);
    let mut done_plots = BTreeSet::<Position>::new();

    const WALK_MAX: usize = 50;

    let mut j = 0;
    for i in 1..=WALK_MAX {
        let mut next_plots = BTreeSet::<Position>::new();
        for pos in &plots {
            for dir in Direction::all() {
                let next_pos = pos.advance_in_dir_by(dir, 1);
                if grid[next_pos.as_grid_pos_in_repeating_grid(&grid)].tile_type != TileType::Rock
                    && !done_plots.contains(&next_pos)
                {
                    next_plots.insert(next_pos);
                    done_plots.insert(next_pos);
                    if i % 2 == 0 {
                        plot_count_even += 1;
                    } else {
                        plot_count_odd += 1;
                    }
                }
            }
        }
        plots = next_plots;
        println!("{}", plots.len());

        if plots.is_empty() {
            j = i;
            break;
        }
    }
    //println!("{:?}", done_plots);
    let min_x = done_plots.iter().min_by(|&a, &b| a.x.cmp(&b.x)).unwrap().x - 10;
    let max_x = done_plots.iter().max_by(|&a, &b| a.x.cmp(&b.x)).unwrap().x + 10;
    let min_y = done_plots.iter().min_by(|&a, &b| a.y.cmp(&b.y)).unwrap().y - 4;
    let max_y = done_plots.iter().max_by(|&a, &b| a.y.cmp(&b.y)).unwrap().y + 4;
    //println!("{:?} / {:?}", min_pos, max_pos);
    for y in min_y..=max_y {
        println!(
            "{}",
            String::from_iter((min_x..=max_x).map(|x| {
                let p = Position { x, y };
                if grid[p.as_grid_pos_in_repeating_grid(&grid)].tile_type == TileType::Rock {
                    '#'
                } else if done_plots.contains(&p) {
                    '+'
                } else {
                    '.'
                }
            }))
        );
    }

    return;

    /*for i in 1.. {
        let mut next_plots = BTreeSet::<Position>::new();
        for pos in &plots {
            for dir in Direction::all() {
                if let Some(next_pos) = pos.advance_in_grid(dir, &grid) {
                    if grid[next_pos.as_grid_pos()].tile_type != TileType::Rock
                        && !done_plots.contains(&next_pos)
                    {
                        next_plots.insert(next_pos);
                        done_plots.insert(next_pos);
                        if i % 2 == 0 {
                            plot_count_even += 1;
                        } else {
                            plot_count_odd += 1;
                        }
                    }
                }
            }
        }
        plots = next_plots;
        println!("{}", plots.len());

        if plots.is_empty() {
            j = i;
            break;
        }
    }*/

    println!("{}", j);

    let reached = plots.len();
    println!("odd {} / even {}", plot_count_odd, plot_count_even)
}
