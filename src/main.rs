#![feature(get_many_mut, map_try_insert, ascii_char, slice_split_once)]
#![allow(dead_code)]

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

    let grid: Vec<Vec<u8>> = io::BufReader::new(file)
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

    let grid: Vec<Vec<u8>> = io::BufReader::new(file)
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
            let _ = parts.next().unwrap();

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

        let current_map = maps.last_mut().unwrap();
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

        let current_map = maps.last_mut().unwrap();
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

fn day6_1() {
    let file = File::open(Path::new("inp6_2.txt")).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let times: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .replace(" ", "")
        .split(':')
        .skip(1)
        .map(|t| t.parse::<i64>().unwrap())
        .collect();

    let distances: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .replace(" ", "")
        .split(':')
        .skip(1)
        .map(|t| t.parse::<i64>().unwrap())
        .collect();

    let races = times.iter().zip(distances.iter());

    let ways_to_win_per_race = races.map(|race| {
        let time = *race.0;
        let best_distance = *race.1;
        let win_distances = (1..time)
            .map(|charge_time| {
                let travel_time = time - charge_time;
                let speed = charge_time;
                let d = speed * travel_time;
                //println!("     {}", d);
                return d;
            })
            .filter(|d| *d > best_distance);
        let no_win_distances = win_distances.count();
        //println!("- {}", no_win_distances);
        return no_win_distances as i64;
    });

    let solution: i64 = ways_to_win_per_race.product();
    println!("{}", solution);
}

fn day6_2() {
    let file = File::open(Path::new("inp6_2.txt")).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let times: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .replace(" ", "")
        .split(':')
        .skip(1)
        .map(|t| t.parse::<i64>().unwrap())
        .collect();

    let distances: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .replace(" ", "")
        .split(':')
        .skip(1)
        .map(|t| t.parse::<i64>().unwrap())
        .collect();

    let races = times.iter().zip(distances.iter());

    let ways_to_win_per_race = races.map(|race| {
        let time = *race.0;
        let best_distance = *race.1;

        let det = ((time * time - 4 * best_distance) as f64).sqrt();
        let min_charge_time = ((time as f64 - det) / 2.0).ceil() as i64;
        let max_charge_time = ((time as f64 + det) / 2.0).floor() as i64;

        let no_win_distances = max_charge_time - min_charge_time + 1;
        println!("- {}", no_win_distances);
        return no_win_distances as i64;
    });

    let solution: i64 = ways_to_win_per_race.product();
    println!("{}", solution);
}

fn day7_1() {
    let file = File::open(Path::new("inp7_2.txt")).unwrap();
    let lines = io::BufReader::new(file).lines();

    #[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
    enum HandType {
        FiveOfAKind = 7,
        FourOfAKind = 6,
        FullHouse = 5,
        ThreeOfAKind = 4,
        TwoPair = 3,
        OnePair = 2,
        HighCard = 1,
    }
    type Hand = Vec<i32>;

    fn determine_hand_type(hand: &Hand) -> HandType {
        let mut hand_bins = BTreeMap::new();
        for card in hand {
            let def = 0;
            let old_val = hand_bins.get(&card).unwrap_or(&def);
            hand_bins.insert(card, old_val + 1);
        }
        if hand_bins.values().any(|count| *count == 5) {
            return HandType::FiveOfAKind;
        }
        if hand_bins.values().any(|count| *count == 4) {
            return HandType::FourOfAKind;
        }
        let pairs = hand_bins.values().filter(|count| **count == 2);
        let pair_count = pairs.count();
        if hand_bins.values().any(|count| *count == 3) {
            if pair_count == 1 {
                return HandType::FullHouse;
            }
            return HandType::ThreeOfAKind;
        }
        if pair_count == 2 {
            return HandType::TwoPair;
        }
        if pair_count == 1 {
            return HandType::OnePair;
        }
        return HandType::HighCard;
    }

    fn parse_hand(hand: &str) -> Hand {
        return hand
            .chars()
            .map(|c| {
                return match c {
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!(),
                };
            })
            .collect();
    }

    #[derive(Debug)]
    struct ParsedHand {
        hand: Hand,
        hand_type: HandType,
        bid: i64,
    }

    let mut all_hands: Vec<ParsedHand> = lines
        .map(|l_| {
            let l = l_.unwrap();
            let split = l.split_once(' ').unwrap();
            let hand_str = split.0;
            let hand = parse_hand(hand_str);
            let hand_type = determine_hand_type(&hand);
            let bid = split.1;

            return ParsedHand {
                hand: hand,
                hand_type: hand_type,
                bid: bid.parse().unwrap(),
            };
        })
        .collect();

    for hand in &all_hands {
        println!("- {:?}", hand);
    }
    println!("---");

    all_hands.sort_unstable_by(|a, b| {
        if a.hand_type == b.hand_type {
            let first_ineq_card = a
                .hand
                .iter()
                .zip(b.hand.iter())
                .find(|(card_a, card_b)| card_a != card_b)
                .unwrap();

            return first_ineq_card.0.cmp(first_ineq_card.1);
        }
        return a.hand_type.cmp(&b.hand_type);
    });

    for hand in &all_hands {
        println!("- {:?}", hand);
    }
    let sum = all_hands.iter().enumerate().fold(0 as i64, |acc, e| {
        let rank = e.0 + 1;
        return acc + (rank as i64) * e.1.bid;
    });
    println!("{}", sum);
}

