use std::{
    error::Error,
    io::{BufRead, BufReader},
};

fn main() {
    calculate_score("input/02/02.test").unwrap_or(());
    calculate_score("input/02/02").unwrap_or(());
}

/*
A => Rock
B => Paper
C => Scissors

X => Rock (1)
Y => Paper (2)
Z => Scissors (3)

rock > Scissors
scissors > paper
paper > rock

won = 6
lost = 0
draw = 3

/*
X => lose
Y => draw
Z => won
*/

*/

fn calculate_score(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = BufReader::new(std::fs::OpenOptions::new().read(true).open(filename)?);

    let score: (u32, u32) = file
        .lines()
        .map(|x| {
            let x = x.unwrap();
            (both_known(&x), outcome_known(&x))
        })
        .fold((0, 0), |(both, out), (both1, out2)| {
            (both + both1, out + out2)
        });
    println!(
        "for file= {}, score= {} outcome_socre= {}",
        filename, score.0, score.1
    );

    Ok(())
}

fn both_known(x: &str) -> u32 {
    match x {
        "A X" => {
            // draw + rock
            3 + 1
        }
        "A Y" => {
            // won +  paper
            6 + 2
        }
        "A Z" => {
            // lost +  paper
            0 + 3
        }
        "B X" => {
            // lost + rock
            0 + 1
        }
        "B Y" => {
            // draw + paper
            3 + 2
        }
        "B Z" => {
            // won + scissors
            6 + 3
        }
        "C X" => {
            // won + rock
            6 + 1
        }
        "C Y" => {
            // lost + paper
            0 + 2
        }
        "C Z" => {
            // draw + scissor
            3 + 3
        }
        _ => 0,
    }
}

fn outcome_known(x: &str) -> u32 {
    match x {
        "A X" => {
            // lose + rock
            0 + 3
        }
        "A Y" => {
            // draw +  rock
            3 + 1
        }
        "A Z" => {
            // won + paper
            6 + 2
        }
        "B X" => {
            // lost + rock
            0 + 1
        }
        "B Y" => {
            // draw + paper
            3 + 2
        }
        "B Z" => {
            // won + scissors
            6 + 3
        }
        "C X" => {
            // lose + paper
            0 + 2
        }
        "C Y" => {
            // draw + scissors
            3 + 3
        }
        "C Z" => {
            // won + rock
            6 + 1
        }
        _ => 0,
    }
}
