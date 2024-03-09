//! Day 4: The Ideal Stocking Stuffer

use md5;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (five_zeros, six_zeros) = get_answers(input);

    println!(
        "Lowest possible number resulting in an MD5 hash {}: {}",
        "starting with five zeros is", five_zeros
    );
    println!(
        "Lowest possible number resulting in an MD5 hash {}: {}",
        "starting with six zeros is", six_zeros
    );
}

fn get_answers(input: &str) -> (usize, usize) {
    // Initialize variables to hold the answers.
    let mut five_zeros = 0;
    let mut six_zeros = 0;

    // Loop through all numbers starting at 0.
    for i in 0.. {
        let hash = md5::compute(format!("{}{}", input, i));
        let hash = format!("{:x}", hash);

        /* Check if the hash starts with six zeros
        and six_zeros hasn't been set. */
        if hash.starts_with("000000") && six_zeros == 0 {
            // Set six_zeros to i.
            six_zeros = i;

            if five_zeros == 0 {
                // Set five_zeros to i.
                five_zeros = i;
            }

            // Break since both five_zeros and six_zeros have been set.
            break;
        }
        /* Check if the hash starts with five zeros
        and five_zeros hasn't been set. */
        else if hash.starts_with("00000") && five_zeros == 0 {
            // Set five_zeros to i.
            five_zeros = i;

            // Break if both five_zeros and six_zeros have been set.
            if six_zeros != 0 {
                break;
            }
        }
    }

    // Return the answers.
    (five_zeros, six_zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(get_answers("abcdef").0, 609043);
        assert_eq!(get_answers("pqrstuv").0, 1048970);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (346386, 9958218));
    }
}
