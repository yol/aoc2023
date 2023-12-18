use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use std::fmt;
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

pub fn print_grid<T, F>(grid: &Grid<T>, map_fn: F)
where
    F: Fn(&T) -> char,
{
    println!(
        "{}",
        grid.iter_rows()
            .map(|row| String::from_iter(row.map(&map_fn)))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, enum_iterator::Sequence)]
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

    pub fn opposite(self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
    pub fn rot_cw(self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
    pub fn rot_ccw(self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        }
    }
    pub fn repr(&self) -> char {
        match self {
            Direction::N => '↑',
            Direction::E => '→',
            Direction::S => '↓',
            Direction::W => '←',
        }
    }
    pub fn is_horizontal(self) -> bool {
        self == Direction::E || self == Direction::W
    }
    pub fn is_vertical(self) -> bool {
        self == Direction::N || self == Direction::S
    }
    pub fn is_perpendicular_to(self, other: Direction) -> bool {
        (self.is_horizontal() && !other.is_horizontal())
            || (self.is_vertical() && !other.is_vertical())
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repr())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}
impl Position {
    pub fn as_grid_pos(self) -> (usize, usize) {
        (self.y as usize, self.x as usize)
    }

    pub fn advance_in_dir_by(self, dir: Direction, extent: isize) -> Position {
        match dir {
            Direction::N => Position {
                x: self.x,
                y: self.y - extent,
            },
            Direction::E => Position {
                x: self.x + extent,
                y: self.y,
            },
            Direction::S => Position {
                x: self.x,
                y: self.y + extent,
            },
            Direction::W => Position {
                x: self.x - extent,
                y: self.y,
            },
        }
    }

    pub fn advance_in_grid<T>(self, dir: Direction, grid: &Grid<T>) -> Option<Position> {
        match dir {
            Direction::N if self.y >= 1 => Some(Position {
                x: self.x,
                y: self.y - 1,
            }),
            Direction::E if (self.x as usize) < grid.cols() - 1 => Some(Position {
                x: self.x + 1,
                y: self.y,
            }),
            Direction::S if (self.y as usize) < grid.rows() - 1 => Some(Position {
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
        (other.x - self.x).abs() as usize + (other.y - self.y).abs() as usize
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
