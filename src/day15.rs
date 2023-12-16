use itertools::Itertools;

use super::util;

fn hash(input: &str) -> u8 {
    input.chars().fold(0_u8, |acc, c| {
        (((acc as u64) + c.as_ascii().unwrap().to_u8() as u64) * 17 % 256) as u8
    })
}

// 00:11:28
pub fn part1() {
    let lines = util::file_lines("inp15_2.txt");
    let instructions = lines[0].split(',');
    let sum: u64 = instructions.map(|i| hash(i) as u64).sum();
    println!("{}", sum);
}

// 00:40:34
pub fn part2() {
    let lines = util::file_lines("inp15_2.txt");
    let instructions = lines[0].split(',');

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct Slot {
        lens_label: String,
        focal_length: u8,
    }

    let mut boxes = vec![Vec::<Slot>::new(); 256];

    for instr in instructions {
        if instr.chars().last().unwrap() == '-' {
            let lens_label = &instr[0..instr.chars().count() - 1];
            let box_no = hash(lens_label) as usize;
            let tbox = &mut boxes[box_no];
            let pos = tbox.iter().find_position(|s| s.lens_label == lens_label);
            if let Some(pos) = pos {
                tbox.remove(pos.0);
            }
        } else {
            let (lens_label, focal_length) = instr.split_once('=').unwrap();
            let focal_length = focal_length.parse().unwrap();
            let box_no = hash(lens_label) as usize;
            let tbox = &mut boxes[box_no];
            let pos = tbox
                .iter_mut()
                .find_position(|s| s.lens_label == lens_label);
            if let Some(pos) = pos {
                pos.1.focal_length = focal_length;
            } else {
                tbox.push(Slot {
                    lens_label: lens_label.to_string(),
                    focal_length,
                });
            }
        }
    }

    let sum = boxes
        .iter()
        .enumerate()
        .map(|(box_no, tbox)| {
            tbox.iter()
                .enumerate()
                .map(|(lens_no, slot)| (box_no + 1) * (lens_no + 1) * (slot.focal_length as usize))
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{}", sum);
}
