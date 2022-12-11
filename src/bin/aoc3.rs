#![feature(iter_array_chunks)]
use std::{
    collections::HashSet,
    error::Error,
    io::{BufRead, BufReader},
};

fn main() {
    perform("input/03/03.test").unwrap_or(());
    perform("input/03/03").unwrap_or(());
    puzzle2("input/03/03.test").unwrap_or(());
    puzzle2("input/03/03").unwrap_or(());
}

fn perform(input: &str) -> Result<(), Box<dyn Error>> {
    let file = BufReader::new(std::fs::OpenOptions::new().read(true).open(input)?);
    let score: u32 = file
        .lines()
        .flatten()
        .map(|line| {
            let half = line.len() / 2;
            let left_map: HashSet<_> = line[..half].chars().collect();
            let right_map: HashSet<_> = line[half..].chars().collect();
            let inter = left_map.intersection(&right_map).next().copied().unwrap();
            let res = if inter.is_uppercase() {
                (inter as u32) - ('A' as u32) + 27
            } else {
                (inter as u32) - ('a' as u32) + 1
            };
            res
        })
        .sum();
    println!("for {input}, total value is = {score}");
    Ok(())
}

fn puzzle2(input: &str) -> Result<(), Box<dyn Error>> {
    let file = BufReader::new(std::fs::OpenOptions::new().read(true).open(input)?);
    let score: u32 = file
        .lines()
        .flatten()
        .array_chunks::<3>()
        .map(|[line1, line2, line3]| {
            let first: HashSet<_> = line1.chars().collect();
            let second: HashSet<_> = line2.chars().collect();
            let third: HashSet<_> = line3.chars().collect();
            let inter: char = first
                .intersection(&second)
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&third)
                .copied()
                .next()
                .unwrap();
            let res = if inter.is_uppercase() {
                (inter as u32) - ('A' as u32) + 27
            } else {
                (inter as u32) - ('a' as u32) + 1
            };
            res
        })
        .sum();
    println!("puzzle2 for {input}, total value is = {score}");
    Ok(())
}
