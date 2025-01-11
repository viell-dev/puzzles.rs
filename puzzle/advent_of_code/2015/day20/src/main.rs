use input_reader::InputReader;

/// Read input and parse it into a u32.
fn read_input() -> u32 {
    InputReader::new()
        .with_path("./input.txt")
        .read()
        .expect("Error reading input")
        .first()
        .expect("Input was empty")
        .trim()
        .parse()
        .expect("Error parsing int")
}
/// This function calculates the sum of all divisors of a given number n.
/// It uses the prime factorization method to efficiently compute the sum.
/// The function can also apply additional constraints based on the problem requirements.
fn sum_of_divisors(n: u32, extra_constraints: bool) -> u32 {
    let original_n = n; // Store the original value of n for constraint checks
    let mut divisors = vec![1]; // Initialize the divisors list with 1
    let mut n = n; // Mutable copy of n for factorization
    let mut p = 2; // Start checking for prime factors from 2

    // Find prime factors and build divisors list
    while p * p <= n {
        if n % p == 0 {
            let mut new_divisors = Vec::new(); // Temporary storage for new divisors
            let mut power = p; // Current power of the prime factor

            while n % p == 0 {
                for &d in &divisors {
                    new_divisors.push(d * power); // Generate new divisors
                }

                power *= p; // Increase the power of the prime factor
                n /= p; // Reduce n by the prime factor
            }

            divisors.extend(new_divisors); // Add new divisors to the list
        }

        p += if p == 2 { 1 } else { 2 }; // Move to the next potential prime factor
    }

    // Handle remaining prime factor
    if n > 1 {
        let mut new_divisors = Vec::new();

        for &d in &divisors {
            new_divisors.push(d * n); // Include the last prime factor
        }

        divisors.extend(new_divisors); // Add the last prime factor's divisors
    }

    // Apply constraints if needed
    if extra_constraints {
        // Sum only divisors that satisfy the 50-house rule
        divisors.into_iter().filter(|&d| original_n <= d * 50).sum()
    } else {
        divisors.into_iter().sum() // Return the sum of all divisors
    }
}

/// My original strait-forward unoptimized approach. (took forever)
#[cfg(test)]
fn get_10_presents_iterator_sum() -> impl Iterator<Item = (u32, u32)> {
    // Every house
    (1..).map(|house| {
        // Every elf
        let sum = (1..=house)
            // That visits the house
            .filter(|elf| house % elf == 0)
            // Gives 10 presents.
            .map(|elf| elf * 10)
            // Sum the presents.
            .sum();

        // Return house number and sum of presents.
        (house, sum)
    })
}

/// Better approach after figuring out it was a prime factorization problem
/// or more specifically a `sigma_1(n)` (sum of divisors) problem.
fn get_10_presents_iterator() -> impl Iterator<Item = (u32, u32)> {
    (1..).map(|house| (house, sum_of_divisors(house, false) * 10))
}

/// Again but with 11 presents and extra constraints.
fn get_11_presents_iterator() -> impl Iterator<Item = (u32, u32)> {
    (1..).map(|house| (house, sum_of_divisors(house, true) * 11))
}

