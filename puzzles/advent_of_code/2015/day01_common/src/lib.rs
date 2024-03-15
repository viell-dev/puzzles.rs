use data_loader::load_data;
use std::{
    fs::File,
    io::{BufReader, Read},
};

/// Enum to represent the direction the elevator should go
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// Struct to represent the elevator
#[derive(Debug)]
pub struct Elevator {
    /// Whether the program is done
    done: bool,
    /// The program to run
    program: BufReader<File>,
    /// The current floor
    floor: i32,
}

impl Elevator {
    /// Create a new elevator
    pub fn new() -> Self {
        // Create a new elevator with the default program
        Self::default()
    }

    /// Get the next instruction from the program
    fn get_next_instruction(&mut self) -> Option<Direction> {
        // If we are done, return None
        if self.done {
            return None;
        }

        // Get the next byte from the program and match it to a direction
        match self.program.by_ref().bytes().next() {
            Some(Ok(b'(')) => Some(Direction::Up),
            Some(Ok(b')')) => Some(Direction::Down),
            _ => None,
        }
    }

    /// Go up one floor
    fn go_up(&mut self) {
        self.floor += 1;
    }

    /// Go down one floor
    fn go_down(&mut self) {
        self.floor -= 1;
    }

    /// Get the current floor
    pub fn current_floor(&self) -> i32 {
        self.floor
    }
}

impl Default for Elevator {
    /// Create a new elevator with the default program
    fn default() -> Self {
        Self {
            done: false,
            program: load_data("day01_input").unwrap(),
            floor: 0,
        }
    }
}

impl Iterator for Elevator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(direction) = self.get_next_instruction() {
            match direction {
                Direction::Up => self.go_up(),
                Direction::Down => self.go_down(),
            }

            Some(self.current_floor())
        } else {
            self.done = true;
            None
        }
    }
}
