use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use grid::Grid;
use itertools::Itertools;
use line_drawing::WalkGrid;

// 00:41:57
pub fn part1() {
    let file = File::open(Path::new("inp11_2.txt")).unwrap();
    let mut lines = io::BufReader::new(file).lines().peekable();
    // FIXME is this OK?
    let first_line = lines.peek().unwrap().as_ref().unwrap().clone();

    let grid_vec: Vec<_> = lines
        .flat_map(|l| l.unwrap().chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();

    let grid = Grid::from_vec(grid_vec, first_line.len());

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

    let mut galaxies = Vec::new();
    for (y, row) in grid.iter_rows().enumerate() {
        for (x, &col) in row.enumerate() {
            if col {
                galaxies.push((x as i64, y as i64));
            }
        }
    }
    println!("{:?}", galaxies);

    let empty_rows: Vec<usize> = (0..grid.rows())
        .filter(|&y| grid.iter_row(y).all(|&c| !c))
        .collect();
    let empty_cols: Vec<usize> = (0..grid.cols())
        .filter(|&x| grid.iter_col(x).all(|&c| !c))
        .collect();
    for galaxy in &mut galaxies {
        galaxy.0 += empty_cols
            .iter()
            .filter(|&&x| (x as i64) < galaxy.0)
            .count() as i64;
        galaxy.1 += empty_rows
            .iter()
            .filter(|&&y| (y as i64) < galaxy.1)
            .count() as i64;
    }
    println!("{:?}", galaxies);

    //let galaxies = vec![(1, 6), (5, 11)];

    let sum: usize = galaxies
        .iter()
        .permutations(2)
        .map(|galaxies| {
            let &a = galaxies[0];
            let &b = galaxies[1];
            WalkGrid::new(a, b).count() - 1
        })
        .sum();

    println!("{}", sum / 2);
}

// 01:08:41
pub fn part2() {
    let file = File::open(Path::new("inp11_2.txt")).unwrap();
    let mut lines = io::BufReader::new(file).lines().peekable();
    // FIXME is this OK?
    let first_line = lines.peek().unwrap().as_ref().unwrap().clone();

    let grid_vec: Vec<_> = lines
        .flat_map(|l| l.unwrap().chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();

    let grid = Grid::from_vec(grid_vec, first_line.len());

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

    let mut galaxies = Vec::new();
    for (y, row) in grid.iter_rows().enumerate() {
        for (x, &col) in row.enumerate() {
            if col {
                galaxies.push((x as i64, y as i64));
            }
        }
    }
    println!("{:?}", galaxies);

    let empty_rows: Vec<usize> = (0..grid.rows())
        .filter(|&y| grid.iter_row(y).all(|&c| !c))
        .collect();
    let empty_cols: Vec<usize> = (0..grid.cols())
        .filter(|&x| grid.iter_col(x).all(|&c| !c))
        .collect();
    for galaxy in &mut galaxies {
        const EXPANSION: i64 = 999_999;
        galaxy.0 += EXPANSION
            * empty_cols
                .iter()
                .filter(|&&x| (x as i64) < galaxy.0)
                .count() as i64;
        galaxy.1 += EXPANSION
            * empty_rows
                .iter()
                .filter(|&&y| (y as i64) < galaxy.1)
                .count() as i64;
    }
    println!("{:?}", galaxies);

    let sum: i64 = galaxies
        .iter()
        .combinations(2)
        .map(|galaxies| {
            let &a = galaxies[0];
            let &b = galaxies[1];

            (b.0 - a.0).abs() + (b.1 - a.1).abs()
        })
        .sum();

    println!("{}", sum);
}
