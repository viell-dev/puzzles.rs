//! Day 17: No Such Thing as Too Much

use itertools::Itertools;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (combinations, min_combinations) = get_answers(input);

    println!(
        "The number of different combinations of containers {} {}",
        "that can exactly fit all 150 liters of eggnog is:", combinations
    );
    println!(
        "The number of different combinations of the {} {} {}",
        "minimum amount of containers that can exactly", // force line-break
        "fit all 150 liters of eggnog is:",
        min_combinations
    );
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (usize, usize) {
    // Parse the input into a vector of container sizes.
    let container_sizes = parse_input(input);

    // Find combinations of container sizes for 150 liters of eggnog.
    find_combinations(container_sizes, 150)
}

/// Parse the input into a vector of container sizes.
fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

/// Find combinations of container sizes for the specified amount of eggnog.
fn find_combinations(container_sizes: Vec<usize>, liters_of_eggnog: usize) -> (usize, usize) {
    // Initialize number of combinations.
    let mut combinations = 0;
    // Initialize minimum number of containers.
    let mut min_containers = usize::MAX;
    // Initialize number of combinations using the minimum number of containers.
    let mut min_combinations = 0;

    // For every combination of containers...
    for combination in (1..=container_sizes.len())
        .map(|i| container_sizes.iter().combinations(i))
        .flatten()
    {
        // Calculate the sum of the containers.
        let sum = combination.iter().map(|n| **n).sum::<usize>();

        // If the the sum is the same as the liters of eggnog needed...
        if sum == liters_of_eggnog {
            // Increment combinations of containers.
            combinations += 1;

            // Get the number of containers.
            let number_of_containers = combination.len();

            /* If the number of containers is less than the minimum number
            of containers found so far... */
            if number_of_containers < min_containers {
                // Set the new minimum number of containers.
                min_containers = number_of_containers;
                /* Reset the number of combinations using the minimum
                number of containers to 1. */
                min_combinations = 1;
            }
            /* If the number of containers is the same as the minimum number
            of containers found so far... */
            else if number_of_containers == min_containers {
                /* Increment the number of combinations using the minimum
                number of containers. */
                min_combinations += 1;
            }
        }
    }

    /* Return the number of combinations and the number of combinations
    using the minimum number of containers. */
    (combinations, min_combinations)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Get the example input from the puzzle page.
    fn get_example_input() -> &'static str {
        "20\n15\n10\n5\n5"
    }

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        let input = get_example_input();
        let container_sizes = parse_input(input);

        assert_eq!(find_combinations(container_sizes, 25).0, 4);
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2_examples() {
        let input = get_example_input();
        let container_sizes = parse_input(input);

        assert_eq!(find_combinations(container_sizes, 25).1, 3);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (1638, 17));
    }
}
