use std::io;

fn main() {
    let input = read_input();
    let parsed = parse_input(&input);
    let largest = find_largest_sum(&parsed);
    println!("Largest sum: {}", largest);
    let largest_three = find_largest_three_sum(&parsed);
    println!("Largest three sum: {}", largest_three);
}

fn read_input() -> Vec<Vec<String>> {
    let mut input = Vec::<Vec::<String>>::new();
    let mut lines = Vec::<String>::new();
    let mut last_line_was_empty = false;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        
        // Break if we encounter two empty lines in a row
        if line.is_empty() {
            if last_line_was_empty {
                break;
            } else {
                last_line_was_empty = true;
                input.push(lines);
                lines = Vec::new();
                continue;
            }
        } else {
            last_line_was_empty = false;
        }

        lines.push(line);
    }

    if !lines.is_empty() {
        input.push(lines);
    }

    input
}

fn parse_input(input: &Vec<Vec<String>>) -> Vec<Vec<usize>> {
    let mut parsed = Vec::<Vec<usize>>::new();

    for group in input {
        let mut group_parsed = Vec::<usize>::new();
        for line in group {
            let line_parsed = line.parse::<usize>().unwrap();
            group_parsed.push(line_parsed);
        }
        parsed.push(group_parsed);
    }

    parsed
}

fn find_largest_sum(input: &Vec<Vec<usize>>) -> usize {
    let mut largest = 0;

    for group in input {
        let sum = group.iter().sum();
        if sum > largest {
            largest = sum;
        }
    }

    largest
}

fn find_largest_three_sum(input: &Vec<Vec<usize>>) -> usize {
    let mut largest1 = 0;
    let mut largest2 = 0;
    let mut largest3 = 0;

    for group in input {
        let sum = group.iter().sum();

        if sum > largest1 {
            largest3 = largest2;
            largest2 = largest1;
            largest1 = sum;
        } else if sum > largest2 {
            largest3 = largest2;
            largest2 = sum;
        } else if sum > largest3 {
            largest3 = sum;
        }
    }

    largest1 + largest2 + largest3
}
