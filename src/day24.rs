use crate::util::{file_lines, parse_ints};
use itertools::{izip, Itertools};
use std::ops::RangeInclusive;
use z3::ast::Ast;

#[derive(Debug, PartialEq)]
struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

impl From<Vec<f64>> for Vec3D {
    fn from(v: Vec<f64>) -> Self {
        assert_eq!(v.len(), 3);
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

impl Vec3D {
    fn to_slice(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

#[derive(Debug, PartialEq)]
struct HailStone {
    pos: Vec3D,
    velocity: Vec3D,
}

fn parse_stones(lines: &[String]) -> Vec<HailStone> {
    lines
        .iter()
        .map(|l| {
            let (p, v) = l.split_once('@').unwrap();
            HailStone {
                pos: parse_ints(p).into(),
                velocity: parse_ints(v).into(),
            }
        })
        .collect_vec()
}

pub fn part1() {
    let lines = file_lines("inp24_2.txt");
    let hailstones = parse_stones(&lines);

    const MIN_COORD: f64 = 200000000000000.0;
    const MAX_COORD: f64 = 400000000000000.0;

    let intersections = hailstones
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
    let hailstones = parse_stones(&lines);

    // First try: Solve with sage -> see day24_p2.ipynb#
    // (3 hailstones are actually enough to solve the equations)
    (1..=3).for_each(|i| {
        let s = &hailstones[i];
        println!("x{} = {}", i, s.pos.x);
        println!("y{} = {}", i, s.pos.y);
        println!("z{} = {}", i, s.pos.z);
        println!("vx{} = {}", i, s.velocity.x);
        println!("vy{} = {}", i, s.velocity.y);
        println!("vz{} = {}", i, s.velocity.z);
    });

    // Now with Rust :-)
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let int_var_range = |name: &str, range: RangeInclusive<usize>| {
        range
            .map(|i| z3::ast::Int::new_const(&ctx, format!("{}{}", name, i)))
            .collect_vec()
    };
    let int_const_vec = |c: &[f64; 3]| {
        c.iter()
            .map(|&val| z3::ast::Int::from_i64(&ctx, val as i64))
            .collect_vec()
    };

    let rock_pos = int_var_range("r", 0..=2);
    let rock_vel = int_var_range("rv", 0..=2);
    let time = int_var_range("t", 1..=hailstones.len());

    for (h, t) in hailstones.iter().zip(&time) {
        let hail_pos = int_const_vec(&h.pos.to_slice());
        let hail_vel = int_const_vec(&h.velocity.to_slice());

        // Build equations for each of the dimensions
        for (hp, hv, rp, rv) in izip!(&hail_pos, &hail_vel, &rock_pos, &rock_vel) {
            let hail_trajectory = hp + t * hv;
            let rock_trajectory = rp + t * rv;
            solver.assert(&(rock_trajectory._eq(&hail_trajectory)));
        }
    }

    // Solve
    assert_eq!(solver.check(), z3::SatResult::Sat);
    let model = solver.get_model().unwrap();
    // Get position vector and sum it up
    let pos_sum: i64 = rock_pos
        .iter()
        .map(|p| model.eval(p, true).unwrap().as_i64().unwrap())
        .sum();
    println!("{}", pos_sum);
}
