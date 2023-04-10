fn get_awnsers(input: &str) -> (i32, i32) {
    let mut floor: i32 = 0;
    let mut basement: i32 = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if basement == 0 && floor == -1 {
            basement = i as i32 + 1;
        }
    }

    (floor, basement)
}

fn main() {
    let input = include_str!("input.txt").trim();

    let (floor, basement) = get_awnsers(input);

    if basement == 0 {
        println!("Santa never entered the basement");
    } else {
        println!("Santa first entered the basement at position {}", basement);
    }

    println!("Santa ended up on floor {}", floor);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(get_awnsers("(())").0, 0);
        assert_eq!(get_awnsers("()()").0, 0);
        assert_eq!(get_awnsers("(((").0, 3);
        assert_eq!(get_awnsers("(()(()(").0, 3);
        assert_eq!(get_awnsers("))(((((").0, 3);
        assert_eq!(get_awnsers("())").0, -1);
        assert_eq!(get_awnsers("))(").0, -1);
        assert_eq!(get_awnsers(")))").0, -3);
        assert_eq!(get_awnsers(")())())").0, -3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(get_awnsers(")").1, 1);
        assert_eq!(get_awnsers("()())").1, 5);
    }

    #[test]
    fn test_awnsers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_awnsers(input), (232, 1783));
    }
}
