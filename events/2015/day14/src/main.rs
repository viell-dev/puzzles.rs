//! Day 14: Reindeer Olympics

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (max_distance, max_points) = get_answers(input);

    println!(
        "The distance that the winning reindeer traveled is: {}",
        max_distance
    );
    println!(
        "The number of points that the winning reindeer has: {}",
        max_points
    );
}

/// Number of seconds a race lasts.
const SECONDS: usize = 2503;

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (usize, usize) {
    // Initialize a vector of reindeer.
    let mut reindeer = Vec::with_capacity(input.lines().count());

    // Parse each line of the input into a reindeer.
    for line in input.lines() {
        reindeer.push(Reindeer::new(line));
    }

    // Get the max distance traveled by any reindeer.
    let max_distance = reindeer
        .iter()
        .map(|r| r.clone().nth(SECONDS - 1).unwrap())
        .max_by(|a, b| a.cmp(&b))
        .unwrap();

    // Get the max points earned by any reindeer.
    let max_points = get_max_points(&mut reindeer, SECONDS);

    // Return the answers.
    (max_distance, max_points)
}

/// A reindeer.
#[derive(Clone)]
struct Reindeer {
    speed: usize,
    fly_time: usize,
    rest_time: usize,
    seconds_passed: usize,
    distance_flown: usize,
    points: usize,
}

impl Reindeer {
    /// Create a new reindeer from a line of input.
    fn new(line: &str) -> Self {
        if let [_, "can", "fly", speed, "km/s", "for", fly_time, "seconds,", "but", "then", "must", "rest", "for", rest_time, "seconds."] =
            line.split_whitespace().collect::<Vec<_>>().as_slice()
        {
            Self {
                speed: speed.parse::<usize>().unwrap(),
                fly_time: fly_time.parse::<usize>().unwrap(),
                rest_time: rest_time.parse::<usize>().unwrap(),
                seconds_passed: 0,
                distance_flown: 0,
                points: 0,
            }
        } else {
            panic!("Invalid input: {}", line);
        }
    }
}

impl Iterator for Reindeer {
    type Item = usize;

    /// Get the distance traveled by the reindeer in the next second.
    fn next(&mut self) -> Option<Self::Item> {
        if self.seconds_passed % (self.fly_time + self.rest_time) < self.fly_time {
            self.distance_flown += self.speed;
        }

        self.seconds_passed += 1;
        Some(self.distance_flown)
    }
}

/// Get the max points earned by any reindeer.
fn get_max_points(reindeer: &mut Vec<Reindeer>, seconds: usize) -> usize {
    // For each second that the race lasts...
    for _ in 0..seconds {
        // Get the current max distance traveled by any reindeer.
        let max_distance = reindeer
            .iter_mut()
            .map(|r| r.next().unwrap())
            .max_by(|a, b| a.cmp(&b))
            .unwrap();

        // Give a point to each reindeer that is in the lead.
        for r in reindeer.iter_mut() {
            if r.distance_flown == max_distance {
                r.points += 1;
            }
        }
    }

    // Return the max points earned by any reindeer.
    reindeer
        .iter()
        .map(|r| r.points)
        .max_by(|a, b| a.cmp(&b))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        r#"
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
"#
        .trim()
    }

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        let input = get_example_input();

        let mut reindeer = Vec::with_capacity(input.lines().count());
        for line in input.lines() {
            reindeer.push(Reindeer::new(line));
        }

        let sec_1 = reindeer
            .iter_mut()
            .map(|r| r.next().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(sec_1, vec![14, 16]);

        let sec_10 = reindeer
            .iter_mut()
            .map(|r| r.nth(10 - 1 - 1).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(sec_10, vec![140, 160]);

        let sec_11 = reindeer
            .iter_mut()
            .map(|r| r.next().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(sec_11, vec![140, 176]);

        let sec_12 = reindeer
            .iter_mut()
            .map(|r| r.next().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(sec_12, vec![140, 176]);

        let sec_1000 = reindeer
            .iter_mut()
            .map(|r| r.nth(1000 - 12 - 1).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(sec_1000, vec![1120, 1056]);
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2_examples() {
        let input = get_example_input();

        let mut reindeer = Vec::with_capacity(input.lines().count());
        for line in input.lines() {
            reindeer.push(Reindeer::new(line));
        }

        let max_points = get_max_points(&mut reindeer, 1000);
        assert_eq!(max_points, 689);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (2660, 1256));
    }
}
