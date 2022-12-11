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

    fn move_some_retain_order(&mut self, many: usize, from: usize, to: usize) {
        let pos = self.stacks[to].len();
        (0..many).for_each(|_| {
            let out = self.stacks[from].pop().unwrap();
            self.stacks[to].insert(pos, out);
        })
    }

    fn print_top(&self) -> String {
        self.stacks.iter().map(|x| x.last()).flatten().collect()
    }
}

fn first(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut phase = Phase::Input;
    let (board_9000, board_9001) = BufReader::new(OpenOptions::new().read(true).open(filename)?)
        .lines()
        .flatten()
        .fold(
            (Board::new(), Board::new()),
            |(mut board_9000, mut board_9001), line| {
                if line == "" {
                    phase = Phase::Action;
                    (board_9000, board_9001)
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
                            board_9000.move_some(many, from, to);
                            board_9001.move_some_retain_order(many, from, to);
                        }
                        Phase::Input => {
                            let mut line = line;
                            line.push(' ');
                            line.chars().array_chunks::<4>().enumerate().for_each(
                                |(index, [bracket, char, _, _])| {
                                    if char != ' ' && bracket == '[' {
                                        board_9000.insert(index, char);
                                        board_9001.insert(index, char);
                                    }
                                },
                            );
                        }
                    }
                    (board_9000, board_9001)
                }
            },
        );
    println!(
        "for file {filename} \n\ttop view is of 9000 is {}\n\ttop view of 9001 is {}",
        board_9000.print_top(),
        board_9001.print_top()
    );
    Ok(())
}

fn main() {
    first("input/05/05.test").unwrap_or(());
    first("input/05/05").unwrap_or(());
}
