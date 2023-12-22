use std::{
    cmp::{max, min},
    collections::BTreeSet,
};

use crate::util::parse_ints;

use super::util::file_lines;
use itertools::Itertools;

pub fn part1() {
    let lines = file_lines("inp22_2.txt");

    type Pos3D = (i64, i64, i64);
    type Brick = (Pos3D, Pos3D);

    // z: growing to sky
    // ground: z=0; bricks: z >= 1

    let mut bricks = lines
        .iter()
        .map(|l| -> Brick {
            let (b1, b2) = l.split_once('~').unwrap();
            let i1 = parse_ints(b1);
            let i2 = parse_ints(b2);
            // Brick grows from .0 to .1
            (
                (min(i1[0], i2[0]), min(i1[1], i2[1]), min(i1[2], i2[2])),
                (max(i1[0], i2[0]), max(i1[1], i2[1]), max(i1[2], i2[2])),
            )
        })
        .collect_vec();
    println!("{:?}", bricks);

    let mut fallen = true;
    while fallen {
        fallen = false;
        for brick_i in 0..bricks.len() {
            let brick = &bricks[brick_i];
            let face_area_a = (brick.0 .0, brick.0 .1);
            let face_area_b = (brick.1 .0, brick.1 .1);
            let mut can_fall_to = 1;
            for x in face_area_a.0..=face_area_b.0 {
                for y in face_area_a.1..=face_area_b.1 {
                    // Check max dist
                    for other_brick in &bricks {
                        // Point in brick and below?
                        if other_brick.1.2 < brick.0.2 // brick below? NOTE: same brick is never below itself
                        && (other_brick.0.0..=other_brick.1.0).contains(&x) && (other_brick.0.1..=other_brick.1.1).contains(&y)
                        // area overlap?
                        {
                            can_fall_to = max(can_fall_to, other_brick.1 .2 + 1);
                        }
                    }
                }
            }
            if can_fall_to != brick.0 .2 {
                fallen = true;
                let dz = brick.1 .2 - brick.0 .2;
                let mut_brick = &mut bricks[brick_i];
                mut_brick.0 .2 = can_fall_to;
                mut_brick.1 .2 = can_fall_to + dz;
            }
        }
    }

    let mut can_disintegrate = 0;
    for brick_i in 0..bricks.len() {
        let mut new_bricks = bricks.clone();
        new_bricks.remove(brick_i);

        let mut fallen = false;
        for new_brick_i in 0..new_bricks.len() {
            let brick = &new_bricks[new_brick_i];
            let face_area_a = (brick.0 .0, brick.0 .1);
            let face_area_b = (brick.1 .0, brick.1 .1);
            let mut can_fall_to = 1;
            for x in face_area_a.0..=face_area_b.0 {
                for y in face_area_a.1..=face_area_b.1 {
                    // Check max dist
                    for other_brick in &new_bricks {
                        // Point in brick and below?
                        if other_brick.1.2 < brick.0.2 // brick below? NOTE: same brick is never below itself
                        && (other_brick.0.0..=other_brick.1.0).contains(&x) && (other_brick.0.1..=other_brick.1.1).contains(&y)
                        // area overlap?
                        {
                            can_fall_to = max(can_fall_to, other_brick.1 .2 + 1);
                        }
                    }
                }
            }
            if can_fall_to != brick.0 .2 {
                fallen = true;
                break;
            }
        }
        if !fallen {
            println!("can disintegrate: {}", brick_i);
            can_disintegrate += 1;
        }
    }

    println!("{:?}", bricks);
    println!("{}", can_disintegrate);
}

pub fn part2() {
    let lines = file_lines("inp22_2.txt");

    type Pos3D = (i64, i64, i64);
    type Brick = (Pos3D, Pos3D);

    // z: growing to sky
    // ground: z=0; bricks: z >= 1

    let mut bricks = lines
        .iter()
        .map(|l| -> Brick {
            let (b1, b2) = l.split_once('~').unwrap();
            let i1 = parse_ints(b1);
            let i2 = parse_ints(b2);
            // Brick grows from .0 to .1
            (
                (min(i1[0], i2[0]), min(i1[1], i2[1]), min(i1[2], i2[2])),
                (max(i1[0], i2[0]), max(i1[1], i2[1]), max(i1[2], i2[2])),
            )
        })
        .collect_vec();
    println!("{:?}", bricks);

    let mut fallen = true;
    while fallen {
        fallen = false;
        for brick_i in 0..bricks.len() {
            let brick = &bricks[brick_i];
            let face_area_a = (brick.0 .0, brick.0 .1);
            let face_area_b = (brick.1 .0, brick.1 .1);
            let mut can_fall_to = 1;
            for x in face_area_a.0..=face_area_b.0 {
                for y in face_area_a.1..=face_area_b.1 {
                    // Check max dist
                    for other_brick in &bricks {
                        // Point in brick and below?
                        if other_brick.1.2 < brick.0.2 // brick below? NOTE: same brick is never below itself
                        && (other_brick.0.0..=other_brick.1.0).contains(&x) && (other_brick.0.1..=other_brick.1.1).contains(&y)
                        // area overlap?
                        {
                            can_fall_to = max(can_fall_to, other_brick.1 .2 + 1);
                        }
                    }
                }
            }
            if can_fall_to != brick.0 .2 {
                fallen = true;
                let dz = brick.1 .2 - brick.0 .2;
                let mut_brick = &mut bricks[brick_i];
                mut_brick.0 .2 = can_fall_to;
                mut_brick.1 .2 = can_fall_to + dz;
            }
        }
    }

    let mut total_fall_count = 0;
    for brick_i in 0..bricks.len() {
        let mut new_bricks = bricks.clone();
        new_bricks.remove(brick_i);

        let mut fallen = true;
        let mut fall_set = BTreeSet::new();
        while fallen {
            fallen = false;
            for new_brick_i in 0..new_bricks.len() {
                let brick = &new_bricks[new_brick_i];
                let face_area_a = (brick.0 .0, brick.0 .1);
                let face_area_b = (brick.1 .0, brick.1 .1);
                let mut can_fall_to = 1;
                for x in face_area_a.0..=face_area_b.0 {
                    for y in face_area_a.1..=face_area_b.1 {
                        // Check max dist
                        for other_brick in &new_bricks {
                            // Point in brick and below?
                            if other_brick.1.2 < brick.0.2 // brick below? NOTE: same brick is never below itself
                            && (other_brick.0.0..=other_brick.1.0).contains(&x) && (other_brick.0.1..=other_brick.1.1).contains(&y)
                            // area overlap?
                            {
                                can_fall_to = max(can_fall_to, other_brick.1 .2 + 1);
                            }
                        }
                    }
                }
                if can_fall_to != brick.0 .2 {
                    fallen = true;
                    let dz = brick.1 .2 - brick.0 .2;
                    let mut_brick = &mut new_bricks[new_brick_i];
                    mut_brick.0 .2 = can_fall_to;
                    mut_brick.1 .2 = can_fall_to + dz;
                    fall_set.insert(new_brick_i);
                }
            }
        }

        if !fall_set.is_empty() {
            println!("disintegrate {} for win {}", brick_i, fall_set.len());
            total_fall_count += fall_set.len();
        }
    }

    println!("{:?}", bricks);
    println!("{}", total_fall_count);
}