fn day7_2() {
    let file = File::open(Path::new("inp7_2.txt")).unwrap();
    let lines = io::BufReader::new(file).lines();

    #[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
    enum HandType {
        FiveOfAKind = 7,
        FourOfAKind = 6,
        FullHouse = 5,
        ThreeOfAKind = 4,
        TwoPair = 3,
        OnePair = 2,
        HighCard = 1,
    }
    type Hand = Vec<i32>;
    const JOKER: i32 = 1;

    fn determine_hand_type(hand: &Hand) -> HandType {
        let mut hand_bins = BTreeMap::new();
        for &card in hand {
            hand_bins
                .entry(card)
                .and_modify(|c| {
                    *c += 1;
                })
                .or_insert(1);
        }
        let jokers = hand_bins.remove(&JOKER).unwrap_or(0);
        let (&best_card, &best_card_count) = hand_bins
            .iter()
            .max_by_key(|(_, &v)| v)
            .unwrap_or((&JOKER, &0)) // only jokers
            ;
        hand_bins.insert(best_card, best_card_count + jokers);
        let mut sorted_counts: Vec<_> = hand_bins.values().collect();
        sorted_counts.sort_unstable_by(|a, b| b.cmp(a));
        return match sorted_counts[0] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match sorted_counts[1] {
                2 => HandType::FullHouse,
                1 => HandType::ThreeOfAKind,
                _ => panic!(),
            },
            2 => match sorted_counts[1] {
                2 => HandType::TwoPair,
                1 => HandType::OnePair,
                _ => panic!(),
            },
            1 => HandType::HighCard,
            _ => panic!(),
        };
    }

    fn parse_hand(hand: &str) -> Hand {
        return hand
            .chars()
            .map(|c| match c {
                '2'..='9' => c.to_digit(10).unwrap() as i32,
                'T' => 10,
                'J' => JOKER,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!(),
            })
            .collect();
    }

    #[derive(Debug)]
    struct ParsedHand {
        hand: Hand,
        hand_text: String,
        hand_type: HandType,
        bid: i64,
    }

    let mut all_hands: Vec<ParsedHand> = lines
        .map(|l_| {
            let l = l_.unwrap();
            let split = l.split_once(' ').unwrap();
            let hand_str = split.0;
            let hand = parse_hand(hand_str);
            let hand_type = determine_hand_type(&hand);
            let bid = split.1;

            return ParsedHand {
                hand,
                hand_text: hand_str.to_string(),
                hand_type,
                bid: bid.parse().unwrap(),
            };
        })
        .collect();

    for hand in &all_hands {
        println!("- {:?}", hand);
    }
    println!("---");

    all_hands.sort_unstable_by(|a, b| {
        if a.hand_type == b.hand_type {
            return a.hand.cmp(&b.hand);
        }
        return a.hand_type.cmp(&b.hand_type);
    });

    for hand in &all_hands {
        println!("- {:?}", hand);
    }
    let sum = all_hands.iter().enumerate().fold(0 as i64, |acc, e| {
        let rank = e.0 + 1;
        return acc + (rank as i64) * e.1.bid;
    });
    println!("{}", sum);
}

