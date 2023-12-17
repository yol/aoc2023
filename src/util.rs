use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use std::fmt::Debug;
use std::str::FromStr;

use grid::Grid;
use itertools::Itertools;

pub fn parse_ints<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.split(|c| c == ',' || c == ' ')
        .map(|p| p.parse::<T>().unwrap())
        .collect_vec()
}

pub fn file_lines(filename: &str) -> Vec<String> {
    let file = File::open(Path::new(filename)).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect_vec()
}

pub fn parse_digit(c: char) -> u8 {
    c.to_digit(10).unwrap() as u8
}

pub fn build_grid<T, F>(lines: &Vec<String>, map_fn: F) -> Grid<T>
where
    F: Fn(char) -> T,
{
    let first_line = &lines[0];
    let grid_vec = lines
        .iter()
        .flat_map(|l| l.chars().map(&map_fn).collect_vec())
        .collect_vec();
    Grid::from_vec(grid_vec, first_line.len())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, enum_iterator::Sequence)]
pub enum Direction {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

impl Direction {
    pub fn all() -> enum_iterator::All<Direction> {
        enum_iterator::all::<Self>()
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
    pub fn repr(&self) -> char {
        match self {
            Direction::N => '╵',
            Direction::E => '╶',
            Direction::S => '╷',
            Direction::W => '╴',
        }
    }
    pub fn is_horizontal(self) -> bool {
        self == Direction::E || self == Direction::W
    }
    pub fn is_vertical(self) -> bool {
        self == Direction::N || self == Direction::S
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn as_grid_pos(self) -> (usize, usize) {
        (self.y, self.x)
    }

    pub fn advance_in_grid<T>(self, dir: Direction, grid: &Grid<T>) -> Option<Position> {
        match dir {
            Direction::N if self.y >= 1 => Some(Position {
                x: self.x,
                y: self.y - 1,
            }),
            Direction::E if self.x < grid.cols() - 1 => Some(Position {
                x: self.x + 1,
                y: self.y,
            }),
            Direction::S if self.y < grid.rows() - 1 => Some(Position {
                x: self.x,
                y: self.y + 1,
            }),
            Direction::W if self.x >= 1 => Some(Position {
                x: self.x - 1,
                y: self.y,
            }),
            _ => None,
        }
    }

    pub fn manhattan_distance_to(self, other: Position) -> usize {
        (other.x as isize - self.x as isize).abs() as usize
            + (other.y as isize - self.y as isize).abs() as usize
    }
}
