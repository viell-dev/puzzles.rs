use input_reader::InputReader;
use itertools::Itertools;

const LITERS_OF_EGGNOG: u32 = 150;

fn read_input() -> Vec<u32> {
    // Get containers from input.
    let mut containers = InputReader::new()
        .with_path("./input.txt")
        .read()
        .expect("reading input failed")
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .map(|n| n.parse::<u32>().expect("invalid input"))
        .collect::<Vec<_>>();

    // Sort largest to smallest containers.
    containers.sort_by(|a, b| b.cmp(a));

    containers
}

fn find_combinations(containers: &[u32], target: u32) -> Vec<Vec<u32>> {
    let mut results = Vec::new();

    // Find combinations that sum to target
    for length in 1..=containers.len() {
        for combinations in containers.iter().cloned().combinations(length) {
            if combinations.iter().sum::<u32>() == target {
                results.push(combinations);
            }
        }
    }

    results
}

fn find_combinations_with_least_containers(combinations: &[Vec<u32>]) -> usize {
    // Clone for mutation.
    let mut combinations = combinations.to_vec();

    // Sort results by least to most containers
    combinations.sort_by_key(|combination| combination.len());

    // Count combinations with same length as the first one.
    combinations
        .iter()
        .take_while(|c| c.len() == combinations[0].len())
        .count()
}

fn main() {
    // Get containers from input.
    let containers = read_input();

    // Find combinations
    let combinations = find_combinations(&containers, LITERS_OF_EGGNOG);

    // Find number of combinations with the least amount of containers.
    let least_containers = find_combinations_with_least_containers(&combinations);

    // Print the results:
    println!(
        "Combinations: {}, Smallest combination size: {}",
        combinations.len(),
        least_containers
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_combinations() {
        let target = 25;
        let containers = vec![20, 15, 10, 5, 5];

        let combinations = find_combinations(&containers, target);

        // Verify we got the expected number of combinations
        assert!(!combinations.is_empty());
        assert_eq!(combinations.len(), 4);

        // Verify each combination sums to target
        for combination in &combinations {
            assert_eq!(combination.iter().sum::<u32>(), target);
        }
    }

    #[test]
    fn test_find_combinations_with_least_containers() {
        let combinations = vec![
            vec![15, 10],
            vec![20, 5],    // first 5
            vec![15, 5, 5], // shifted the order to also test sorting.
            vec![20, 5],    // second 5
        ];

        let result = find_combinations_with_least_containers(&combinations);
        assert_eq!(result, 3); // Should find 3 combination with length 2
    }
}
