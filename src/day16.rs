use std::{cmp::max, collections::VecDeque};

use grid::Grid;

use super::util;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
    fn repr(&self) -> char {
        match self {
            Direction::N => '╵',
            Direction::E => '╶',
            Direction::S => '╷',
            Direction::W => '╴',
        }
    }
    fn is_horizontal(self) -> bool {
        self == Direction::E || self == Direction::W
    }
    fn is_vertical(self) -> bool {
        self == Direction::N || self == Direction::S
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn as_grid_pos(self) -> (usize, usize) {
        (self.y, self.x)
    }

    fn advance_in_grid<T>(self, dir: Direction, grid: &Grid<T>) -> Option<Position> {
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
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum TileType {
    Empty = b'.',
    SplitV = b'|',
    SplitH = b'-',
    MirrorCw = b'/',
    MirrorCcw = b'\\',
}
impl From<char> for TileType {
    fn from(c: char) -> Self {
        // FIXME
        match c {
            '.' => TileType::Empty,
            '|' => TileType::SplitV,
            '-' => TileType::SplitH,
            '/' => TileType::MirrorCw,
            '\\' => TileType::MirrorCcw,
            _ => panic!(),
        }
    }
}
impl Into<char> for TileType {
    fn into(self) -> char {
        self as u8 as char
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    tile_type: TileType,
    energized: bool,
    energized_dir: [bool; 4],
}

pub fn part1() {
    let lines = util::file_lines("inp16_2.txt");

    let first_line = &lines[0];
    let grid_vec: Vec<_> = lines
        .iter()
        .flat_map(|l| {
            l.chars()
                .map(|c| Tile {
                    tile_type: c.into(),
                    energized: false,
                    energized_dir: [false, false, false, false],
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let mut grid = Grid::from_vec(grid_vec, first_line.len());

    fn print_grid(grid: &Grid<Tile>) {
        println!(
            "{}",
            grid.iter_rows()
                .map(|row| String::from_iter(row.map(|t| Into::<char>::into(t.tile_type))))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
    print_grid(&grid);

    struct QueueEntry {
        pos: Position,
        dir: Direction,
    }
    let mut q = VecDeque::from([QueueEntry {
        pos: Position { x: 0, y: 0 },
        dir: Direction::E,
    }]);

    let mut i = 0;

    while let Some(entry) = q.pop_front() {
        i += 1;
        if i > 100000000 {
            break;
        }
        let pos = entry.pos;
        let dir = entry.dir;
        grid[entry.pos.as_grid_pos()].energized = true;
        let grid_tile = &grid[entry.pos.as_grid_pos()];

        let mut try_push = |dir| {
            if let Some(next_pos) = pos.advance_in_grid(dir, &grid) {
                q.push_back(QueueEntry {
                    pos: next_pos,
                    dir: dir,
                });
            }
        };

        match grid_tile.tile_type {
            TileType::Empty => {
                try_push(dir);
            }
            TileType::SplitH => {
                if dir.is_horizontal() {
                    try_push(dir);
                } else {
                    try_push(Direction::E);
                    try_push(Direction::W);
                }
            }
            TileType::SplitV => {
                if dir.is_vertical() {
                    try_push(dir);
                } else {
                    try_push(Direction::N);
                    try_push(Direction::S);
                }
            }
            TileType::MirrorCw => {
                let next_dir = match dir {
                    Direction::N => Direction::E,
                    Direction::W => Direction::S,
                    Direction::E => Direction::N,
                    Direction::S => Direction::W,
                };
                try_push(next_dir);
            }
            TileType::MirrorCcw => {
                let next_dir = match dir {
                    Direction::N => Direction::W,
                    Direction::E => Direction::S,
                    Direction::W => Direction::N,
                    Direction::S => Direction::E,
                };
                try_push(next_dir);
            }
        }
    }

    println!(
        "{}",
        grid.iter_rows()
            .map(|row| String::from_iter(row.map(|t| match t.energized {
                false => '.',
                true => '#',
            })))
            .collect::<Vec<String>>()
            .join("\n")
    );

    let energized = grid.iter().filter(|t| t.energized).count();
    println!("{}", energized);
}

pub fn part2() {
    let lines = util::file_lines("inp16_2.txt");

    let first_line = &lines[0];
    let grid_vec: Vec<_> = lines
        .iter()
        .flat_map(|l| {
            l.chars()
                .map(|c| Tile {
                    tile_type: c.into(),
                    energized: false,
                    energized_dir: [false; 4],
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let mut grid = Grid::from_vec(grid_vec, first_line.len());

    fn print_grid(grid: &Grid<Tile>) {
        println!(
            "{}",
            grid.iter_rows()
                .map(|row| String::from_iter(row.map(|t| Into::<char>::into(t.tile_type))))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
    print_grid(&grid);

    struct QueueEntry {
        pos: Position,
        dir: Direction,
    }

    fn energize_from(pos: Position, dir: Direction, grid: &mut Grid<Tile>) -> usize {
        let mut q = VecDeque::from([QueueEntry { pos, dir }]);

        while let Some(entry) = q.pop_front() {
            let pos = entry.pos;
            let dir = entry.dir;
            grid[entry.pos.as_grid_pos()].energized_dir[dir as usize] = true;
            let grid_tile = &grid[entry.pos.as_grid_pos()];

            let mut try_push = |dir| {
                if let Some(next_pos) = pos.advance_in_grid(dir, &grid) {
                    if grid[next_pos.as_grid_pos()].energized_dir[dir as usize] {
                        // Already visited in this direction
                        return;
                    }
                    q.push_back(QueueEntry {
                        pos: next_pos,
                        dir: dir,
                    });
                }
            };

            match grid_tile.tile_type {
                TileType::Empty => {
                    try_push(dir);
                }
                TileType::SplitH => {
                    if dir.is_horizontal() {
                        try_push(dir);
                    } else {
                        try_push(Direction::E);
                        try_push(Direction::W);
                    }
                }
                TileType::SplitV => {
                    if dir.is_vertical() {
                        try_push(dir);
                    } else {
                        try_push(Direction::N);
                        try_push(Direction::S);
                    }
                }
                TileType::MirrorCw => {
                    let next_dir = match dir {
                        Direction::N => Direction::E,
                        Direction::W => Direction::S,
                        Direction::E => Direction::N,
                        Direction::S => Direction::W,
                    };
                    try_push(next_dir);
                }
                TileType::MirrorCcw => {
                    let next_dir = match dir {
                        Direction::N => Direction::W,
                        Direction::E => Direction::S,
                        Direction::W => Direction::N,
                        Direction::S => Direction::E,
                    };
                    try_push(next_dir);
                }
            }
        }

        let res = grid
            .iter()
            .filter(|t| t.energized_dir.iter().any(|&d| d))
            .count();

        // Reset grid
        for tile in grid.iter_mut() {
            tile.energized_dir = [false; 4];
        }

        res
    }

    let max_col_energy = (0..grid.cols())
        .map(|x| {
            max(
                energize_from(Position { x, y: 0 }, Direction::S, &mut grid),
                energize_from(
                    Position {
                        x,
                        y: grid.rows() - 1,
                    },
                    Direction::N,
                    &mut grid,
                ),
            )
        })
        .max()
        .unwrap();
    let max_row_energy = (0..grid.rows())
        .map(|y| {
            max(
                energize_from(Position { x: 0, y }, Direction::E, &mut grid),
                energize_from(
                    Position {
                        x: grid.cols() - 1,
                        y,
                    },
                    Direction::W,
                    &mut grid,
                ),
            )
        })
        .max()
        .unwrap();
    let max_energy = max(max_col_energy, max_row_energy);

    println!("{}", max_energy);
}
