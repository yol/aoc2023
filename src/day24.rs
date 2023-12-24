use crate::util::{file_lines, parse_ints};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

impl From<Vec<f64>> for Vec3D {
    fn from(v: Vec<f64>) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

#[derive(Debug, PartialEq)]
struct Stone {
    pos: Vec3D,
    velocity: Vec3D,
}

fn parse_stones(lines: &[String]) -> Vec<Stone> {
    lines
        .iter()
        .map(|l| {
            let (p, v) = l.split_once('@').unwrap();
            Stone {
                pos: parse_ints(p).into(),
                velocity: parse_ints(v).into(),
            }
        })
        .collect_vec()
}

pub fn part1() {
    let lines = file_lines("inp24_2.txt");
    let stones = parse_stones(&lines);

    const MIN_COORD: f64 = 200000000000000.0;
    const MAX_COORD: f64 = 400000000000000.0;

    let intersections = stones
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            let (x1, y1) = (a.pos.x, a.pos.y);
            let (x2, y2) = (b.pos.x, b.pos.y);
            let (v1x, v1y) = (a.velocity.x, a.velocity.y);
            let (v2x, v2y) = (b.velocity.x, b.velocity.y);
            let m2 = ((y1 - y2) + (v1y / v1x) * (x2 - x1)) / (v2y - v1y * (v2x / v1x));
            let m1 = (x2 + m2 * v2x - x1) / v1x;

            let px = x2 + m2 * v2x;
            let py = y2 + m2 * v2y;

            println!(
                "{:?} intersect {:?} m1 {}/m2 {} at {},{}",
                a, b, m1, m2, px, py
            );

            m1 >= 0.0
                && m2 >= 0.0
                && (MIN_COORD..=MAX_COORD).contains(&px)
                && (MIN_COORD..=MAX_COORD).contains(&py)
        })
        .count();

    println!("{}", intersections);
}

pub fn part2() {
    let lines = file_lines("inp24_2.txt");
    let stones = parse_stones(&lines);

    // First try: Solve with sage -> see day24_p2.ipynb
    (1..=3).for_each(|i| {
        let s = &stones[i];
        println!("x{} = {}", i, s.pos.x);
        println!("y{} = {}", i, s.pos.y);
        println!("z{} = {}", i, s.pos.z);
        println!("vx{} = {}", i, s.velocity.x);
        println!("vy{} = {}", i, s.velocity.y);
        println!("vz{} = {}", i, s.velocity.z);
    });
}
