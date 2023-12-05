use regex::Regex;
use std::cmp::{max, min};
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;

fn numeral_to_int(c: &str) -> i64 {
    return match c {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!(),
    };
}

fn day1() {
    let path = Path::new("inp2.txt");
    let file = match File::open(&path) {
        Err(why) => panic!("{}", why),
        Ok(file) => file,
    };
    let re =
        Regex::new(r"(0|1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let calib_vals = io::BufReader::new(file).lines().map(|l| {
        let calib = l.unwrap();
        let p1 = re.find(&calib).unwrap();
        let mut p2 = p1;
        loop {
            p2 = match re.find_at(&calib, p2.start() + 1) {
                None => break,
                Some(x) => x,
            };
        }
        let res = 10 * numeral_to_int(p1.as_str()) + numeral_to_int(p2.as_str());
        return res;
    });
    let sum: i64 = calib_vals.sum();
    println!("sum: {}", sum);
}

fn day2() {
    let file = File::open(Path::new("inp2_2.txt")).unwrap();
    // why do I have to give the type here?
    let games: Vec<(i64, Vec<[i64; 3]>)> = io::BufReader::new(file)
        .lines()
        .map(|l| {
            let line = l.unwrap();

            let mut game_split = line.split(":");
            let game_id_str = game_split.next().unwrap();
            let game_content_str = game_split.next().unwrap();

            let game_id: i64 = game_id_str[5..].parse().unwrap();

            let contents: Vec<[i64; 3]> = game_content_str
                .split(";")
                .map(|round| {
                    let round_contents_str = round.split(",");
                    let mut round_contents = [0, 0, 0];
                    round_contents_str.for_each(|e| {
                        let mut p = e.trim().split(" ");
                        let count: i64 = p.next().unwrap().parse().unwrap();
                        let index = match p.next().unwrap() {
                            "red" => 0,
                            "green" => 1,
                            "blue" => 2,
                            _ => panic!(),
                        };
                        round_contents[index] += count;
                    });
                    return round_contents;
                })
                .collect();
            return (game_id, contents);
        })
        .collect();

    // Part 1
    let ok_games = games.iter().filter(|game| {
        let limits = [12, 13, 14];
        let possible = game
            .1
            .iter()
            .all(|round| round[0] <= limits[0] && round[1] <= limits[1] && round[2] <= limits[2]);

        return possible;
    });

    let sum_ok_games: i64 = ok_games.map(|g| g.0).sum();
    println!("{}", sum_ok_games);

    // Part 2
    let power_per_game = games.iter().map(|game| {
        let mut minimum_cubes = [0, 0, 0];
        for round in &game.1 {
            for i in 0..round.len() {
                minimum_cubes[i] = max(minimum_cubes[i], round[i]);
            }
        }
        let power: i64 = minimum_cubes.iter().product();
        return power;
    });
    let sum_power: i64 = power_per_game.sum();
    println!("{}", sum_power);
}

fn day3_1() {
    let file = File::open(Path::new("inp3_2.txt")).unwrap();

    let mut grid: Vec<Vec<u8>> = io::BufReader::new(file)
        .lines()
        .map(|l| {
            let line = l.unwrap();
            return line.bytes().collect();
        })
        .collect();

    let num_re = Regex::new(r"\d+").unwrap();
    let mut part_nos: Vec<i64> = Vec::new();

    for (line_no, line) in grid.iter().enumerate() {
        num_re
            .find_iter(str::from_utf8(&line).unwrap())
            .for_each(|num_match| {
                let part_no: i64 = num_match.as_str().parse().unwrap();
                let start = num_match.start().saturating_sub(1);
                let end = min(line.len(), num_match.end() + 1);

                fn is_symbol(c: char) -> bool {
                    return c != '.' && !c.is_numeric();
                }

                fn has_symbol(s: &[u8]) -> bool {
                    return str::from_utf8(&s).unwrap().find(is_symbol).is_some();
                }

                let mut adj_symbol = false;

                if line_no > 0 {
                    let line_above = &grid[line_no - 1][start..end];
                    adj_symbol = adj_symbol || has_symbol(line_above);
                }

                adj_symbol = adj_symbol
                    || is_symbol(line[start] as char)
                    || is_symbol(line[end - 1] as char);

                if line_no < (grid.len() - 1) {
                    let line_below = &grid[line_no + 1][start..end];
                    adj_symbol = adj_symbol || has_symbol(line_below);
                }

                if !adj_symbol {
                    return;
                }

                println!("{}", part_no);
                part_nos.push(part_no);
            });
    }

    let part_no_sum: i64 = part_nos.iter().sum();
    println!("sum: {}", part_no_sum);
}

