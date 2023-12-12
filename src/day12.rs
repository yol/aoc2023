use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;

pub fn part1() {
    let file = File::open(Path::new("inp12_2.txt")).unwrap();
    let lines = io::BufReader::new(file).lines().peekable();

    const OK: i8 = 0;
    const DAMAGED: i8 = 1;
    const UNKNOWN: i8 = -1;

    let spring_combinations = lines.map(|l| {
        // FIXME why do I have to write this?
        let l_unwrap = l.unwrap();
        let mut line_parts = l_unwrap.split_whitespace();
        let spring = line_parts
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => OK,
                '#' => DAMAGED,
                '?' => UNKNOWN,
                _ => panic!(),
            })
            .collect_vec();
        let grouped_record = line_parts
            .next()
            .unwrap()
            .split(',')
            .map(|r| r.parse().unwrap())
            .collect_vec();
        let open_positions = spring
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == UNKNOWN)
            .map(|(pos, _)| pos)
            .collect_vec();

        (0..2_usize.pow(open_positions.len() as u32))
            .filter(|idx| {
                let mut this_spring = spring.clone();
                for (zero_idx, char_idx) in open_positions.iter().enumerate() {
                    let verdict = ((idx >> zero_idx) & 1) > 0;
                    this_spring[*char_idx] = match verdict {
                        false => OK,
                        true => DAMAGED,
                    }
                }
                let mut this_grouped_record: Vec<usize> = Vec::new();
                let mut this_group_size = 0_usize;
                // FIXME not nice
                for c in this_spring {
                    if c == DAMAGED {
                        this_group_size += 1;
                    } else {
                        if this_group_size > 0 {
                            this_grouped_record.push(this_group_size);
                        }
                        this_group_size = 0;
                    }
                }
                if this_group_size > 0 {
                    this_grouped_record.push(this_group_size);
                }
                this_grouped_record == grouped_record
            })
            .count()
    });
    let sum: usize = spring_combinations.sum();
    println!("{}", sum);
}

pub fn part2() {
    let file = File::open(Path::new("inp12_1.txt")).unwrap();
    let lines = io::BufReader::new(file).lines().collect_vec();

    const OK: i8 = 0;
    const DAMAGED: i8 = 1;
    const UNKNOWN: i8 = -1;

    let spring_combinations = lines.par_iter().progress().map(|l| {
        // FIXME why do I have to write this? why as_ref()?
        let l_unwrap = l.as_ref().unwrap();
        let mut line_parts = l_unwrap.split_whitespace();
        let orig_spring = line_parts
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => OK,
                '#' => DAMAGED,
                '?' => UNKNOWN,
                _ => panic!(),
            })
            .collect_vec();
        let mut spring = orig_spring.clone();
        for _ in 0..4 {
            let mut spring_copy = orig_spring.clone();
            spring.push(UNKNOWN);
            spring.append(&mut spring_copy);
        }

        let orig_grouped_record = line_parts
            .next()
            .unwrap()
            .split(',')
            .map(|r| r.parse().unwrap())
            .collect_vec();
        let mut grouped_record = orig_grouped_record.clone();
        for _ in 0..4 {
            let mut record_copy = orig_grouped_record.clone();
            grouped_record.append(&mut record_copy);
        }
        let total_damaged: usize = grouped_record.iter().sum();

        let open_positions = spring
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == UNKNOWN)
            .map(|(pos, _)| pos)
            .collect_vec();

        let clearly_damaged = spring.iter().filter(|&&c| c == DAMAGED).count();
        let damaged_to_assign = total_damaged - clearly_damaged;
        println!("{} C {}", open_positions.len(), damaged_to_assign);

        open_positions
            .iter()
            .combinations(damaged_to_assign)
            .filter(|damaged_positions| {
                let mut this_group_size = 0_usize;
                let mut check_iter = grouped_record.iter();
                // FIXME not nice
                for (idx, c) in spring.iter().enumerate() {
                    if *c == DAMAGED || damaged_positions.contains(&&idx) {
                        this_group_size += 1;
                    } else {
                        if this_group_size > 0 {
                            let to_check = check_iter.next();
                            if to_check.is_none() || *to_check.unwrap() != this_group_size {
                                return false;
                            }
                        }
                        this_group_size = 0;
                    }
                }
                let to_check = check_iter.next();
                if to_check.is_none() || *to_check.unwrap() != this_group_size {
                    return false;
                }
                true
            })
            .count()
    });
    let sum: usize = spring_combinations.sum();
    println!("{}", sum);
}
