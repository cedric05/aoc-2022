use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

struct ParseError;

#[derive(Debug)]
struct Bar(u32, u32);

impl TryFrom<&str> for Bar {
    type Error = ParseError;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut line = line.split("-").map(str::parse::<u32>).flatten();
        Ok(Bar(
            line.next().ok_or(ParseError)?,
            line.next().ok_or(ParseError)?,
        ))
    }
}

impl Bar {
    fn part_of(&self, second: &Self) -> bool {
        self.0 <= second.0 && self.1 >= second.1
    }

    fn either_part_of(&self, second: &Self) -> bool {
        self.part_of(second) || second.part_of(self)
    }

    fn left_overlap(&self, second: &Self) -> bool {
        (self.1 >= second.0) && (self.0 <= second.1)
    }
    fn either_overlap(&self, second: &Self) -> bool {
        self.left_overlap(second) || second.left_overlap(self)
    }
}

fn first(filename: &str) -> Result<(), Box<dyn Error>> {
    match BufReader::new(OpenOptions::new().read(true).open(filename)?)
        .lines()
        .flatten()
        .try_fold((0, 0), |(mut part_of, mut overlap_of), line| {
            let mut bars = line.split(",").map(Bar::try_from).flatten();
            let bar1 = bars.next().ok_or(ParseError)?;
            let bar2 = bars.next().ok_or(ParseError)?;
            if bar1.either_part_of(&bar2) {
                part_of += 1;
            }
            if bar1.either_overlap(&bar2) {
                overlap_of += 1;
            }
            Result::<_, ParseError>::Ok((part_of, overlap_of))
        }) {
        Ok((part_of, overlap_of)) => {
            println!("for file: {filename}, entirely part = {part_of}, overlap = {overlap_of}");
        }
        Err(_) => {
            println!("parse error");
        }
    }

    Ok(())
}

fn main() {
    first("input/04/04.test").unwrap_or(());
    first("input/04/04").unwrap_or(());
}