fn day3_2() {
    let file = File::open(Path::new("inp3_2.txt")).unwrap();

    let mut grid: Vec<Vec<u8>> = io::BufReader::new(file)
        .lines()
        .map(|l| {
            let line = l.unwrap();
            return line.bytes().collect();
        })
        .collect();

    let num_re = Regex::new(r"\d+").unwrap();

    struct Part {
        no: i64,
        pos: (usize, usize),
        sym_pos: (usize, usize),
    }

    let mut parts: Vec<Part> = Vec::new();

    for (line_no, line) in grid.iter().enumerate() {
        num_re
            .find_iter(str::from_utf8(&line).unwrap())
            .for_each(|num_match| {
                let part_no: i64 = num_match.as_str().parse().unwrap();
                let start = num_match.start().saturating_sub(1);
                let end = min(line.len(), num_match.end() + 1);

                fn is_symbol(c: char) -> bool {
                    //return c != '.' && !c.is_numeric();
                    return c == '*';
                }

                fn has_symbol(s: &[u8]) -> Option<usize> {
                    return str::from_utf8(&s).unwrap().find(is_symbol);
                }

                fn find_symbol(
                    line_no: usize,
                    grid: &Vec<Vec<u8>>,
                    start: usize,
                    end: usize,
                ) -> Option<(usize, usize)> {
                    let line = &grid[line_no];
                    if line_no > 0 {
                        let line_above = &grid[line_no - 1][start..end];
                        let sym_pos = has_symbol(line_above);
                        if sym_pos.is_some() {
                            return Some((line_no - 1, start + sym_pos.unwrap()));
                        }
                    }

                    if is_symbol(line[start] as char) {
                        return Some((line_no, start));
                    }
                    if is_symbol(line[end - 1] as char) {
                        return Some((line_no, end - 1));
                    }

                    if line_no < (grid.len() - 1) {
                        let line_below = &grid[line_no + 1][start..end];
                        let sym_pos = has_symbol(line_below);
                        if sym_pos.is_some() {
                            return Some((line_no + 1, start + sym_pos.unwrap()));
                        }
                    }

                    return None;
                }

                let sym_pos = find_symbol(line_no, &grid, start, end);

                if !sym_pos.is_some() {
                    return;
                }

                let part = Part {
                    no: part_no,
                    pos: (line_no, num_match.start()),
                    sym_pos: sym_pos.unwrap(),
                };

                //println!("{}", part.no);
                parts.push(part);
            });
    }

    let mut gear_ratio_sum = 0;
    for part in &parts {
        println!(
            "{} @{},{} sym@{},{}",
            part.no, part.pos.0, part.pos.1, part.sym_pos.0, part.sym_pos.1
        );
        let other_part = parts
            .iter()
            .find(|other_part| other_part.sym_pos == part.sym_pos && other_part.pos != part.pos);
        if other_part.is_none() {
            continue;
        }
        let gear_ratio = part.no * other_part.unwrap().no;
        println!("{}*{} -> {}", part.no, other_part.unwrap().no, gear_ratio);
        gear_ratio_sum += gear_ratio;
    }

    //let part_no_sum: i64 = part_nos.iter().sum();
    println!("sum: {}", gear_ratio_sum / 2);
}

