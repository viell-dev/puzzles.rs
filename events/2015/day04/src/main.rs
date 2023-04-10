use md5;

fn get_awnsers(input: &str) -> (i32, i32) {
    let mut five_zeros = 0;
    let mut six_zeros = 0;

    for i in 0.. {
        let hash = md5::compute(format!("{}{}", input, i));
        let hash = format!("{:x}", hash);

        if six_zeros == 0 && hash.starts_with("000000") {
            six_zeros = i;
        }
        if five_zeros == 0 && hash.starts_with("00000") {
            five_zeros = i;
        }

        if six_zeros != 0 && five_zeros != 0 {
            break;
        }
    }

    (five_zeros, six_zeros)
}

fn main() {
    let input = include_str!("input.txt").trim();
    let (five_zeros, six_zeros) = get_awnsers(input);
    println!(
        "Lowest possible number producing MD5 starting with 5 zeros is: {}",
        five_zeros
    );
    println!(
        "Lowest possible number producing MD5 starting with 6 zeros is: {}",
        six_zeros
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(get_awnsers("abcdef").0, 609043);
        assert_eq!(get_awnsers("pqrstuv").0, 1048970);
    }

    #[test]
    fn test_awnsers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_awnsers(input), (346386, 9958218));
    }
}
