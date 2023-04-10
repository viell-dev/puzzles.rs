use std::collections::HashSet;

fn get_awnsers(input: &str) -> (i32, i32) {
    let mut houses1 = HashSet::new();
    let mut santa1 = (0, 0);
    houses1.insert(santa1);

    for c in input.chars() {
        match c {
            '^' => santa1.1 += 1,
            'v' => santa1.1 -= 1,
            '>' => santa1.0 += 1,
            '<' => santa1.0 -= 1,
            _ => (),
        }
        houses1.insert(santa1);
    }

    let mut houses2 = HashSet::new();
    let mut santa2 = (0, 0);
    houses2.insert(santa2);
    let mut robo_santa = (0, 0);
    let mut robo_turn = false;

    for c in input.chars() {
        let target = if robo_turn {
            &mut robo_santa
        } else {
            &mut santa2
        };

        match c {
            '^' => target.1 += 1,
            'v' => target.1 -= 1,
            '>' => target.0 += 1,
            '<' => target.0 -= 1,
            _ => (),
        }
        houses2.insert(*target);

        robo_turn = !robo_turn;
    }

    (houses1.len() as i32, houses2.len() as i32)
}

fn main() {
    let input = include_str!("input.txt").trim();
    let (h1, h2) = get_awnsers(input);

    println!("Santa delivers at least one present to {} houses.", h1);
    println!(
        "Santa and Robo-Santa delivers at least one present to {} houses.",
        h2
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(get_awnsers(">").0, 2);
        assert_eq!(get_awnsers("^>v<").0, 4);
        assert_eq!(get_awnsers("^v^v^v^v^v").0, 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(get_awnsers("^v").1, 3);
        assert_eq!(get_awnsers("^>v<").1, 3);
        assert_eq!(get_awnsers("^v^v^v^v^v").1, 11);
    }

    #[test]
    fn test_awnsers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_awnsers(input), (2565, 2639));
    }
}