fn day4_1() {
    let file = File::open(Path::new("inp4_2.txt")).unwrap();

    let sum: i64 = io::BufReader::new(file)
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let mut parts = line.splitn(3, |c| c == ':' || c == '|');
            let card_no_txt = parts.next().unwrap();

            fn parse_nos(no_str: &str) -> HashSet<i64> {
                return no_str
                    .split_whitespace()
                    .map(|s| s.trim().parse::<i64>().unwrap())
                    .collect();
            }

            let winning_nos = parse_nos(parts.next().unwrap());
            let card_nos = parse_nos(parts.next().unwrap());

            let winning_on_card = card_nos.intersection(&winning_nos);
            let winning_count = winning_on_card.count();
            let card_val = if winning_count == 0 {
                0
            } else {
                (2 as i64).pow((winning_count as u32) - 1)
            };
            return card_val;
        })
        .sum();

    println!("{}", sum);
}

fn day4_2() {
    let file = File::open(Path::new("inp4_2.txt")).unwrap();

    let mut card_counts: Vec<i64> = Vec::new();

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    card_counts.resize(lines.len(), 1);

    for line in lines {
        let mut parts = line.splitn(3, |c| c == ':' || c == '|');
        let card_no = parts
            .next()
            .unwrap()
            .split_whitespace()
            .next_back()
            .unwrap()
            .parse::<usize>()
            .unwrap()
            - 1;

        fn parse_nos(no_str: &str) -> HashSet<i64> {
            return no_str
                .split_whitespace()
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();
        }

        let winning_nos = parse_nos(parts.next().unwrap());
        let card_nos = parse_nos(parts.next().unwrap());

        let winning_on_card = card_nos.intersection(&winning_nos);
        let winning_count = winning_on_card.count();

        let this_card_count = card_counts[card_no];
        for i in 0..winning_count {
            card_counts[card_no + 1 + i] += this_card_count;
        }
    }

    let sum: i64 = card_counts.iter().sum();

    println!("{}", sum);
}

fn day5_1() {
    let file = File::open(Path::new("inp5_2.txt")).unwrap();

    struct DestRule {
        start: i64,
        length: i64,
    }

    struct ConvMap {
        source_type: String,
        dest_type: String,
        rules_source_to_dest: BTreeMap<i64, DestRule>,
    }

    let mut maps: Vec<ConvMap> = Vec::new();

    let mut lines = io::BufReader::new(file).lines();

    let seed_line = lines.next().unwrap().unwrap();
    let seeds: Vec<i64> = seed_line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    for line_w in lines {
        let line = line_w.unwrap();
        if line.is_empty() {
            continue;
        }

        let dash_parts: Vec<&str> = line.split('-').collect();
        if dash_parts.len() > 1 {
            let map = ConvMap {
                source_type: dash_parts[0].to_string(),
                dest_type: dash_parts[2].split_once(' ').unwrap().0.to_string(),
                rules_source_to_dest: BTreeMap::new(),
            };
            maps.push(map);
            continue;
        }

        let mut current_map = maps.last_mut().unwrap();
        let mut line_parts = line.split_whitespace();
        let dest_start = line_parts.next().unwrap().parse().unwrap();
        let src_start = line_parts.next().unwrap().parse().unwrap();
        let range_len = line_parts.next().unwrap().parse().unwrap();

        current_map.rules_source_to_dest.insert(
            src_start,
            DestRule {
                start: dest_start,
                length: range_len,
            },
        );
    }

    let min_loc = seeds
        .iter()
        .map(|seed| {
            let mut cur_type = "seed".to_string();
            let mut cur_no = *seed;

            loop {
                let next_map = maps.iter().find(|m| m.source_type == cur_type).unwrap();

                let rule = next_map
                    .rules_source_to_dest
                    .iter()
                    .find(|r| cur_no >= *r.0 && cur_no < r.0 + r.1.length);

                if rule.is_some() {
                    let rule_val = rule.unwrap();
                    let no_in_range = cur_no - rule_val.0;
                    cur_no = rule_val.1.start + no_in_range;
                }

                cur_type = next_map.dest_type.clone();

                if cur_type == "location" {
                    println!("OK: seed {} -> location {}", seed, cur_no);
                    return Some(cur_no);
                }
            }
        })
        .filter(|l| l.is_some())
        .map(|o| o.unwrap())
        .min()
        .unwrap();

    println!("min loc: {}", min_loc);
}

