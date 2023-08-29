//! Day 3: Perfectly Spherical Houses in a Vacuum

use std::collections::HashSet;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (houses1, houses2) = get_answers(input);

    println!(
        "The number of houses that received at least {} {}.",
        "one present is", houses1
    );
    println!(
        "The number of houses that received at least {} {}.",
        "one present, the next year, is", houses2
    );
}

fn get_answers(input: &str) -> (usize, usize) {
    // Initialize the sets of houses.
    let mut houses1 = HashSet::new();
    let mut houses2 = HashSet::new();

    // Initialize the positions of the Santas.
    let mut santa1: (isize, isize) = (0, 0);
    let mut santa2: (isize, isize) = (0, 0);
    let mut robo_santa: (isize, isize) = (0, 0);

    // Insert the starting positions into the sets.
    houses1.insert(santa1);
    houses2.insert(santa2);

    // Flag to whether it's santa2's turn or robo_santa's turn.
    let mut santa2s_turn = true;

    // Iterate over each character in the input.
    for c in input.chars() {
        match c {
            '^' => santa1.1 += 1,
            'v' => santa1.1 -= 1,
            '>' => santa1.0 += 1,
            '<' => santa1.0 -= 1,
            _ => (),
        }
        houses1.insert(santa1);

        if santa2s_turn {
            match c {
                '^' => santa2.1 += 1,
                'v' => santa2.1 -= 1,
                '>' => santa2.0 += 1,
                '<' => santa2.0 -= 1,
                _ => (),
            }
            houses2.insert(santa2);
        } else {
            match c {
                '^' => robo_santa.1 += 1,
                'v' => robo_santa.1 -= 1,
                '>' => robo_santa.0 += 1,
                '<' => robo_santa.0 -= 1,
                _ => (),
            }
            houses2.insert(robo_santa);
        }

        santa2s_turn = !santa2s_turn;
    }

    (houses1.len(), houses2.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        assert_eq!(get_answers(">").0, 2);
        assert_eq!(get_answers("^>v<").0, 4);
        assert_eq!(get_answers("^v^v^v^v^v").0, 2);
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2() {
        assert_eq!(get_answers("^v").1, 3);
        assert_eq!(get_answers("^>v<").1, 3);
        assert_eq!(get_answers("^v^v^v^v^v").1, 11);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (2565, 2639));
    }
}
