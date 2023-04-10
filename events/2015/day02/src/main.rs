#[derive(Debug, PartialEq)]
enum ParseError {
    ParseIntError,
    TooManyParts,
    TooFewParts,
}

fn get_awnsers(input: &str) -> Result<(i32, i32), ParseError> {
    let mut lwh = Vec::<i32>::with_capacity(3);

    for part in input.split("x") {
        if lwh.len() == 3 {
            return Err(ParseError::TooManyParts);
        }

        match part.parse::<i32>() {
            Ok(i) => lwh.push(i),
            Err(_) => return Err(ParseError::ParseIntError),
        }
    }

    if lwh.len() != 3 {
        return Err(ParseError::TooFewParts);
    }

    lwh.sort();

    let (l, w, h) = (lwh[0], lwh[1], lwh[2]);

    let (lw, wh, hl) = (l * w, w * h, h * l);

    let area = 3 * lw + 2 * wh + 2 * hl;

    let ribbon = 2 * l + 2 * w + l * w * h;

    Ok((area, ribbon))
}

fn main() {
    let input = include_str!("input.txt").trim();
    let mut total_area = 0;
    let mut total_ribbon = 0;

    for line in input.lines() {
        match get_awnsers(line) {
            Ok((area, ribbon)) => {
                total_area += area;
                total_ribbon += ribbon;
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }

    println!("Total area: {}", total_area);
    println!("Total ribbon: {}", total_ribbon);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(get_awnsers("2x3x4").unwrap().0, 58);
        assert_eq!(get_awnsers("1x1x10").unwrap().0, 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(get_awnsers("2x3x4").unwrap().1, 34);
        assert_eq!(get_awnsers("1x1x10").unwrap().1, 14);
    }

    #[test]
    fn test_error() {
        assert_eq!(get_awnsers("axbxc").unwrap_err(), ParseError::ParseIntError);
        assert_eq!(
            get_awnsers("2x3x4x5").unwrap_err(),
            ParseError::TooManyParts
        );
        assert_eq!(get_awnsers("2x3").unwrap_err(), ParseError::TooFewParts);
    }

    #[test]
    fn test_awnsers() {
        let input = include_str!("input.txt").trim();
        let mut total_area = 0;
        let mut total_ribbon = 0;

        for line in input.lines() {
            match get_awnsers(line) {
                Ok((area, ribbon)) => {
                    total_area += area;
                    total_ribbon += ribbon;
                }
                Err(e) => println!("Error: {:?}", e),
            }
        }

        assert_eq!((total_area, total_ribbon), (1586300, 3737498));
    }
}
