use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    time::{Duration, Instant},
};

use input_reader::InputReader;
use multimap::MultiMap;

/// Transformations start from an electron.
const ELECTRON: &str = "e";

/// Seconds before timing out the BFS.
const TIMEOUT: u64 = 60;

// Get line iterator from reading the puzzle input.
fn read_input() -> impl Iterator<Item = String> {
    InputReader::new()
        .with_path("input.txt")
        .read_streaming()
        .expect("Failed to read input")
        .map(|line| line.expect("Failed to read line"))
}

/// Get transformations and target medicine molecule
fn parse_input(lines: impl Iterator<Item = String>) -> (MultiMap<String, String>, String) {
    // Create multi map for storing transformations.
    let mut transformations = MultiMap::new();

    // For every line containing a transformation.
    for line in lines.filter(|line| !line.is_empty()) {
        // We've hit the molecule if the line doesn't contain a fat arrow.
        if !line.contains(" => ") {
            return (transformations, line);
        }

        // If statement above guarantees that the delimiter exists in the line.
        let (from, to) = line.split_once(" => ").unwrap();

        // Add transformation to multi map.
        transformations.insert(from.to_string(), to.to_string());
    }

    panic!("The molecule was not found in the input")
}

/// Apply transformations to a molecule recursively.
fn apply_transformations<T: AsRef<str>, M: AsRef<str>>(
    transformations: &[(usize, T, T)],
    molecule: M,
) -> String {
    // Convert the molecule to an owned string.
    let mut molecule = molecule.as_ref().to_string();

    match transformations {
        // Short-circuit if there are no transformations to apply.
        [] => molecule,
        // Extract the index, from and to values from the head
        // of the slice and the tail as the new transformations.
        [(index, from, to), transformations @ ..] => {
            let from = from.as_ref();
            let to = to.as_ref();

            if &molecule[*index..index + from.len()] != from {
                panic!(
                    "Invalid transformation encountered:\n\
                    \"{} => {}\" is not a valid transformation at index {} of the molecule\n\
                    Molecule:{}",
                    from, to, index, molecule
                );
            }

            // Perform the transformation.
            molecule.replace_range(*index..index + from.len(), to);

            // Recurse
            apply_transformations(transformations, &molecule)
        }
    }
}

/// Get possible next steps after one transformation.
fn get_possible_next_steps<M: AsRef<str>>(
    transformations: &MultiMap<String, String>,
    molecule: M,
) -> HashSet<String> {
    let molecule = molecule.as_ref();

    // Apply each applicable transformation to molecule and collect the results
    transformations
        .flat_iter()
        .flat_map(|(from, to)| {
            molecule
                .match_indices(from)
                .map(move |(index, _)| apply_transformations(&[(index, from, to)], molecule))
        })
        .collect()
}

/// Find the required `transformations` to go from `ELECTRON` to `target_molecule`.
fn transformations_to_target<M: AsRef<str>>(
    transformations: &MultiMap<String, String>,
    target_molecule: M,
) -> Vec<(usize, String, String)> {
    let target_molecule = target_molecule.as_ref();

    // Create reverse mappings for working backwards
    let mut reverse_transformations = Vec::new();
    for (from, to) in transformations.flat_iter() {
        reverse_transformations.push((to.clone(), from.clone()));
    }

    // Sort by length to prefer longer matches first (can significantly reduce search space)
    reverse_transformations.sort_by(|(a, _), (b, _)| b.len().cmp(&a.len()));

    // Create BFS heap (Breadth-First Search).
    // Init the heap with the target molecule.
    let mut heap = BinaryHeap::from([Reverse((
        target_molecule.len(), // Longer strings will have lower priority
        target_molecule.to_string(),
        Vec::<(usize, String, String)>::new(),
    ))]);

    // Create hash set to track seen values.
    // Init the hash set with the target molecule.
    let mut seen = HashSet::from([target_molecule.to_string()]);

    // Start a timeout.
    let start_time = Instant::now();
    let timeout = Duration::from_secs(TIMEOUT);

    // Pop items from the heap.
    while let Some(Reverse((_, current_molecule, current_path))) = heap.pop() {
        if start_time.elapsed() > timeout {
            panic!("Timeout of {} sec reached", TIMEOUT);
        }

        if current_molecule == ELECTRON {
            // Reverse the path and return it.
            let mut reverse_path = current_path;
            reverse_path.reverse();
            return reverse_path;
        }

        // Try each possible reduction
        for (to, from) in &reverse_transformations {
            let mut start_index = 0;

            // While transformations can be applied
            while let Some(index) = current_molecule.clone()[start_index..].find(to) {
                // Get the actual index
                let actual_index = start_index + index;

                // Apply the reverse transformation
                let new_molecule =
                    apply_transformations(&[(actual_index, to, from)], current_molecule.clone());

                // If the molecule hasn't been seen before add it and the path to the heap.
                if !seen.contains(&new_molecule) {
                    seen.insert(new_molecule.clone());
                    let mut new_path = current_path.clone();
                    new_path.push((actual_index, from.clone(), to.clone()));

                    heap.push(Reverse((new_molecule.len(), new_molecule, new_path)));
                }

                // Update start index so we don't loop the same sequence multiple times.
                start_index = actual_index + 1;
            }
        }
    }

    panic!("Failed to find path");
}

