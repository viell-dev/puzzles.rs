//! Day 7: Some Assembly Required

use std::collections::HashMap;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (a1, a2) = get_answers(input);

    println!("The signal ultimately provided to wire a is: {}", a1);
    println!("The new signal ultimately provided to wire a is: {}", a2);
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (u16, u16) {
    // Build the circuit.
    let mut circuit = build_circuit(input);
    // Run the circuit and get the map of wire names to values.
    let map1 = run_circuit(&circuit);
    // Get the value of wire a.
    let a1 = map1.get("a").unwrap().clone();
    // Convert the value of wire a to a string.
    let a = a1.to_string();

    // Set the gate of wire b to a copy of the value of wire a.
    circuit.insert("b", Gate::Copy(&a));
    // Run the circuit again and get the map of wire names to values.
    let map2 = run_circuit(&circuit);
    // Get the new value of wire a.
    let a2 = map2.get("a").unwrap().clone();

    // Return the answers.
    (a1, a2)
}

/// Run the circuit and return the values of all wires.
fn run_circuit<'a>(circuit: &'a HashMap<&str, Gate>) -> HashMap<&'a str, u16> {
    // Map of wire names to values.
    let mut map = HashMap::<&'a str, u16>::new();

    // Run the circuit for each wire.
    for key in circuit.keys() {
        // Since the circuit may contain recursive references, only run the gate
        // if the wire is not already in the map.
        if !map.contains_key(key) {
            run_gate(key, &circuit, &mut map);
        }
    }

    map
}

/// Run a gate and set the value of the target wire in the map.
/// Also return the value of the target wire for recursive calls.
fn run_gate<'a>(
    key: &'a str,
    circuit: &'a HashMap<&str, Gate>,
    map: &mut HashMap<&'a str, u16>,
) -> u16 {
    // Get the gate for the current wire.
    let gate = circuit.get(key).unwrap();

    // Get the value that the gate resolves to.
    let value = match gate {
        Gate::Copy(source) => resolve(source, circuit, map),
        Gate::Not(source) => !resolve(source, circuit, map),
        Gate::And(source1, source2) => {
            let value1 = resolve(source1, circuit, map);
            let value2 = resolve(source2, circuit, map);

            value1 & value2
        }
        Gate::Or(source1, source2) => {
            let value1 = resolve(source1, circuit, map);
            let value2 = resolve(source2, circuit, map);

            value1 | value2
        }
        Gate::LeftShift(source1, source2) => {
            let value1 = resolve(source1, circuit, map);
            let value2 = resolve(source2, circuit, map);

            value1 << value2
        }
        Gate::RightShift(source1, source2) => {
            let value1 = resolve(source1, circuit, map);
            let value2 = resolve(source2, circuit, map);

            value1 >> value2
        }
    };

    map.insert(key, value);

    value
}

/// Resolve a source wire or value.
fn resolve<'a>(
    source: &'a str,
    circuit: &'a HashMap<&str, Gate>,
    map: &mut HashMap<&'a str, u16>,
) -> u16 {
    source
        // Try to parse the source as a u16.
        .parse::<u16>()
        .ok()
        // If that fails, try to get the value from the map.
        .or(map.get(source).cloned())
        // If that fails, run the gate for the source.
        .unwrap_or_else(|| run_gate(source, circuit, map))
}

/// Build a circuit from the puzzle input.
fn build_circuit<'a>(input: &'a str) -> HashMap<&'a str, Gate<'a>> {
    let mut circuit = HashMap::<&str, Gate>::new();

    for instruction in input.lines() {
        match parse_instruction(instruction) {
            (target, gate) => circuit.insert(target, gate),
        };
    }

    circuit
}

/// Gates the puzzle input may contain.
enum Gate<'a> {
    Copy(&'a str),
    Not(&'a str),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    LeftShift(&'a str, &'a str),
    RightShift(&'a str, &'a str),
}

/// Parse an instruction into a target and a gate.
fn parse_instruction(instruction: &str) -> (&str, Gate) {
    // Split the instruction by whitespace.
    let mut parts = instruction.split_whitespace().collect::<Vec<_>>();
    // The last part is the target.
    let target = parts.pop().unwrap();

    // Drop the arrow.
    parts.pop();

    // If there is only one part left, it's a copy gate.
    if let [source] = parts.as_slice() {
        (target, Gate::Copy(source))
    }
    // If there are two parts left, it's a not gate.
    else if let [_, source] = parts.as_slice() {
        (target, Gate::Not(source))
    }
    // If there are three parts left, we need to parse the operator.
    else if let [source1, operator, source2] = parts.as_slice() {
        // Parse the operator and return the appropriate gate.
        match operator {
            &"AND" => (target, Gate::And(source1, source2)),
            &"OR" => (target, Gate::Or(source1, source2)),
            &"LSHIFT" => (target, Gate::LeftShift(source1, source2)),
            &"RSHIFT" => (target, Gate::RightShift(source1, source2)),
            _ => panic!("Invalid input."),
        }
    } else {
        panic!("Invalid input.")
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_example() {
        let instructions = "123 -> x\n\
            456 -> y\n\
            x AND y -> d\n\
            x OR y -> e\n\
            x LSHIFT 2 -> f\n\
            y RSHIFT 2 -> g\n\
            NOT x -> h\n\
            NOT y -> i";

        let map = HashMap::<&str, u16>::from([
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ]);

        assert_eq!(run_circuit(&build_circuit(instructions)), map);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (3176, 14710));
    }
}
