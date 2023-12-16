use std::{
    collections::VecDeque,
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
    let file = File::open(Path::new("inp12_2.txt")).unwrap();
    let lines = io::BufReader::new(file).lines().collect_vec();

    const OK: i8 = 0;
    const DAMAGED: i8 = 1;
    const UNKNOWN: i8 = -1;

    fn print_entry(entry: &QueueEntry) {
        println!(
            "{} | {:?}",
            entry
                .spring
                .iter()
                .map(|&c| match c {
                    OK => '.',
                    DAMAGED => '#',
                    UNKNOWN => '?',
                    _ => panic!(),
                })
                .collect::<String>(),
            entry.groups_to_assign
        );
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct QueueEntry {
        spring: Vec<i8>,
        groups_to_assign: Vec<usize>,
    }

    let spring_combinations = lines
        //.iter()
        .par_iter()
        .progress()
        .map(|l| {
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
            // Let "DAMAGED" counting loop always finish with OK
            spring.push(OK);

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

            let mut q = VecDeque::from([QueueEntry {
                spring,
                groups_to_assign: grouped_record,
            }]);

            let mut ok_combinations = 0;
            while let Some(entry) = q.pop_back() {
                if cfg!(debug_assertions) {
                    print_entry(&entry);
                }

                // Skip OK times at the start, they are not interesting
                let next_not_ok_char_idx = match entry.spring.iter().find_position(|&&c| c != OK) {
                    // All are OK, but we still have damages to place -> not possible
                    None => {
                        if cfg!(debug_assertions) {
                            println!("-> exit1");
                        }
                        continue;
                    }
                    Some(x) => x.0,
                };
                let spring = &entry.spring[next_not_ok_char_idx..];

                // Skip DAMAGED springs at the start and subtract them from the group to place
                /*let next_unk_char_idx = spring
                    .iter()
                    .find_position(|&&c| c == UNKNOWN)
                    .unwrap_or((0, &0))
                    .0;
                spring = &spring[next_unk_char_idx..]

                let group_to_assign = match entry.groups_to_assign[0].checked_sub(next_unk_char_idx)
                {
                    None => {
                        if cfg!(debug_assertions) {
                            println!("-> exit3");
                        }
                        continue;
                    }
                    Some(x) => x,
                };*/
                let group_to_assign = entry.groups_to_assign[0];

                // Now the spring starts with UNKNOWN, try to place the next group somewhere
                let max_group_len_from_here = spring
                    .iter()
                    .find_position(|&&c| c != DAMAGED && c != UNKNOWN)
                    .unwrap()
                    .0;

                if group_to_assign > max_group_len_from_here {
                    // Not possible to place group here, try without the next ?/# group
                    let next_ok_char_idx = match spring.iter().find_position(|&&c| c == OK) {
                        None => {
                            if cfg!(debug_assertions) {
                                println!("-> exit2");
                            }
                            continue;
                        }
                        Some(x) => x.0,
                    };
                    let next_spring = spring[next_ok_char_idx + 1..].into();

                    let next_entry = QueueEntry {
                        spring: next_spring,
                        groups_to_assign: entry.groups_to_assign.clone(),
                    };
                    q.push_back(next_entry);
                    /*if cfg!(debug_assertions) {
                        println!("-> exit");
                    }*/
                    continue;
                }

                /*if entry.groups_to_assign.len() == 1 {
                    // The last group is guaranteed to be assignable now
                    if ok_combinations % 1000 == 0 {
                        println!("[   {}   ]", ok_combinations);
                    }
                    println!("OK: ///");
                    print_entry(&entry);
                    println!("    \\\\\\");
                    ok_combinations += 1;
                    continue;
                }*/

                // Try all placings
                let mut first_fixed_damage: Option<usize> = None;
                for start_i in 0..=(max_group_len_from_here - group_to_assign) {
                    // Check: we cannot overrun a contiguous group in the input
                    let this_ch = spring[start_i];
                    if this_ch == DAMAGED {
                        first_fixed_damage.get_or_insert(start_i);
                    }
                    // Check: it has to be possible to make an OK after the group
                    let ch_after_group = spring[start_i + group_to_assign];
                    if ch_after_group == DAMAGED {
                        // Not possible
                        // FIXME break?
                        continue;
                    }
                    if let Some(first_fixed_val) = first_fixed_damage {
                        if start_i + group_to_assign > group_to_assign + first_fixed_val {
                            break;
                        }
                    }
                    // FIXME additional check that we do not overrun the max. possible length
                    // Place group
                    /*let mut next_spring: Vec<i8> = spring[start_i..].into();
                    for col in 0..group_to_assign {
                        next_spring[col] = DAMAGED;
                    }
                    next_spring[group_to_assign] = OK;*/

                    let next_spring = spring[start_i + group_to_assign + 1..].into();
                    let next_groups_to_assign = entry.groups_to_assign[1..].into();

                    let next_entry = QueueEntry {
                        spring: next_spring,
                        groups_to_assign: next_groups_to_assign,
                    };
                    if cfg!(debug_assertions) {
                        println!("start_i {} -> entry:", start_i);
                        print_entry(&next_entry);
                        println!("+");
                    }

                    if next_entry.groups_to_assign.is_empty() {
                        if ok_combinations % 1000 == 0 {
                            //println!("[   {}   ]", ok_combinations);
                        }
                        if cfg!(debug_assertions) {
                            println!("OK: ///");
                            print_entry(&entry);
                            println!("    \\\\\\");
                        }
                        ok_combinations += 1;
                    } else {
                        q.push_back(next_entry);
                    }
                }
                // Try skipping this placeholder altogether
                let next_entry = QueueEntry {
                    spring: spring[max_group_len_from_here..].into(),
                    groups_to_assign: entry.groups_to_assign,
                };
                q.push_back(next_entry);
                if cfg!(debug_assertions) {
                    println!("--");
                }

                /*let mut this_group_size = 0;
                for (idx, c) in spring.iter().enumerate() {
                    match *c {
                        DAMAGED => {
                            this_group_size += 1;
                            if this_group_size > group_to_assign {
                                // Exceeded what is possible here already
                                if cfg!(debug_assertions) {
                                    println!("-> exit 2");
                                }
                                break;
                            }
                        }
                        OK => {
                            if this_group_size > 0 {
                                // Reached end of group
                                if this_group_size == group_to_assign {
                                    // Group was OK, continue with assigning next
                                    let new_groups_to_assign: Vec<usize> =


                                    if new_groups_to_assign.is_empty() {
                                        // Done!
                                        // Need to check the rest...
                                        if spring.iter().all(|&c| c != DAMAGED) {
                                            if ok_combinations % 1000 == 0 {
                                                println!("[   {}   ]", ok_combinations);
                                            }
                                            ok_combinations += 1;
                                        } else {
                                            println!("-> exit3");
                                        }
                                        break;
                                    }
                                    let new_spring: Vec<i8> = spring[idx..].into();
                                    let _ = q.push_back(QueueEntry {
                                        spring: new_spring,
                                        groups_to_assign: new_groups_to_assign,
                                    });
                                } else {
                                    if cfg!(debug_assertions) {
                                        println!("-> exit");
                                    }
                                }
                                break;
                            }
                        }
                        UNKNOWN => {
                            // Branch out: try to place group at this index
                            let max_group_len_from_here = spring
                                .iter()
                                .skip(idx)
                                .find_position(|&&c| c != DAMAGED && c != UNKNOWN)
                                .unwrap()
                                .0;

                            let mut ok_spring: Vec<i8> = spring.into();
                            ok_spring[idx] = OK;
                            q.push_back(QueueEntry {
                                spring: ok_spring,
                                groups_to_assign: entry.groups_to_assign.clone(), //new_groups_to_assign.clone(),
                            });

                            let mut damaged_spring: Vec<i8> = spring.into();
                            damaged_spring[idx] = DAMAGED;
                            q.push_back(QueueEntry {
                                spring: damaged_spring,
                                groups_to_assign: entry.groups_to_assign.clone(), // new_groups_to_assign,
                            });
                            break;
                        }
                        _ => panic!(),
                    }
                }*/
            }

            ok_combinations
        });
    let sum: usize = spring_combinations.sum();
    println!("{}", sum);
}
