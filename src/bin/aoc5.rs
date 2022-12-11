#![feature(iter_array_chunks)]
use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

enum Phase {
    Input,
    Action,
}

#[derive(Debug)]
struct Board {
    stacks: Vec<Vec<char>>,
}

impl Board {
    fn new() -> Self {
        Board { stacks: vec![] }
    }
    fn insert(&mut self, index: usize, contain: char) {
        match self.stacks.get_mut(index) {
            Some(stack) => {
                stack.insert(0, contain);
            }
            None => {
                (self.stacks.len()..index).for_each(|_| self.stacks.push(vec![]));
                self.stacks.push(vec![contain])
            }
        }
    }

    fn move_some(&mut self, many: usize, from: usize, to: usize) {
        (0..many).for_each(|_| {
            let out = self.stacks[from].pop().unwrap();
            self.stacks[to].push(out);
        })
    }

    fn print_top(&self) -> String {
        self.stacks.iter().map(|x| x.last()).flatten().collect()
    }
}

fn first(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut phase = Phase::Input;
    let board = BufReader::new(OpenOptions::new().read(true).open(filename)?)
        .lines()
        .flatten()
        .fold(Board::new(), |mut board, line| {
            if line == "" {
                phase = Phase::Action;
                board
            } else {
                match phase {
                    Phase::Action => {
                        let action_data: Vec<usize> = line
                            .split(' ')
                            .skip(1)
                            .step_by(2)
                            .map(str::parse)
                            .flatten()
                            .collect();
                        let (many, from, to) =
                            (action_data[0], action_data[1] - 1, action_data[2] - 1);
                        board.move_some(many, from, to);
                    }
                    Phase::Input => {
                        let mut line = line;
                        line.push(' ');
                        line.chars().array_chunks::<4>().enumerate().for_each(
                            |(index, [bracket, char, _, _])| {
                                if char != ' ' && bracket == '[' {
                                    board.insert(index, char);
                                }
                            },
                        );
                    }
                }
                board
            }
        });
    println!("for file {filename} top view is {:?}", board.print_top());
    Ok(())
}

fn main() {
    first("input/05/05.test").unwrap_or(());
    first("input/05/05").unwrap_or(());
}
