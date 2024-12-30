use input_reader::InputReader;
use md5::{Digest, Md5};

fn parse_input(input: &str) -> impl Iterator<Item = String> + Clone + '_ {
    (1..)
        .map(move |number| format!("{}{}", input, number))
        .map(|value| {
            let mut hasher = Md5::new();
            hasher.update(value);
            format!("{:x}", hasher.finalize())
        })
}

fn five_zeros(mut iter: impl Iterator<Item = String>) -> usize {
    iter.position(|value| value.starts_with("00000"))
        .expect("Value not found")
        + 1
}

fn six_zeros(mut iter: impl Iterator<Item = String>) -> usize {
    iter.position(|value| value.starts_with("000000"))
        .expect("Value not found")
        + 1
}

fn main() {
    let input_reader = InputReader::new().with_path("./input.txt");
    let input = match input_reader.read() {
        Ok(lines) => match lines.first() {
            Some(line) if !line.trim().is_empty() => line.trim().to_owned(),
            _ => panic!("Error reading input: Input was empty"),
        },
        Err(error) => panic!("Error reading input: {:#?}", error),
    };

    let values = parse_input(&input);
    let five_zeros = five_zeros(values.clone());
    let six_zeros = six_zeros(values);

    println!("Five zeros: {}, Six zeros: {}", five_zeros, six_zeros);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        //spell-checker: disable
        assert_eq!(
            parse_input("abcdef").skip(100).take(10).collect::<Vec<_>>(),
            vec![
                "cb872a1689d6ae9991a1f5bc63161cf5".to_owned(),
                "2bf2ac69acb234d24fa9a353b54d0573".to_owned(),
                "ffd9e8c5f225ee4e1238cf73319d1f15".to_owned(),
                "0ae020a610ee9578a18586c03bee0477".to_owned(),
                "2f4dce1f9ca1fdeb22cfe7bb8cf623f3".to_owned(),
                "0d1292c17fc818e6f51b6384163a2140".to_owned(),
                "42d893c546ce0867ee5eee889e44988a".to_owned(),
                "dc68c3d4ce4352f335b2404affae31ed".to_owned(),
                "bdb0d7f77df3bff034e649d43a4bf717".to_owned(),
                "cc94c27abde6381a92f28b527698bef4".to_owned()
            ]
        );
        assert_eq!(
            parse_input("pqrstuv")
                .skip(100)
                .take(10)
                .collect::<Vec<_>>(),
            vec![
                "eabc9fe1a89c6ffa68b9c2aeca87d4d4".to_owned(),
                "9752aee83134a2b3fc43ee47986ec4b8".to_owned(),
                "c6ec5480934e8d10692b408745fc8423".to_owned(),
                "d90000914fc0a8b80f865ae1abf64e43".to_owned(),
                "e064f6ac19968f673390abda9577e59c".to_owned(),
                "a4f489526ed6e6bd40f9465fbedb1543".to_owned(),
                "9b69a8390013b8ea0df26dc936a2b4a2".to_owned(),
                "32ad02931c9ec7069f2ba83c0dd45fb0".to_owned(),
                "7c3ab492fa7890a846dfd2c840fa1598".to_owned(),
                "5fa38374791139656281bd383979d658".to_owned()
            ]
        );
        //spell-checker: enable
    }

    #[test]
    fn test_five_zeros() {
        //spell-checker: disable
        assert_eq!(five_zeros(parse_input("abcdef")), 609043);
        assert_eq!(five_zeros(parse_input("pqrstuv")), 1048970);
        //spell-checker: enable
    }

    #[test]
    fn test_six_zeros() {
        //spell-checker: disable
        assert_eq!(six_zeros(parse_input("abcdef")), 6742839);
        assert_eq!(six_zeros(parse_input("pqrstuv")), 5714438);
        //spell-checker: enable
    }
}
