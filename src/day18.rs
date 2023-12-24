use std::{
    cmp::{max, min},
    collections::VecDeque,
};

use super::util::{file_lines, print_grid, Direction, Position};
use grid::Grid;
use itertools::Itertools;
use std::fmt;

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
        x: (SIZE / 2) as isize,
        y: (SIZE / 2) as isize,
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
    let lines = file_lines("inp18_2.txt");

    #[derive(Eq, PartialEq, Clone, Copy)]
    struct LineSeg {
        a: Position,
        b: Position,
        dir: Direction,
    }
    impl fmt::Debug for LineSeg {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{:?}{:?}{:?}]", self.a, self.dir, self.b)
        }
    }

    impl LineSeg {
        fn project_point(&self, p: Position) -> Option<Position> {
            if self.dir.is_horizontal() {
                assert_eq!(self.a.y, self.b.y);
                if p.x >= min(self.a.x, self.b.x) && p.x <= max(self.a.x, self.b.x) {
                    Some(Position {
                        x: p.x,
                        y: self.a.y,
                    })
                } else {
                    None
                }
            } else {
                assert_eq!(self.a.x, self.b.x);
                if p.y >= min(self.a.y, self.b.y) && p.y <= max(self.a.y, self.b.y) {
                    Some(Position {
                        x: self.a.x,
                        y: p.y,
                    })
                } else {
                    None
                }
            }
        }

        /// Length in grid units including start and end point
        fn length(&self) -> usize {
            self.a.manhattan_distance_to(self.b) + 1
        }

        fn new_from_pos(a: Position, b: Position) -> LineSeg {
            LineSeg {
                a,
                b,
                dir: Self::determine_dir(a, b),
            }
        }

        fn determine_dir(a: Position, b: Position) -> Direction {
            if a.x == b.x {
                if b.y > a.y {
                    Direction::S
                } else {
                    Direction::N
                }
            } else if a.y == b.y {
                if b.x > a.x {
                    Direction::E
                } else {
                    Direction::W
                }
            } else {
                panic!()
            }
        }

        fn reverse(&self) -> LineSeg {
            LineSeg {
                a: self.b,
                b: self.a,
                dir: self.dir.opposite(),
            }
        }
    }

    type Polygon = Vec<LineSeg>;

    let mut poly = Polygon::new();
    let start_pos = Position { x: 0, y: 0 };

    // Parse polygon
    {
        let mut pos = start_pos;

        for instruction in lines {
            let instr_parts = instruction.split_whitespace().collect_vec();
            let dir = match instr_parts[2].chars().nth(7).unwrap() {
                '0' => Direction::E,
                '1' => Direction::S,
                '2' => Direction::W,
                '3' => Direction::N,
                _ => panic!(),
            };
            let dist = isize::from_str_radix(&instr_parts[2][2..7], 16).unwrap();

            let next_pos = pos.advance_in_dir_by(dir, dist);
            poly.push(LineSeg {
                a: pos,
                b: next_pos,
                dir,
            });
            pos = next_pos;
        }
    }

    fn print_poly(poly: &Polygon) {
        const SIZE: usize = 20;
        const OFFSET: usize = 2;

        let mut grid: Grid<char> = Grid::init(SIZE, SIZE, '.');
        for seg in poly {
            let mut pos = seg.a;
            pos.x += OFFSET as isize;
            pos.y += OFFSET as isize;
            for _ in 0..seg.length() {
                grid[pos.as_grid_pos()] = seg.dir.repr();
                pos = pos.advance_in_grid(seg.dir, &grid).unwrap();
            }
        }

        print_grid(&grid, |&c| c);
    }
    //print_poly(&poly);

    let mut area_correct: i64 = 0;

    // Simplify polygon
    {
        type Dir = Direction;

        let mut swap_in_out = false;

        while poly.len() > 4 {
            //println!("start!");
            println!("{:?}", poly);
            fn simplify(poly: &mut Polygon, swap_in_out: bool) -> Option<i64> {
                for (index, (&l1, &l2, &l3, &l4)) in
                    poly.iter().circular_tuple_windows().enumerate()
                {
                    if l1.dir == l3.dir.opposite() && l2.dir.is_perpendicular_to(l1.dir) {
                        let start_pos = l1.a;
                        let new_end = l3.project_point(start_pos);
                        if let Some(new_end) = new_end {
                            // Successfully projected the start position onto the end line

                            // We can simplify this segment
                            // Replace 3 lines by 1 or 2 lines

                            println!(
                                "Simplify in {}: {:?}/{:?}/{:?}  (next: {:?})",
                                index, l1, l2, l3, l4
                            );

                            let new_seg = LineSeg::new_from_pos(l1.a, new_end);

                            // Check rotation direction to see if the simplification is cutting away from
                            // inside or outside the polygon
                            let mut cutting_out_inside = l2.dir == l1.dir.rot_cw();
                            if swap_in_out {
                                cutting_out_inside = !cutting_out_inside;
                            }

                            let area_before = if cutting_out_inside {
                                // Actual area
                                l1.length() * l2.length()
                            } else {
                                // Length of the perimeter
                                l1.length() + l2.length() + l1.length() - 2 /* edge points */
                            };
                            let area_after = if cutting_out_inside {
                                // Length of the perimeter
                                l2.length()
                            } else {
                                // Actual area
                                l1.length() * l2.length()
                            };
                            let area_diff = (area_before as i64) - (area_after as i64);
                            println!(
                                "<area> inside?{} before {} after {} -> diff {}",
                                cutting_out_inside, area_before, area_after, area_diff
                            );

                            // Replace this line
                            {
                                let l1mut = &mut poly[index];
                                *l1mut = new_seg;
                                //println!("-> {:?}", l1mut);
                            }

                            if new_seg.b == l4.a {
                                // Only one line needed
                                println!("remove 2nd segment");
                                poly.remove((index + 1) % poly.len());
                                poly.remove((index + 1) % poly.len());
                            } else {
                                // Need line segment connecting new_seg and following line
                                let plen = poly.len();
                                let l3mut = &mut poly[(index + 1) % plen];
                                *l3mut = LineSeg::new_from_pos(new_seg.b, l4.a);
                                println!("-----> replace seg after: {:?}", l3mut);
                                poly.remove((index + 2) % poly.len());
                            }

                            //println!("{:?}", poly);

                            return Some(area_diff);
                        }
                    }
                }
                None
            }
            match simplify(&mut poly, swap_in_out) {
                Some(d) => area_correct += d,
                None => {
                    //print_poly(&poly);
                    //panic!();
                    poly.reverse();
                    swap_in_out = !swap_in_out;
                    for seg in &mut poly {
                        *seg = seg.reverse();
                    }
                    match simplify(&mut poly, swap_in_out) {
                        Some(d) => area_correct += d,
                        None => panic!(),
                    }
                }
            }
            //print_poly(&poly);

            let mut merged = true;
            while merged {
                merged = false;
                for (index, (l0, l1)) in poly.iter().circular_tuple_windows().enumerate() {
                    assert_eq!(l0.b, l1.a);
                    if l0.dir == l1.dir {
                        println!("merge {}! {:?},{:?}", index, l0, l1);
                        merged = true;
                        // Merge lines
                        poly[index].b = l1.b;
                        poly.remove((index + 1) % poly.len());
                        break;
                    }
                }
            }

            //println!("---");
        }
    }

    println!("{:?}", poly);
    let area = (poly[0].length() * poly[1].length()) as i64;
    let total_area = area + area_correct;
    println!("{}", total_area);
}
