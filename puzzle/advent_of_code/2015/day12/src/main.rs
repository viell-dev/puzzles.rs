use input_reader::InputReader;
use serde_json::{from_str, Value};

fn sum_all_numbers(json: &Value) -> i64 {
    match json.clone() {
        Value::Array(v) => v.iter().map(sum_all_numbers).sum(),
        Value::Object(v) => v.values().map(sum_all_numbers).sum(),
        Value::Number(v) => v.as_i64().expect("invalid input"),
        _ => 0,
    }
}

fn purge_red(json: Value) -> Value {
    match json {
        Value::Object(v) => v
            .values()
            .find(|v| v == &&Value::String(String::from("red")))
            .map_or_else(
                || {
                    let mut v = v.clone();
                    v.values_mut().for_each(|v| *v = purge_red(v.clone()));
                    Value::Object(v)
                },
                |_| Value::Null,
            ),
        Value::Array(v) => Value::Array(v.iter().map(|v| purge_red(v.clone())).collect::<Vec<_>>()),
        v => v,
    }
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

    let json = from_str::<Value>(&input).expect("invalid input");

    let sum1 = sum_all_numbers(&json);
    let sum2 = sum_all_numbers(&purge_red(json));

    println!("Sum1: {}, Sum2: {}", sum1, sum2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_all_numbers() {
        assert_eq!(
            sum_all_numbers(&from_str::<Value>(r#"[1,2,3]"#).unwrap()),
            6
        );
        assert_eq!(
            sum_all_numbers(&from_str::<Value>(r#"{"a":2,"b":4}"#).unwrap()),
            6
        );
        assert_eq!(
            sum_all_numbers(&from_str::<Value>(r#"[[[3]]]"#).unwrap()),
            3
        );
        assert_eq!(
            sum_all_numbers(&from_str::<Value>(r#"{"a":{"b":4},"c":-1}"#).unwrap()),
            3
        );
        assert_eq!(
            sum_all_numbers(&from_str::<Value>(r#"{"a":[-1,1]}"#).unwrap()),
            0
        );
        assert_eq!(
            sum_all_numbers(&from_str::<Value>(r#"[-1,{"a":1}]"#).unwrap()),
            0
        );
        assert_eq!(sum_all_numbers(&from_str::<Value>(r#"[]"#).unwrap()), 0);
        assert_eq!(sum_all_numbers(&from_str::<Value>(r#"{}"#).unwrap()), 0);
    }

    #[test]
    fn test_purge_red() {
        assert_eq!(
            purge_red(from_str::<Value>(r#"[1,2,3]"#).unwrap()),
            from_str::<Value>(r#"[1,2,3]"#).unwrap()
        );
        assert_eq!(
            purge_red(from_str::<Value>(r#"[1,{"c":"red","b":2},3]"#).unwrap()),
            from_str::<Value>(r#"[1,null,3]"#).unwrap()
        );
        assert_eq!(
            purge_red(from_str::<Value>(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap()),
            from_str::<Value>(r#"null"#).unwrap()
        );
        assert_eq!(
            purge_red(from_str::<Value>(r#"[1,"red",5]"#).unwrap()),
            from_str::<Value>(r#"[1,"red",5]"#).unwrap()
        );
    }

    #[test]
    fn test_purge_red_sum_all_numbers() {
        assert_eq!(
            sum_all_numbers(&purge_red(from_str::<Value>(r#"[1,2,3]"#).unwrap())),
            6
        );
        assert_eq!(
            sum_all_numbers(&purge_red(
                from_str::<Value>(r#"[1,{"c":"red","b":2},3]"#).unwrap()
            )),
            4
        );
        assert_eq!(
            sum_all_numbers(&purge_red(
                from_str::<Value>(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap()
            )),
            0
        );
        assert_eq!(
            sum_all_numbers(&purge_red(from_str::<Value>(r#"[1,"red",5]"#).unwrap())),
            6
        );
    }
}
