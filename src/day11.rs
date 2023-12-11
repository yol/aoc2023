use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use grid::Grid;
use itertools::Itertools;
use line_drawing::WalkGrid;

pub fn part1() {
    let file = File::open(Path::new("inp11_2.txt")).unwrap();
    let mut lines = io::BufReader::new(file).lines().peekable();
    // FIXME is this OK?
    let first_line = lines.peek().unwrap().as_ref().unwrap().clone();

    let grid_vec: Vec<_> = lines
        .flat_map(|l| l.unwrap().chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();

    let mut grid = Grid::from_vec(grid_vec, first_line.len());

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

    fn insert_space(grid: &mut Grid<bool>) {
        let mut y = 0;
        while y < grid.rows() {
            let mut row = grid.iter_row(y);
            if row.all(|&c| c == false) {
                grid.insert_row(y, vec![false; grid.cols()]);
                y += 1;
            }
            y += 1;
        }
    };
    insert_space(&mut grid);
    grid.transpose();
    insert_space(&mut grid);
    grid.transpose();

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
            if col == true {
                galaxies.push((x as isize, y as isize));
            }
        }
    }
    //let galaxies = vec![(1, 6), (5, 11)];

    println!("{:?}", galaxies);

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