fn day8_1() {
    let file = File::open(Path::new("inp8_2.txt")).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let lr: Vec<_> = lines.next().unwrap().unwrap().chars().collect();
    let mut lr_iter = lr.iter().cycle();
    lines.next();

    let instr_re = Regex::new(r"^(\w+) = \((\w+), (\w+)\)$").unwrap();

    struct Node {
        next_l: String,
        next_r: String,
    }

    let mut node_map: BTreeMap<String, Node> = BTreeMap::new();

    for line in lines {
        let instr = line.unwrap();
        let instr_s = instr.as_str();
        let (_, [this_node, next_node_l, next_node_r]) =
            instr_re.captures(&instr_s).unwrap().extract();
        node_map.insert(
            this_node.to_string(),
            Node {
                next_l: next_node_l.to_string(),
                next_r: next_node_r.to_string(),
            },
        );
    }

    let mut cur_node = "AAA".to_string();
    let mut steps = 0;
    while cur_node != "ZZZ" {
        let instr = node_map.get(cur_node.as_str()).unwrap();
        let next_lr = lr_iter.next().unwrap();
        cur_node = match next_lr {
            'L' => &instr.next_l,
            'R' => &instr.next_r,
            _ => panic!(),
        }
        .to_string();
        steps += 1;
    }
    println!("{}", steps);
}

fn day8_2() {
    let file = File::open(Path::new("inp8_2.txt")).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let lr: Vec<_> = lines.next().unwrap().unwrap().chars().collect();
    let mut lr_iter = lr.iter().cycle();
    lines.next();

    let instr_re = Regex::new(r"^(\w+) = \((\w+), (\w+)\)$").unwrap();

    #[derive(Debug)]
    struct Node {
        next_l: String,
        next_r: String,
    }

    let mut node_map: BTreeMap<String, Node> = BTreeMap::new();

    for line in lines {
        let instr = line.unwrap();
        let instr_s = instr.as_str();
        let (_, [this_node, next_node_l, next_node_r]) =
            instr_re.captures(&instr_s).unwrap().extract();
        node_map.insert(
            this_node.to_string(),
            Node {
                next_l: next_node_l.to_string(),
                next_r: next_node_r.to_string(),
            },
        );
    }

    let mut cur_nodes: Vec<_> = node_map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| k.clone())
        .collect();
    let mut steps = 0;
    while !cur_nodes.iter().all(|n| n.ends_with("Z")) {
        let next_lr = lr_iter.next().unwrap();
        cur_nodes = cur_nodes
            .iter()
            .map(|cur_node| {
                let instr = node_map.get(cur_node.as_str()).unwrap();
                return match next_lr {
                    'L' => &instr.next_l,
                    'R' => &instr.next_r,
                    _ => panic!(),
                }
                .to_string();
            })
            .collect();

        for (i, node) in cur_nodes.iter().enumerate() {
            if node.ends_with("Z") {
                println!("[{}] {}: OK in step {}", i, node, steps);
                // Now use calculator... :)
            }
        }

        steps += 1;
    }
    // println!("{}", steps);
}

fn day9_1() {
    let file = File::open(Path::new("inp9_2.txt")).unwrap();
    let lines = io::BufReader::new(file).lines();

    let next_vals = lines.map(|l| {
        let line = l.unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut histories = vec![nums];
        while !histories.last().unwrap().iter().all(|&c| c == 0) {
            let this = histories.last().unwrap();
            let iter_a = this.iter().take(this.len() - 1);
            let iter_b = this.iter().skip(1);
            let pairs = iter_a.zip(iter_b);
            let diffs = pairs.map(|(a, b)| b - a).collect();
            histories.push(diffs);
        }
        // FIXME for loop with negative step?
        for i in 0..histories.len() - 1 {
            let len = histories.len();
            // FIXME ugly :( but it doesn't seem to be possible to borrow more than one array item
            let mut iter_mut = histories.iter_mut();
            let before = iter_mut.nth(len - i - 2).unwrap();
            let this = iter_mut.next().unwrap();
            let new_el = this.last().unwrap() + before.last().unwrap();
            before.push(new_el);
        }
        println!("{:?}", histories);
        return *histories[0].last().unwrap();
    });
    let sum: i32 = next_vals.sum();
    println!("{}", sum);
}

fn day9_2() {
    let file = File::open(Path::new("inp9_2.txt")).unwrap();
    let lines = io::BufReader::new(file).lines();

    let next_vals = lines.map(|l| {
        let line = l.unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut histories = vec![nums];
        while !histories.last().unwrap().iter().all(|&c| c == 0) {
            let this = histories.last().unwrap();
            let pairs = this.windows(2);
            let diffs = pairs.map(|s| s[1] - s[0]).collect();
            histories.push(diffs);
        }
        for i in num::range_step((histories.len() - 1) as i32, 0, -1) {
            let [before, this] = histories
                .get_many_mut([i as usize - 1, i as usize])
                .unwrap();
            let new_el = -this[0] + before[0];
            before.insert(0, new_el);
        }
        println!("{:?}", histories);
        return histories[0][0];
    });
    let sum: i32 = next_vals.sum();
    println!("{}", sum);
}

