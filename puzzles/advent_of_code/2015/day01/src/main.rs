use derive_more::{Deref, DerefMut, Display};
use input_reader::{Input, Outcome, read_input};
use std::collections::VecDeque;

fn main() {
    let input = match read_input().expect("failed to read input") {
        Outcome::Exit => return,
        Outcome::Continue(input) => input,
    };

    let directions = parse_input(input);

    let final_floor = solve_part1(directions.clone());
    println!("Part 1 solution: {final_floor}");

    let first_basement_position = solve_part2(directions);
    match first_basement_position {
        Some(pos) => println!("Part 2 solution: {pos}"),
        None => println!("Part 2 solution: Not found"),
    }
}

// -----------------------------------------------------------------------------
// Input Parsing
// -----------------------------------------------------------------------------

fn parse_input(input: Input) -> Directions {
    input
        .chars()
        .filter_map(|c| match c.expect("failed to read char") {
            '(' => Some(Direction::Up),
            ')' => Some(Direction::Down),
            _ => None, // ignore unknown chars
        })
        .collect::<VecDeque<_>>()
        .into()
}

// -----------------------------------------------------------------------------
// Solutions
// -----------------------------------------------------------------------------

fn solve_part1(directions: Directions) -> Floor {
    Santa::new(directions).last().unwrap_or_default()
}

fn solve_part2(directions: Directions) -> Option<usize> {
    Santa::new(directions)
        .position(Floor::is_basement)
        .map(|v| v.saturating_add(1))
}

// -----------------------------------------------------------------------------
// Internals
// -----------------------------------------------------------------------------

/// Directions given to Santa
#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Eq)]
struct Directions(VecDeque<Direction>); // VecDeque for FIFO

impl<T> From<T> for Directions
where
    VecDeque<Direction>: From<T>,
{
    fn from(value: T) -> Self {
        Directions(VecDeque::from(value))
    }
}

/// Direction Santa can be told to move in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

/// A floor of the apartment building Santa is delivering presents to.
#[derive(Clone, Copy, Debug, Default, Display, PartialEq)]
struct Floor(#[display] i32);

impl Floor {
    pub fn is_basement(self) -> bool {
        self.0 < 0
    }
}

/// Santa
#[derive(Debug)]
struct Santa {
    current_floor: Floor,
    directions: Directions,
}

impl Santa {
    fn new(directions: Directions) -> Self {
        Self {
            current_floor: Floor::default(),
            directions,
        }
    }

    fn go_up_one_floor(&mut self) {
        self.current_floor = Floor(self.current_floor.0.saturating_add(1));
    }

    fn go_down_one_floor(&mut self) {
        self.current_floor = Floor(self.current_floor.0.saturating_sub(1));
    }
}

impl Iterator for Santa {
    type Item = Floor;

    fn next(&mut self) -> Option<Self::Item> {
        let next_direction = self.directions.pop_front()?;

        match next_direction {
            Direction::Up => self.go_up_one_floor(),
            Direction::Down => self.go_down_one_floor(),
        }

        Some(self.current_floor)
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! directions {
        ($($variant:ident),+ $(,)?) => {
            Directions::from(vec![$(Direction::$variant),+])
        };
    }

    struct TestData {
        input: Input,
        parsed: Directions,
        part1: Floor,
        part2: Option<usize>,
    }

    fn get_test_data() -> Vec<TestData> {
        vec![
            // AoC examples:
            TestData {
                input: Input::from("(())"),
                parsed: directions!(Up, Up, Down, Down),
                part1: Floor(0),
                part2: None,
            },
            TestData {
                input: Input::from("()()"),
                parsed: directions!(Up, Down, Up, Down),
                part1: Floor(0),
                part2: None,
            },
            TestData {
                input: Input::from("((("),
                parsed: directions!(Up, Up, Up),
                part1: Floor(3),
                part2: None,
            },
            TestData {
                input: Input::from("(()(()("),
                parsed: directions!(Up, Up, Down, Up, Up, Down, Up),
                part1: Floor(3),
                part2: None,
            },
            TestData {
                input: Input::from("))((((("),
                parsed: directions!(Down, Down, Up, Up, Up, Up, Up),
                part1: Floor(3),
                part2: Some(1),
            },
            TestData {
                input: Input::from("())"),
                parsed: directions!(Up, Down, Down),
                part1: Floor(-1),
                part2: Some(3),
            },
            TestData {
                input: Input::from("))("),
                parsed: directions!(Down, Down, Up),
                part1: Floor(-1),
                part2: Some(1),
            },
            TestData {
                input: Input::from(")))"),
                parsed: directions!(Down, Down, Down),
                part1: Floor(-3),
                part2: Some(1),
            },
            TestData {
                input: Input::from(")())())"),
                parsed: directions!(Down, Up, Down, Down, Up, Down, Down),
                part1: Floor(-3),
                part2: Some(1),
            },
            // Edge cases:
            TestData {
                input: Input::from("test"), // invalid only
                parsed: Directions::from([]),
                part1: Floor(0),
                part2: None,
            },
            TestData {
                input: Input::from("(()test(()"), // some invalid
                parsed: directions!(Up, Up, Down, Up, Up, Down),
                part1: Floor(2),
                part2: None,
            },
        ]
    }

    #[test]
    fn test_parse_input() {
        for data in get_test_data() {
            assert_eq!(parse_input(data.input), data.parsed);
        }
    }

    #[test]
    fn test_solve_part1() {
        for data in get_test_data() {
            assert_eq!(solve_part1(data.parsed), data.part1);
        }
    }

    #[test]
    fn test_solve_part2() {
        for data in get_test_data() {
            assert_eq!(solve_part2(data.parsed), data.part2);
        }
    }
}
