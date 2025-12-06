#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
    Empty,
    Paper,
    Marked,
    Removed,
}

impl Debug for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Place::Empty => ".",
                Place::Paper => "@",
                Place::Marked => "#",
                Place::Removed => "x",
            }
        )
    }
}

impl Place {
    fn parse(value: char) -> Place {
        match value {
            '@' => Place::Paper,
            _ => Place::Empty,
        }
    }

    fn is_marked(&self, x: usize, y: usize, map: &[Vec<Place>]) -> bool {
        let ix = x as isize;
        let iy = y as isize;

        let adjasent = [
            (ix - 1, iy - 1),
            (ix, iy - 1),
            (ix + 1, iy - 1),
            (ix - 1, iy),
            (ix + 1, iy),
            (ix - 1, iy + 1),
            (ix, iy + 1),
            (ix + 1, iy + 1),
        ];

        let adjasent = adjasent
            .iter()
            .filter_map(|(x, y)| {
                let x = usize::try_from(*x).ok()?;
                let y = usize::try_from(*y).ok()?;
                let place = map.get(y)?.get(x)?;
                match place {
                    Place::Paper => Some(()),
                    _ => None,
                }
            })
            .count();

        adjasent < 4
    }
}

fn solver<R: BufRead>(reader: R, once: bool) -> Result<usize> {
    let mut answer = reader
        .lines()
        .map(|l| l.unwrap().chars().map(Place::parse).collect_vec())
        .collect_vec();

    let mut result = 0;
    let mut marked_p = Vec::with_capacity(10000);
    let mut removed_p = Vec::with_capacity(10000);

    loop {
        marked_p.clear();
        removed_p.clear();
        answer.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, place)| match place {
                Place::Empty | Place::Removed => (),
                Place::Paper => {
                    if place.is_marked(x, y, &answer) {
                        marked_p.push((x, y));
                    }
                }
                Place::Marked => {
                    removed_p.push((x, y));
                }
            })
        });

        for (x, y) in &marked_p {
            let place = answer.get_mut(*y).unwrap().get_mut(*x).unwrap();
            *place = Place::Marked;
        }

        for (x, y) in &removed_p {
            let place = answer.get_mut(*y).unwrap().get_mut(*x).unwrap();
            *place = Place::Removed;
        }

        if marked_p.is_empty() {
            break;
        }

        result += marked_p.len();

        if once {
            break;
        }
    }

    Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        solver(reader, true)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        solver(reader, false)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    let input_file = BufReader::new(File::open(concatcp!("input/", DAY, "-bobosher.txt"))?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
