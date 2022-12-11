#![feature(iter_array_chunks)]
use std::{
    collections::HashSet,
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

fn first(filename: &str) -> Result<(), Box<dyn Error>> {
    BufReader::new(OpenOptions::new().read(true).open(filename)?)
        .lines()
        .flatten()
        .enumerate()
        .for_each(|(linenum, x)| {
            let len = x.len();
            let first = (0..(len - 14))
                .skip_while(|index| {
                    let mut x = x.chars();
                    let set = HashSet::from([
                        x.nth(*index),
                        x.next(),
                        x.next(),
                        x.next(),
                        x.next(),
                        x.next(),
                        x.next(),
                        x.next(),
                        x.next(),
                        x.next(),
                        x.next(),
                        x.next(),
                        x.next(),
                        x.next(),
                    ]);
                    set.len() != 14
                })
                .next()
                .map(|x| x + 14)
                .unwrap();
            println!("for input {filename} line: {linenum} first section: {first}")
        });

    Ok(())
}

fn main() {
    first("input/06/06.test").unwrap_or(());
    first("input/06/06").unwrap_or(());
}
