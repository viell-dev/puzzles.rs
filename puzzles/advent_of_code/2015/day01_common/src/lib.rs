use std::{
    fs::File,
    io::{BufReader, Read},
};

use data_loader::load_data;

#[derive(Debug)]
pub struct Elevator {
    data: BufReader<File>,
    floor: i32,
}

impl Elevator {
    pub fn new() -> Self {
        Self::default()
    }

    fn go_up(&mut self) {
        self.floor += 1;
    }

    fn go_down(&mut self) {
        self.floor -= 1;
    }

    pub fn current_floor(&self) -> i32 {
        self.floor
    }
}

impl Default for Elevator {
    fn default() -> Self {
        Self {
            data: load_data("day01_input").unwrap(),
            floor: 0,
        }
    }
}

impl Iterator for Elevator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.data.by_ref().bytes().next() {
            Some(Ok(b'(')) => self.go_up(),
            Some(Ok(b')')) => self.go_down(),
            _ => return None,
        }
        Some(self.current_floor())
    }
}