fn day10() {
    let file = File::open(Path::new("inp10_2.txt")).unwrap();

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Tile {
        Ground,
        NS,
        WE,
        NE,
        NW,
        SW,
        SE,
        Start,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum LoopCover {
        Unknown,
        In,
        Out,
    }

    impl LoopCover {
        fn opposite(&self) -> LoopCover {
            match self {
                LoopCover::In => LoopCover::Out,
                LoopCover::Out => LoopCover::In,
                LoopCover::Unknown => LoopCover::Unknown,
            }
        }
    }

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
    }

    impl Tile {
        fn connections(&self) -> [bool; 4] {
            const F: bool = false;
            const T: bool = true;
            match self {
                Tile::Ground => [F, F, F, F],
                Tile::NS => [T, F, T, F],
                Tile::WE => [F, T, F, T],
                Tile::NE => [T, T, F, F],
                Tile::NW => [T, F, F, T],
                Tile::SW => [F, F, T, T],
                Tile::SE => [F, T, T, F],
                Tile::Start => [T, T, T, T],
            }
        }

        fn connects_to(&self, next_tile: Tile, direction: Direction) -> bool {
            let con_a = self.connections();
            let con_b = next_tile.connections();

            con_a[direction as usize] && con_b[direction.opposite() as usize]
        }

        fn repr(&self) -> char {
            match self {
                Tile::WE => '─',
                Tile::NS => '│',
                Tile::NW => '┘',
                Tile::NE => '└',
                Tile::SE => '┌',
                Tile::SW => '┐',
                Tile::Start => 'S',
                Tile::Ground => ' ',
            }
        }

        fn from_aoc_repr(c: char) -> Tile {
            match c {
                '.' => Tile::Ground,
                '-' => Tile::WE,
                '|' => Tile::NS,
                'L' => Tile::NE,
                'J' => Tile::NW,
                '7' => Tile::SW,
                'F' => Tile::SE,
                'S' => Tile::Start,
                _ => panic!(),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct GridEntry {
        tile: Tile,
        dist_from_start: usize,
        loop_cover: LoopCover,
        incoming_direction: Option<Direction>,
        outgoing_direction: Option<Direction>,
    }

    type Grid = Vec<Vec<GridEntry>>;
    let mut grid: Grid = io::BufReader::new(file)
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let row: Vec<_> = line
                .chars()
                .map(|c| GridEntry {
                    tile: Tile::from_aoc_repr(c),
                    dist_from_start: 0,
                    loop_cover: LoopCover::Unknown,
                    incoming_direction: None,
                    outgoing_direction: None,
                })
                .collect();
            return row;
        })
        .collect();

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Position {
        x: usize,
        y: usize,
    }

    let mut start_pos: Option<Position> = None;
    // FIXME nicer way to do this with iters?
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col.tile == Tile::Start {
                start_pos = Some(Position { x, y });
                break;
            }
        }
    }
    assert!(start_pos.is_some());

    let grid_w = grid[0].len();
    let grid_h = grid.len();

    let advance_pos = |pos: Position, dir: Direction| match dir {
        Direction::N if pos.y >= 1 => Some(Position {
            x: pos.x,
            y: pos.y - 1,
        }),
        Direction::E if pos.x < grid_w - 1 => Some(Position {
            x: pos.x + 1,
            y: pos.y,
        }),
        Direction::S if pos.y < grid_h - 1 => Some(Position {
            x: pos.x,
            y: pos.y + 1,
        }),
        Direction::W if pos.x >= 1 => Some(Position {
            x: pos.x - 1,
            y: pos.y,
        }),
        _ => None,
    };

    let mut next_pos = start_pos;

    while next_pos.is_some() {
        let pos = next_pos.take().unwrap();
        let grid_entry = grid[pos.y][pos.x];

        let mut check_dir = |dir: Direction| -> bool {
            if grid_entry.incoming_direction == Some(dir) {
                return false;
            }
            let new_pos = match advance_pos(pos, dir) {
                None => return false,
                Some(p) => p,
            };
            let new_grid_entry = &mut grid[new_pos.y][new_pos.x];
            if grid_entry.tile.connects_to(new_grid_entry.tile, dir)
                && new_grid_entry.dist_from_start == 0
            {
                new_grid_entry.dist_from_start = grid_entry.dist_from_start + 1;
                new_grid_entry.incoming_direction = Some(dir.opposite());
                if new_grid_entry.tile != Tile::Start {
                    next_pos = Some(new_pos);
                }
                grid[pos.y][pos.x].outgoing_direction = Some(dir);
                return true;
            }
            false
        };
        // Find connected adjacent tile
        if check_dir(Direction::N) {
            continue;
        }
        if check_dir(Direction::E) {
            continue;
        }
        if check_dir(Direction::S) {
            continue;
        }
        if check_dir(Direction::W) {
            continue;
        }
    }

    // Replace start tile
    let start_entry = &mut grid[start_pos.unwrap().y][start_pos.unwrap().x];
    let (start_in, start_out) = (
        start_entry.incoming_direction.unwrap(),
        start_entry.outgoing_direction.unwrap(),
    );
    start_entry.tile = match start_in {
        Direction::N => match start_out {
            Direction::E => Tile::NE,
            Direction::S => Tile::NS,
            Direction::W => Tile::NW,
            _ => panic!(),
        },
        Direction::E => match start_out {
            Direction::N => Tile::NE,
            Direction::S => Tile::SE,
            Direction::W => Tile::WE,
            _ => panic!(),
        },
        Direction::S => match start_out {
            Direction::N => Tile::NS,
            Direction::E => Tile::SE,
            Direction::W => Tile::SW,
            _ => panic!(),
        },
        Direction::W => match start_out {
            Direction::N => Tile::NW,
            Direction::E => Tile::WE,
            Direction::S => Tile::SW,
            _ => panic!(),
        },
    };

    fn print_grid(grid: &Grid, elem_closure: fn(&GridEntry) -> char) {
        println!(
            "{}",
            grid.iter()
                .map(|row| String::from_iter(row.iter().map(elem_closure)))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    print_grid(&grid, |e| e.tile.repr());
    print_grid(&grid, |e| match e.dist_from_start {
        0 => '.',
        _ => e.tile.repr(),
    });
    print_grid(&grid, |e| match e.incoming_direction {
        None => '.',
        Some(d) => d.repr(),
    });
    print_grid(&grid, |e| match e.outgoing_direction {
        None => '.',
        Some(d) => d.repr(),
    });

    fn get_vertical_direction(dir_a: Direction, dir_b: Direction) -> Direction {
        match dir_a {
            Direction::N | Direction::S => dir_a,
            Direction::W | Direction::E => dir_b,
        }
    }

    for row in grid.iter_mut() {
        let mut cover = LoopCover::Out;
        let mut bend_from_direction: Option<Direction> = None;
        for col in row.iter_mut() {
            // Is part of loop?
            if col.dist_from_start > 0 {
                match col.tile {
                    // Crossing straight pipe
                    Tile::NS => cover = cover.opposite(),
                    // Crossing begin of bend
                    Tile::NE | Tile::SE => {
                        bend_from_direction = Some(get_vertical_direction(
                            col.outgoing_direction.unwrap(),
                            col.incoming_direction.unwrap(),
                        ))
                    }
                    // Crossing end of bend
                    Tile::NW | Tile::SW => {
                        // Check if bend opens up space behind it
                        let vertical_direction = get_vertical_direction(
                            col.outgoing_direction.unwrap(),
                            col.incoming_direction.unwrap(),
                        );
                        if vertical_direction != bend_from_direction.unwrap() {
                            cover = cover.opposite();
                            bend_from_direction = None;
                        }
                    }
                    Tile::WE => { // Ignored since we are traversing rows
                    }
                    _ => panic!(),
                }
            } else {
                col.loop_cover = cover;
            }
        }
    }

    print_grid(&grid, |e| match e.loop_cover {
        LoopCover::In => 'I',
        LoopCover::Out => 'O',
        _ => '.',
    });

    let loop_len = grid[start_pos.unwrap().y][start_pos.unwrap().x].dist_from_start;
    println!("{}", loop_len / 2);

    let loop_area: usize = grid
        .iter()
        .map(|row| row.iter().filter(|e| e.loop_cover == LoopCover::In).count())
        .sum();

    println!("{}", loop_area);
}

mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod util;

fn main() {
    day21::part1();
}
