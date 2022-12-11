#![allow(dead_code)]
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

enum DirEntry {
    File(FileEntry),
    Directory(Directory),
}

struct FileEntry {
    name: String,
    size: usize,
}

struct Directory {
    name: String,
    entries: HashMap<String, DirEntry>,
    size: usize,
}

impl Directory {
    fn new() -> Self {
        Directory {
            name: "/".to_string(),
            entries: Default::default(),
            size: 0,
        }
    }

    fn add_file_entry(&mut self, name: String, size: usize) {
        if !self.entries.contains_key(&name) {
            let entry = DirEntry::File(FileEntry {
                name: name.clone(),
                size: size,
            });
            self.entries.insert(name, entry);
        }
    }

    fn add_directory(&mut self, directory: String) {
        if !self.entries.contains_key(&directory) {
            let entry = DirEntry::Directory(Directory {
                name: directory.clone(),
                entries: Default::default(),
                size: 0,
            });
            self.entries.insert(directory, entry);
        }
    }

    fn get_mut_directory(&mut self, directory_path: &Vec<String>) -> &mut Self {
        let mut ret = self;
        for path in directory_path {
            match ret.entries.get_mut(path) {
                Some(DirEntry::Directory(ref mut dir)) => ret = dir,
                _ => panic!("directory must be created {path}"),
            }
        }
        ret
    }
}

impl Directory {
    #[allow(clippy::never)]
    fn print_structure(&self, prefix: &str) {
        let current_full_path = format!("{prefix}/{}", self.name);
        println!("{current_full_path} dir size= {}", self.size);
        for (_name, entry) in &self.entries {
            match entry {
                DirEntry::File(file) => {
                    println!("{current_full_path}/{_name} file size= {}", file.size);
                }
                DirEntry::Directory(dir) => {
                    // println!("{current_full_path}/{_name} dir size= {}", dir.size);
                    dir.print_structure(&current_full_path);
                }
            }
        }
    }

    fn compute_size(&mut self) -> usize {
        let size = self
            .entries
            .iter_mut()
            .map(|(_x, entry)| match entry {
                DirEntry::File(file) => file.size,
                DirEntry::Directory(ref mut dir) => dir.compute_size(),
            })
            .sum();
        self.size = size;
        size
    }

    fn list_dir_sizes(&self) -> Vec<usize> {
        self.entries
            .iter()
            .flat_map(|(_x, entry)| match entry {
                DirEntry::File(_file) => vec![],
                DirEntry::Directory(dir) => {
                    let mut sub = dir.list_dir_sizes();
                    sub.push(dir.size);
                    sub
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ParseError")
    }
}

impl Error for ParseError {}

fn first(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut root_directory = Directory::new();
    let mut pointer = &mut root_directory;
    let mut current_dir = vec![];
    for line in BufReader::new(OpenOptions::new().read(true).open(filename)?)
        .lines()
        .flatten()
    {
        if line.starts_with("$ cd") {
            let new_dir = line[5..].to_string();
            if new_dir == "/" {
                pointer = &mut root_directory;
                continue;
            } else if new_dir == ".." {
                current_dir.pop();
                pointer = root_directory.get_mut_directory(&current_dir);
            } else {
                current_dir.push(new_dir);
                pointer = root_directory.get_mut_directory(&current_dir);
            }
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir") {
            let new_dir = line[4..].to_string();
            pointer.add_directory(new_dir.to_string());
        } else {
            let mut line = line.split(" ");
            let size = line.next().map(str::parse::<usize>).ok_or(ParseError)??;
            let name = line.next().ok_or(ParseError)?;
            pointer.add_file_entry(name.to_string(), size);
        }
    }
    root_directory.compute_size();
    let total_size_with_dir_size_le_1_mbsize: usize = root_directory
        .list_dir_sizes()
        .iter()
        .filter(|x| *x < &100000)
        .sum();
    println!("for file {filename}");
    println!("\t [part 1] max 100000 dir sum size is {total_size_with_dir_size_le_1_mbsize}");

    let in_need = root_directory.size - 40000000;
    let to_delete: usize = root_directory
        .list_dir_sizes()
        .iter()
        .filter(|x| *x >= &in_need)
        .min()
        .copied()
        .unwrap();
    println!("\t [part 2] directory to delete is {to_delete:?}\n\n");

    Ok(())
}

fn main() {
    first("input/07/07.test").unwrap_or(());
    first("input/07/07").unwrap_or(());
}