fn main() {
    let input = read_input();

    let (lowest_10, _) = get_10_presents_iterator()
        .find(|(_, presents)| presents >= &input)
        .unwrap();
    println!("Lowest house number: {} (10 presents per elf)", lowest_10);

    let (lowest_11, _) = get_11_presents_iterator()
        .find(|(_, presents)| presents >= &input)
        .unwrap();
    println!("Lowest house number: {} (11 presents per elf)", lowest_11);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_divisors() {
        // Test cases without extra constraints
        assert_eq!(sum_of_divisors(1, false), 1); // 1
        assert_eq!(sum_of_divisors(2, false), 3); // 1 + 2
        assert_eq!(sum_of_divisors(3, false), 4); // 1 + 3
        assert_eq!(sum_of_divisors(4, false), 7); // 1 + 2 + 4
        assert_eq!(sum_of_divisors(6, false), 12); // 1 + 2 + 3 + 6
        assert_eq!(sum_of_divisors(8, false), 15); // 1 + 2 + 4 + 8

        // Test cases with extra constraints
        assert_eq!(sum_of_divisors(1, true), 1); // 1
        assert_eq!(sum_of_divisors(2, true), 3); // 1 + 2
        assert_eq!(sum_of_divisors(3, true), 4); // 1 + 3
        assert_eq!(sum_of_divisors(4, true), 7); // 1 + 2 + 4

        // Test larger numbers where the 50-house rule starts to matter
        assert_eq!(sum_of_divisors(51, true), 71); // All divisors except 1 (51 > 50*1)
        assert_eq!(sum_of_divisors(102, true), 213); // All divisors except 1,2 (102 > 50*2)
        assert_eq!(sum_of_divisors(152, true), 297); // All divisors except 1,2,3 (152 > 50*3)
    }

    #[test]
    fn test_get_10_presents_iterator() {
        let results = get_10_presents_iterator().take(9).collect::<Vec<_>>();
        let expected = vec![
            (1, 10),  // House 1 got 10 presents.
            (2, 30),  // House 2 got 30 presents.
            (3, 40),  // House 3 got 40 presents.
            (4, 70),  // House 4 got 70 presents.
            (5, 60),  // House 5 got 60 presents.
            (6, 120), // House 6 got 120 presents.
            (7, 80),  // House 7 got 80 presents.
            (8, 150), // House 8 got 150 presents.
            (9, 130), // House 9 got 130 presents.
        ];

        assert_eq!(results, expected);

        // Also test the old one
        let results = get_10_presents_iterator_sum().take(9).collect::<Vec<_>>();
        assert_eq!(results, expected);

        // Skip 50 and check these values as well
        let results = get_10_presents_iterator()
            .skip(50)
            .take(9)
            .collect::<Vec<_>>();
        let expected = vec![
            (51, 720),  // House 51 received 720 presents.
            (52, 980),  // House 52 received 980 presents.
            (53, 540),  // House 53 received 540 presents.
            (54, 1200), // House 54 received 1200 presents.
            (55, 720),  // House 55 received 720 presents.
            (56, 1200), // House 56 received 1200 presents.
            (57, 800),  // House 57 received 800 presents.
            (58, 900),  // House 58 received 900 presents.
            (59, 600),  // House 59 received 600 presents.
        ];

        assert_eq!(results, expected);
    }

    #[test]
    fn test_get_11_presents_iterator() {
        let results = get_11_presents_iterator().take(9).collect::<Vec<_>>();
        let expected = vec![
            (1, 11),  // House 1 got 10 presents.
            (2, 33),  // House 2 got 30 presents.
            (3, 44),  // House 3 got 40 presents.
            (4, 77),  // House 4 got 70 presents.
            (5, 66),  // House 5 got 60 presents.
            (6, 132), // House 6 got 120 presents.
            (7, 88),  // House 7 got 80 presents.
            (8, 165), // House 8 got 150 presents.
            (9, 143), // House 9 got 130 presents.
        ];

        assert_eq!(results, expected);

        // Skip 50 and check these values as well
        let results = get_11_presents_iterator()
            .skip(50)
            .take(9)
            .collect::<Vec<_>>();
        let expected = vec![
            (51, 781),  // House 51 received 781 presents.
            (52, 1067), // House 52 received 1067 presents.
            (53, 583),  // House 53 received 583 presents.
            (54, 1309), // House 54 received 1309 presents.
            (55, 781),  // House 55 received 781 presents.
            (56, 1309), // House 56 received 1309 presents.
            (57, 869),  // House 57 received 869 presents.
            (58, 979),  // House 58 received 979 presents.
            (59, 649),  // House 59 received 649 presents.
        ];

        assert_eq!(results, expected);
    }
}
