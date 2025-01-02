use input_reader::InputReader;

type Reindeer = (String, u32, u32, u32);

fn parse_input(input: &[String]) -> impl Iterator<Item = Reindeer> + '_ {
    input.iter().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[0].to_string();
        let speed = parts[3].parse().expect("invalid input");
        let fly_time = parts[6].parse().expect("invalid input");
        let rest_time = parts[13].parse().expect("invalid input");

        (name, speed, fly_time, rest_time)
    })
}

fn find_winning_reindeer(
    race_seconds: u32,
    reindeers: impl Iterator<Item = Reindeer>,
) -> (Reindeer, u32) {
    reindeers
        .map(|reindeer| {
            let (name, speed, fly_time, rest_time) = reindeer;
            let cycle_time = fly_time + rest_time;
            let complete_cycles = race_seconds / cycle_time;
            let remaining_seconds = race_seconds % cycle_time;
            let flying_seconds = complete_cycles * fly_time + remaining_seconds.min(fly_time);
            let distance = flying_seconds * speed;
            (name, speed, fly_time, rest_time, distance)
        })
        .max_by_key(|(_, _, _, _, distance)| *distance)
        .map(|(name, speed, fly_time, rest_time, distance)| {
            ((name, speed, fly_time, rest_time), distance)
        })
        .expect("no reindeers found")
}

fn find_highest_scoring_reindeer(
    race_seconds: u32,
    reindeers: impl Iterator<Item = Reindeer>,
) -> (Reindeer, u32) {
    let reindeers: Vec<_> = reindeers.collect();
    let mut scores = vec![0; reindeers.len()];

    for second in 1..=race_seconds {
        let positions: Vec<_> = reindeers
            .iter()
            .map(|(_, speed, fly_time, rest_time)| {
                let cycle_time = fly_time + rest_time;
                let complete_cycles = second / cycle_time;
                let remaining_seconds = second % cycle_time;
                let flying_seconds = complete_cycles * fly_time + remaining_seconds.min(*fly_time);
                flying_seconds * speed
            })
            .collect();

        let max_distance = positions.iter().max().unwrap();

        // Award points to all reindeers tied for the lead
        for (i, &distance) in positions.iter().enumerate() {
            if distance == *max_distance {
                scores[i] += 1;
            }
        }
    }

    let winner_idx = scores
        .iter()
        .enumerate()
        .max_by_key(|(_, &score)| score)
        .map(|(idx, _)| idx)
        .expect("no winner found");

    let winner = reindeers[winner_idx].clone();
    let winning_score = scores[winner_idx];

    (winner, winning_score)
}

fn main() {
    let input_reader = InputReader::new().with_path("./input.txt");
    let input = match input_reader.read() {
        Ok(lines) => lines
            .iter()
            .filter_map(|line| match line.trim() {
                line if !line.is_empty() => Some(line.to_owned()),
                _ => None,
            })
            .collect::<Vec<_>>(),
        Err(error) => panic!("Error reading input: {:#?}", error),
    };

    let reindeers = parse_input(&input);
    let (winner, winning_distance) = find_winning_reindeer(2503, reindeers);

    println!(
        "Winner by distance: {} with {} km",
        winner.0, winning_distance
    );

    let reindeers = parse_input(&input);
    let (winner_points, winning_score) = find_highest_scoring_reindeer(2503, reindeers);

    println!(
        "Winner by points: {} with {} points",
        winner_points.0, winning_score
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.".to_owned(),
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.".to_owned(),
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = get_test_input();
        let reindeers: Vec<Reindeer> = parse_input(&input).collect();

        assert_eq!(reindeers.len(), 2);

        assert_eq!(reindeers[0].0, "Comet");
        assert_eq!(reindeers[0].1, 14);
        assert_eq!(reindeers[0].2, 10);
        assert_eq!(reindeers[0].3, 127);

        assert_eq!(reindeers[1].0, "Dancer");
        assert_eq!(reindeers[1].1, 16);
        assert_eq!(reindeers[1].2, 11);
        assert_eq!(reindeers[1].3, 162);
    }

    #[test]
    fn test_find_winning_reindeer() {
        let input = get_test_input();
        let reindeers = parse_input(&input);

        let (winner, distance) = find_winning_reindeer(1000, reindeers);

        assert_eq!(winner.0, "Comet");
        assert_eq!(distance, 1120);
    }

    #[test]
    fn test_find_highest_scoring_reindeer() {
        let input = get_test_input();
        let reindeers = parse_input(&input);

        let (winner, points) = find_highest_scoring_reindeer(1000, reindeers);

        assert_eq!(winner.0, "Dancer");
        assert_eq!(points, 689);
    }
}
