use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use std::str::FromStr;

use itertools::Itertools;

/* pub fn parse_ints<T>(s: &str) -> Vec<T>
where
    T: FromStr,
{
    s.split(|c| c == ',' || c == ' ')
        .map(|p| p.parse::<T>().unwrap())
        .collect()
        .into()
} */

pub fn file_lines(filename: &str) -> Vec<String> {
    let file = File::open(Path::new(filename)).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect_vec()
}
