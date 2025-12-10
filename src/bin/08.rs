#![allow(unexpected_cfgs)]

use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Vec3(usize, usize, usize);

impl TryFrom<&str> for Vec3 {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let (x, y, z) = value
            .split(",")
            .filter_map(|num| num.parse::<usize>().ok())
            .collect_tuple::<(usize, usize, usize)>()
            .ok_or(anyhow!("failed to parse from string"))?;

        Ok(Self(x, y, z))
    }
}

impl Vec3 {
    fn distance(&self, other: &Self) -> f64 {
        let (x1, y1, z1) = (
            self.0.to_string().parse::<f64>().unwrap(),
            self.1.to_string().parse::<f64>().unwrap(),
            self.2.to_string().parse::<f64>().unwrap(),
        );
        let (x2, y2, z2) = (
            other.0.to_string().parse::<f64>().unwrap(),
            other.1.to_string().parse::<f64>().unwrap(),
            other.2.to_string().parse::<f64>().unwrap(),
        );

        ((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt()
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let coords = reader
            .lines()
            .filter_map(|line| Vec3::try_from(&(line.unwrap())[..]).ok())
            .collect_vec();

        println!("{coords:?}");

        // let closest_coords: HashMap<Vec3, (Vec3, f64)> = coords
        //     .iter()
        //     .map(|first| {
        //         (
        //             *first,
        //             coords.iter().filter(|second| !first.eq(second)).fold(
        //                 (Vec3::default(), f64::MAX),
        //                 |acc, second| {
        //                     let diff = first.distance(second);
        //                     if diff < acc.1 {
        //                         (*second, diff)
        //                     } else {
        //                         acc
        //                     }
        //                 },
        //             ),
        //         )
        //     })
        //     .collect();
        //
        // let mut used_circuits: Vec<Vec3> = vec![];
        // let mut circuits: Vec<Vec<Vec3>> = vec![];

        // println!("{closest_coords:?}");

        // for (first, (second, _)) in closest_coords {
        //     let Some(circuit) = circuits.iter_mut().find(|circuit| {
        //         circuit
        //             .iter()
        //             .any(|other| first.eq(other) || second.eq(other))
        //     }) else {
        //         circuits.push(vec![first, second]);
        //         continue;
        //     };
        //
        //     if circuit.contains(&first) {
        //         continue;
        //     }
        //
        //     if circuit.contains(&second) {
        //
        //     circuit.push(first);
        // }

        // for this in &coords {
        //     if circuits
        //         .iter_mut()
        //         .any(|circuit| circuit.iter().any(|other| this.eq(other)))
        //     {
        //         continue;
        //     }
        //
        //     let next_closest = coords
        //         .iter()
        //         .filter(|other| !this.eq(other) && !used_circuits.contains(other))
        //         .map(|other| (*other, this.distance(other)))
        //         .sorted_by(|a, b| a.1.total_cmp(&b.1))
        //         .take(1)
        //         .collect_array::<1>()
        //         .unwrap();
        //     let next_closest = next_closest.first().unwrap();
        //
        //     println!("next closest for {this:?} is {next_closest:?}");
        // }

        let permut: Vec<(Vec3, Vec3, f64)> = coords
            .iter()
            .flat_map(|first| {
                coords
                    .iter()
                    .filter(|second| !first.eq(second))
                    .map(|second| {
                        let diff = first.distance(second);
                        (*first, *second, diff)
                    })
                    .collect_vec()
            })
            .sorted_by(|a, b| a.2.total_cmp(&b.2))
            .collect();

        println!("{permut:?}");

        let circuits: Vec<Vec<u32>> = vec![vec![1]];

        for circuit in &circuits {
            println!("len: {} => {circuit:?}", circuit.len());
            println!("----------------");
        }

        let result = circuits
            .iter()
            .map(|circuit| circuit.len())
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .collect_vec();

        println!("{result:?}");

        let result = result.iter().product::<usize>();

        // for (first, second, diff) in diffs {
        //     println!("{first:?} - {second:?} = {diff}");
        // }

        Ok(result)
    }
    // 5 * 4 * 2
    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