fn day5_2() {
    let file = File::open(Path::new("inp5_2.txt")).unwrap();

    #[derive(Copy, Clone, Debug)]
    struct Range {
        start: i64,
        length: i64,
    }
    impl Range {
        fn move_forward(&mut self, amount: i64) {
            self.start += amount;
            self.length -= amount;
            assert!(self.length >= 0);
        }
        fn is_empty(&self) -> bool {
            return self.length == 0;
        }
        fn overlaps(&self, other: &Range) -> bool {
            return (self.start >= other.start && self.start < other.start + other.length)
                || (self.start + self.length >= other.start
                    && self.start + self.length < other.start);
        }
    }

    struct ConvMap {
        source_type: String,
        dest_type: String,
        rules_source_to_dest: BTreeMap<i64, Range>,
    }

    let mut maps: Vec<ConvMap> = Vec::new();

    let mut lines = io::BufReader::new(file).lines();

    let seed_line = lines.next().unwrap().unwrap();
    let seeds: Vec<i64> = seed_line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    for line_w in lines {
        let line = line_w.unwrap();
        if line.is_empty() {
            continue;
        }

        let dash_parts: Vec<&str> = line.split('-').collect();
        if dash_parts.len() > 1 {
            let map = ConvMap {
                source_type: dash_parts[0].to_string(),
                dest_type: dash_parts[2].split_once(' ').unwrap().0.to_string(),
                rules_source_to_dest: BTreeMap::new(),
            };
            maps.push(map);
            continue;
        }

        let mut current_map = maps.last_mut().unwrap();
        let mut line_parts = line.split_whitespace();
        let dest_start = line_parts.next().unwrap().parse().unwrap();
        let src_start = line_parts.next().unwrap().parse().unwrap();
        let range_len = line_parts.next().unwrap().parse().unwrap();

        current_map.rules_source_to_dest.insert(
            src_start,
            Range {
                start: dest_start,
                length: range_len,
            },
        );
    }

    let loc_ranges = seeds.chunks_exact(2).flat_map(|seed| {
        let mut cur_type = "seed".to_string();
        let mut cur_ranges: Vec<Range> = Vec::new();
        cur_ranges.push(Range {
            start: seed[0],
            length: seed[1],
        });

        loop {
            let next_map = maps.iter().find(|m| m.source_type == cur_type).unwrap();

            let mut next_ranges: Vec<Range> = Vec::new();

            for range in cur_ranges {
                let mut remaining_range = range;

                println!(
                    "{} -> {} for range {:?}",
                    cur_type, next_map.dest_type, range
                );

                while !remaining_range.is_empty() {
                    let next_rule = next_map
                        .rules_source_to_dest
                        .iter() // should be sorted by key?
                        .find(|r| {
                            remaining_range.overlaps(&Range {
                                start: *r.0,
                                length: r.1.length,
                            })
                        });

                    match next_rule {
                        None => {
                            // Ran out of rules
                            next_ranges.push(remaining_range);
                            break;
                        }
                        Some(next_rule) => {
                            let remainder_before = next_rule.0 - remaining_range.start;
                            if remainder_before > 0 {
                                // Unmapped space before next rule
                                next_ranges.push(Range {
                                    start: remaining_range.start,
                                    length: remainder_before,
                                });
                                // Reduce scope
                                remaining_range.move_forward(remainder_before);
                            }
                            // Mapped space
                            let offset_to_rule = remaining_range.start - next_rule.0;
                            assert!(offset_to_rule >= 0);
                            let mapped_length =
                                min(next_rule.1.length - offset_to_rule, remaining_range.length);
                            next_ranges.push(Range {
                                start: next_rule.1.start + offset_to_rule,
                                length: mapped_length,
                            });
                            // Reduce scope
                            remaining_range.move_forward(mapped_length);
                        }
                    }
                }
            }
            println!(" yielded ranges: {:?}", next_ranges);

            cur_ranges = next_ranges;

            cur_type = next_map.dest_type.clone();

            if cur_type == "location" {
                break;
            }
        }

        return cur_ranges;
    });

    let min_loc = loc_ranges.map(|r| r.start).min().unwrap();

    println!("min loc: {}", min_loc);
}

fn main() {
    day5_2();
}