fn main() {
    let input = read_input();
    let (transformations, molecule) = parse_input(input);

    let next_steps = get_possible_next_steps(&transformations, &molecule);
    println!("Next steps: {}", next_steps.len());

    let steps = transformations_to_target(&transformations, &molecule);
    println!("Fewest number of steps: {}", steps.len());
}

#[cfg(test)]
mod tests {
    use multimap::multimap;

    use super::*;

    #[test]
    fn test_parse_input() {
        // Example input.
        let input = vec![
            "e => H".to_string(),
            "e => O".to_string(),
            "H => HO".to_string(),
            "H => OH".to_string(),
            "O => HH".to_string(),
            "".to_string(),
            "HOH".to_string(),
        ];

        // Parse input into transformations and molecule.
        let (transformations, molecule) = parse_input(input.into_iter());

        // Define target transformations and molecule.
        let target_transformations = multimap!(
            "e".to_string() => "H".to_string(),
            "e".to_string() => "O".to_string(),
            "H".to_string() => "HO".to_string(),
            "H".to_string() => "OH".to_string(),
            "O".to_string() => "HH".to_string(),
        );
        let target_molecule = "HOH".to_string();

        for (from, to) in transformations.flat_iter() {
            let in_target = target_transformations.contains_key(from)
                && target_transformations.get_vec(from).unwrap().contains(to);

            assert!(in_target, "\"{} => {}\" was not found in target", from, to);
        }

        assert_eq!(molecule, target_molecule);
    }

    #[test]
    fn test_apply_transformations() {
        let transformations = vec![
            (0, "e".to_string(), "O".to_string()),
            (0, "O".to_string(), "HH".to_string()),
            (1, "H".to_string(), "OH".to_string()),
        ];

        let molecule = apply_transformations(&transformations, ELECTRON);

        assert_eq!(molecule, "HOH");
    }

    #[test]
    fn test_get_possible_next_steps() {
        let transformations = multimap!(
            "H".to_string() => "HO".to_string(),
            "H".to_string() => "OH".to_string(),
            "O".to_string() => "HH".to_string(),
        );

        let molecule = "HOH";

        let next_steps = get_possible_next_steps(&transformations, molecule);

        assert_eq!(next_steps.len(), 4);
        assert_eq!(
            next_steps,
            // spell-checker: disable-next-line
            vec!["HOOH", "HOHO", "OHOH", "HHHH"]
                .into_iter()
                .map(String::from)
                .collect::<HashSet<_>>()
        );

        // spell-checker: disable-next-line
        let molecule = "HOHOHO";

        let next_steps = get_possible_next_steps(&transformations, molecule);

        assert_eq!(next_steps.len(), 7);
    }

    #[test]
    fn test_transformations_to_target() {
        let transformations = multimap!(
            "e".to_string() => "H".to_string(),
            "e".to_string() => "O".to_string(),
            "H".to_string() => "HO".to_string(),
            "H".to_string() => "OH".to_string(),
            "O".to_string() => "HH".to_string(),
        );

        let result = transformations_to_target(&transformations, "HOH");

        assert_eq!(result.len(), 3);

        let expected = vec![
            (0, "e".to_string(), "O".to_string()),
            (0, "O".to_string(), "HH".to_string()),
            (0, "H".to_string(), "HO".to_string()),
        ];

        assert_eq!(result, expected);
    }
}
