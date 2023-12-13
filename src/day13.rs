use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use grid::Grid;
use itertools::Itertools;

pub fn part1() {
    let file = File::open(Path::new("inp13_2.txt")).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut grid_vec: Vec<bool> = Vec::new();
    let mut grid_w = 0;
    let mut sum = 0;

    for l in lines {
        let line = l.unwrap();
        if line.is_empty() {
            let mut grid = Grid::from_vec(grid_vec, grid_w);

            println!(
                "{}",
                grid.iter_rows()
                    .map(|row| String::from_iter(row.map(|c| match c {
                        false => '.',
                        true => '#',
                    })))
                    .collect::<Vec<String>>()
                    .join("\n")
            );

            fn find_reflection(grid: &Grid<bool>) -> Option<usize> {
                let reflection_line = (0..grid.rows() - 1).find(|&mirror_at| {
                    (0..=mirror_at).all(|check_y| {
                        //println!("== mirror_at {} check_y {}", mirror_at, check_y);
                        let y_a = mirror_at + check_y + 1;
                        if y_a >= grid.rows() {
                            //println!("exita");
                            return true;
                        }
                        let y_b = match mirror_at.checked_sub(check_y) {
                            None => return true,
                            Some(y) => y,
                        };
                        //println!(" -> y_a {} y_b {}", y_a, y_b);
                        let row_a = grid.iter_row(y_a);
                        let row_b = grid.iter_row(y_b);
                        row_a.eq(row_b)
                    })
                });
                println!("- {:?}", reflection_line);
                reflection_line.and_then(|l| Some(l + 1))
            }

            sum += match find_reflection(&grid) {
                Some(l) => 100 * l,
                None => {
                    println!("transpose");
                    grid.transpose();
                    find_reflection(&grid).unwrap()
                }
            };

            grid_vec = Vec::new();
            continue;
        }

        grid_w = line.len();
        grid_vec.extend(line.chars().map(|c| c == '#'));
    }

    println!("{}", sum);
}

pub fn part2() {
    let file = File::open(Path::new("inp13_2.txt")).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut grid_vec: Vec<bool> = Vec::new();
    let mut grid_w = 0;
    let mut sum = 0;

    for l in lines {
        let line = l.unwrap();
        if line.is_empty() {
            let mut grid = Grid::from_vec(grid_vec, grid_w);

            println!(
                "{}",
                grid.iter_rows()
                    .map(|row| String::from_iter(row.map(|c| match c {
                        false => '.',
                        true => '#',
                    })))
                    .collect::<Vec<String>>()
                    .join("\n")
            );

            fn find_reflections(grid: &Grid<bool>) -> Vec<usize> {
                (0..grid.rows() - 1)
                    .filter(|&mirror_at| {
                        (0..=mirror_at).all(|check_y| {
                            //println!("== mirror_at {} check_y {}", mirror_at, check_y);
                            let y_a = mirror_at + check_y + 1;
                            if y_a >= grid.rows() {
                                //println!("exita");
                                return true;
                            }
                            let y_b = match mirror_at.checked_sub(check_y) {
                                None => return true,
                                Some(y) => y,
                            };
                            //println!(" -> y_a {} y_b {}", y_a, y_b);
                            let row_a = grid.iter_row(y_a);
                            let row_b = grid.iter_row(y_b);
                            row_a.eq(row_b)
                        })
                    })
                    .map(|l| l + 1)
                    .collect_vec()
                //println!("- {:?}", reflection_line);
            }

            let y_reflections = find_reflections(&grid);
            let mut orig_reflection_line: Option<usize> = None;
            if y_reflections.len() == 1 {
                orig_reflection_line = Some(y_reflections[0] * 100);
            } else if y_reflections.len() == 0 {
                println!("transpose");
                let mut transp_grid = grid.clone();
                transp_grid.transpose();
                let x_reflections = find_reflections(&transp_grid);
                if x_reflections.len() == 1 {
                    orig_reflection_line = Some(x_reflections[0]);
                }
            }
            /*
            grid.iter_mut().find(|elem| {
                // this not work
                *elem = !**elem;
                true
            });
            */
            let orig_reflection_line = orig_reflection_line.unwrap();
            println!("old: {}", orig_reflection_line);

            let mut new_reflection_line: Option<usize> = None;
            'outer: for y in 0..grid.rows() {
                for x in 0..grid.cols() {
                    let mut new_grid = grid.clone();
                    new_grid[(y, x)] = !new_grid[(y, x)];
                    if y == 6 && x == 16 {
                        println!("////////////////");
                        println!(
                            "{}",
                            new_grid
                                .iter_rows()
                                .map(|row| String::from_iter(row.map(|c| match c {
                                    false => '.',
                                    true => '#',
                                })))
                                .collect::<Vec<String>>()
                                .join("\n")
                        );
                        println!("-----------------");
                    }

                    let y_reflections = find_reflections(&new_grid);
                    if let Some(new_y_reflection) = y_reflections
                        .iter()
                        .map(|l| l * 100)
                        .find(|&r| r != orig_reflection_line)
                    {
                        new_reflection_line = Some(new_y_reflection);
                        break 'outer;
                    }

                    new_grid.transpose();

                    let x_reflections = find_reflections(&new_grid);
                    if let Some(&new_x_reflection) =
                        x_reflections.iter().find(|&&r| r != orig_reflection_line)
                    {
                        new_reflection_line = Some(new_x_reflection);
                        break 'outer;
                    }

                    if y == 6 && x == 16 {
                        println!("////////////////");
                        println!(
                            "{}",
                            new_grid
                                .iter_rows()
                                .map(|row| String::from_iter(row.map(|c| match c {
                                    false => '.',
                                    true => '#',
                                })))
                                .collect::<Vec<String>>()
                                .join("\n")
                        );
                        println!("-----------------");
                    }
                }
            }
            println!("new: {:?}", new_reflection_line);
            sum += new_reflection_line.unwrap();

            grid_vec = Vec::new();
            continue;
        }

        grid_w = line.len();
        grid_vec.extend(line.chars().map(|c| c == '#'));
    }

    println!("{}", sum);
}
