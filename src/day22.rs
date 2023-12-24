use crate::util::{file_lines, parse_ints};
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    cmp::{max, min},
    collections::BTreeSet,
};

type Pos3D = (i64, i64, i64);
type Brick = (Pos3D, Pos3D);

fn parse_bricks(lines: &[String]) -> Vec<Brick> {
    lines
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
        .collect_vec()
}

#[derive(PartialEq, Eq)]
enum StopAfterFirstFall {
    No,
    Yes,
}

fn perform_fall(
    bricks: &mut Vec<Brick>,
    stop_after_first_fall: StopAfterFirstFall,
) -> BTreeSet<usize> {
    let mut fallen = true;
    let mut fall_set = BTreeSet::new();

    // Continue let bricks fall until there was no change in a complete scan of all bricks
    while fallen {
        fallen = false;
        for brick_i in 0..bricks.len() {
            let brick = &bricks[brick_i];
            let face_area_a = (brick.0 .0, brick.0 .1);
            let face_area_b = (brick.1 .0, brick.1 .1);
            let mut can_fall_to = 1; // ground level

            // Performance optimization: check range overlap instead of every x/y point
            for x in face_area_a.0..=face_area_b.0 {
                for y in face_area_a.1..=face_area_b.1 {
                    // Check max dist
                    // Possible performance optimization: Pre-sort bricks by z coordinate
                    // (or use octrees...)
                    // FIXME &*? wat?
                    for other_brick in &*bricks {
                        // Point in brick and below?
                        if other_brick.1.2 < brick.0.2 // brick below? NOTE: same brick is never below itself
                        && (other_brick.0.0..=other_brick.1.0).contains(&x) && (other_brick.0.1..=other_brick.1.1).contains(&y)
                        // area overlap?
                        {
                            // other brick is in our way -> update maximum possible fall target
                            can_fall_to = max(can_fall_to, other_brick.1 .2 + 1);
                        }
                    }
                }
            }
            if can_fall_to != brick.0 .2 {
                fallen = true;
                fall_set.insert(brick_i);

                // Update brick position
                let dz = brick.1 .2 - brick.0 .2;
                let mut_brick = &mut bricks[brick_i];
                mut_brick.0 .2 = can_fall_to;
                mut_brick.1 .2 = can_fall_to + dz;

                if stop_after_first_fall == StopAfterFirstFall::Yes {
                    return fall_set;
                }
            }
        }
    }
    fall_set
}

pub fn part1() {
    let lines = file_lines("inp22_2.txt");

    // z: growing to sky
    // ground: z=0; bricks: z >= 1

    let mut bricks = parse_bricks(&lines);
    println!("{:?}", bricks);
    perform_fall(&mut bricks, StopAfterFirstFall::No);
    println!("{:?}", bricks);

    let can_disintegrate: usize = (0..bricks.len())
        .into_par_iter()
        .progress()
        .filter(|&brick_i| {
            let mut new_bricks = bricks.clone();
            new_bricks.remove(brick_i);

            let fall_set = perform_fall(&mut new_bricks, StopAfterFirstFall::Yes);
            fall_set.is_empty()
        })
        .count();

    println!("{}", can_disintegrate);
}

pub fn part2() {
    let lines = file_lines("inp22_2.txt");

    let mut bricks = parse_bricks(&lines);

    perform_fall(&mut bricks, StopAfterFirstFall::No);

    let total_fall_count: usize = (0..bricks.len())
        .into_par_iter()
        .progress()
        .map(|brick_i| {
            let mut new_bricks = bricks.clone();
            new_bricks.remove(brick_i);

            let fall_set = perform_fall(&mut new_bricks, StopAfterFirstFall::No);
            fall_set.len()
        })
        .sum();

    println!("{}", total_fall_count);
}
