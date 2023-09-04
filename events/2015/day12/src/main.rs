//! Day 12: JSAbacusFramework.io

use serde_json::Value;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (sum_all, sum_non_red) = get_answers(input);

    println!("The sum of all numbers is: {}", sum_all);
    println!("The sum of all non-red numbers is: {}", sum_non_red);
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (isize, isize) {
    let sum_all = sum_all_numbers(input);
    let sum_non_red = sum_non_red_numbers(input);

    (sum_all, sum_non_red)
}

// Sum all numbers in the string.
fn sum_all_numbers(input: &str) -> isize {
    input
        // Split on all characters that aren't a number or minus sign.
        .split(|c| !char::is_numeric(c) && c != '-')
        // Filter out all empty strings and parse all the numbers.
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                s.parse::<isize>().ok()
            }
        })
        // Sum all the numbers.
        .sum()
}

/// Sum all numbers after purging all objects with the value "red".
fn sum_non_red_numbers(input: &str) -> isize {
    // Parse the string into a JSON value.
    let parsed: Value = serde_json::from_str(input).unwrap();

    // Purge all object containing the value "red".
    let purged = purge_reds(parsed);

    sum_all_numbers(purged.to_string().as_str())
}

/// Recursively clear out all objects with the value "red".
fn purge_reds(value: Value) -> Value {
    match value {
        /* For objects we check if the object contains the string "red".
        If it does then we discard the object and return a null instead.
        Otherwise we run every value of the object through `purge_reds`. */
        Value::Object(map) => map
            .values()
            // Check if there is a value that is `red`.
            .find(|v| v == &&Value::String(String::from("red")))
            .map_or_else(
                // Recurse through map values.
                || {
                    // Clone the map so we can mutate it.
                    let mut map_clone = map.clone();

                    // Run every value in the map through `purge_reds`.
                    map_clone
                        .values_mut()
                        .for_each(|v| *v = purge_reds(v.clone()));

                    // Return the map
                    Value::Object(map_clone)
                },
                // Red was found so return null to discard the map.
                |_| Value::Null,
            ),
        // Arrays just need to run every value through `purge_reds`.
        Value::Array(vec) => Value::Array(
            vec.iter()
                // Recurse through vector values
                .map(|v| purge_reds(v.clone()))
                .collect::<Vec<_>>(),
        ),
        // Other values can be return as-is.
        v => v.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        assert_eq!(sum_all_numbers(r#"[1,2,3]"#), 6);
        assert_eq!(sum_all_numbers(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(sum_all_numbers(r#"[[[3]]]"#), 3);
        assert_eq!(sum_all_numbers(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(sum_all_numbers(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(sum_all_numbers(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(sum_all_numbers(r#"[]"#), 0);
        assert_eq!(sum_all_numbers(r#"{}"#), 0);
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2_examples() {
        assert_eq!(sum_non_red_numbers(r#"[1,2,3]"#), 6);
        assert_eq!(sum_non_red_numbers(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(sum_non_red_numbers(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(sum_non_red_numbers(r#"[1,"red",5]"#), 6);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (156_366, 96_852));
    }
}
