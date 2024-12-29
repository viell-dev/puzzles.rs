use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr, Not, Shl, Shr},
};

use input_reader::InputReader;

#[derive(Debug, Clone, PartialEq)]
enum ParseError {
    Wire(String),
    Raw(String),
    Gate(String),
    Instruction(String),
}

/// A wire is identified by some lowercase letters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Wire(String);

impl TryFrom<String> for Wire {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.chars().all(|c| c.is_ascii_lowercase()) {
            true => Ok(Self(value)),
            false => Err(ParseError::Wire(value)),
        }
    }
}

impl TryFrom<Value> for Wire {
    type Error = ParseError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Wire(wire) => Ok(Self(wire)),
            Value::Raw(raw) => Err(ParseError::Wire(raw.to_string())),
        }
    }
}

/// Raw values are 16-bit signals.
#[derive(Debug, Clone, PartialEq)]
struct Raw(u16);

impl BitAnd for Raw {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0.bitand(rhs.0))
    }
}

impl BitOr for Raw {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitor(rhs.0))
    }
}

impl Shl for Raw {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self(self.0.shl(rhs.0))
    }
}

impl Shr for Raw {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self(self.0.shr(rhs.0))
    }
}

impl Not for Raw {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(self.0.not())
    }
}

impl From<u16> for Raw {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for Raw {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.parse::<u16>() {
            Ok(int) => Ok(Self(int)),
            Err(_) => Err(ParseError::Raw(value)),
        }
    }
}

impl TryFrom<Value> for Raw {
    type Error = ParseError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Wire(wire) => Err(ParseError::Raw(wire)),
            Value::Raw(raw) => Ok(Self(raw)),
        }
    }
}

/// A value is either a wire identifier or a 16-bit signal
#[derive(Debug, Clone, PartialEq)]
enum Value {
    Wire(String),
    Raw(u16),
}

impl From<Wire> for Value {
    fn from(value: Wire) -> Self {
        Value::Wire(value.0)
    }
}

impl From<Raw> for Value {
    fn from(value: Raw) -> Self {
        Value::Raw(value.0)
    }
}

impl TryFrom<String> for Value {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Wire::try_from(value.clone())
            .map(|wire| wire.into())
            .or(Raw::try_from(value).map(|raw| raw.into()))
    }
}

/// A logic gate and the wire/signal's connected to it.
#[derive(Debug, Clone, PartialEq)]
enum Gate {
    Set(Value),
    And(Value, Value),
    Or(Value, Value),
    LeftShift(Value, Value),
    RightShift(Value, Value),
    Not(Value),
}

impl TryFrom<String> for Gate {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match *value.split_whitespace().collect::<Vec<_>>().as_slice() {
            [a] => Value::try_from(a.to_owned()).map(Gate::Set),
            [a, "AND", b] => Value::try_from(a.to_owned())
                .and_then(|a| Value::try_from(b.to_owned()).map(|b| (a, b)))
                .map(|(a, b)| Gate::And(a, b)),
            [a, "OR", b] => Value::try_from(a.to_owned())
                .and_then(|a| Value::try_from(b.to_owned()).map(|b| (a, b)))
                .map(|(a, b)| Gate::Or(a, b)),
            [a, "LSHIFT", b] => Value::try_from(a.to_owned())
                .and_then(|a| Value::try_from(b.to_owned()).map(|b| (a, b)))
                .map(|(a, b)| Gate::LeftShift(a, b)),
            [a, "RSHIFT", b] => Value::try_from(a.to_owned())
                .and_then(|a| Value::try_from(b.to_owned()).map(|b| (a, b)))
                .map(|(a, b)| Gate::RightShift(a, b)),
            ["NOT", a] => Value::try_from(a.to_owned()).map(Gate::Not),
            _ => Err(ParseError::Gate(value)),
        }
        .map_err(|error| match error {
            ParseError::Gate(error) | ParseError::Wire(error) | ParseError::Raw(error) => {
                ParseError::Gate(error)
            }
            ParseError::Instruction(_) => unreachable!(),
        })
    }
}

/// An instruction for building a circuit
#[derive(Debug, Clone, PartialEq)]
struct Instruction(Gate, Wire);

impl TryFrom<String> for Instruction {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match *value.split(" -> ").collect::<Vec<_>>().as_slice() {
            [gate, wire] => Gate::try_from(gate.to_owned()).and_then(|gate| {
                Wire::try_from(wire.to_owned()).map(|wire| Instruction(gate, wire))
            }),
            _ => Err(ParseError::Instruction(value.clone())),
        }
        .map_err(|error| match error {
            ParseError::Gate(_) | ParseError::Wire(_) => ParseError::Instruction(value),
            _ => unreachable!(),
        })
    }
}

fn parse_input(input: &[String]) -> impl Iterator<Item = Instruction> + Clone + '_ {
    input
        .iter()
        .map(|line| Instruction::try_from(line.clone()).expect("Invalid input"))
}

/// A circuit
struct Circuit {
    circuit: HashMap<Wire, Gate>,
    results: HashMap<Wire, Raw>,
}

impl Circuit {
    pub fn new(instructions: impl Iterator<Item = Instruction>) -> Circuit {
        let mut circuit = HashMap::new();

        for Instruction(gate, wire) in instructions {
            circuit.insert(wire, gate);
        }

        Circuit {
            circuit,
            results: HashMap::new(),
        }
    }

    pub fn insert(&mut self, instruction: Instruction) {
        self.circuit.insert(instruction.1, instruction.0);
    }

    pub fn run(&mut self) -> &HashMap<Wire, Raw> {
        self.results.clear();

        for wire in self.circuit.clone().keys() {
            if !self.results.contains_key(wire) {
                self.resolve(wire);
            }
        }

        &self.results
    }

    fn resolve_value(&mut self, value: &Value) -> Raw {
        match value {
            Value::Raw(raw) => Raw(*raw),
            Value::Wire(wire) => self.resolve(&Wire(wire.to_string())),
        }
    }

    fn resolve(&mut self, wire: &Wire) -> Raw {
        if let Some(raw) = self.results.get(wire) {
            raw.clone()
        } else {
            let gate = self
                .circuit
                .get(wire)
                .expect("should exist or something is wrong")
                .clone();

            let raw = match gate {
                Gate::Set(a) => self.resolve_value(&a),
                Gate::And(a, b) => self.resolve_value(&a) & self.resolve_value(&b),
                Gate::Or(a, b) => self.resolve_value(&a) | self.resolve_value(&b),
                Gate::LeftShift(a, b) => self.resolve_value(&a) << self.resolve_value(&b),
                Gate::RightShift(a, b) => self.resolve_value(&a) >> self.resolve_value(&b),
                Gate::Not(a) => !self.resolve_value(&a),
            };

            self.results.insert(wire.clone(), raw.clone());

            raw
        }
    }
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

    let instructions = parse_input(&input);
    let mut circuit = Circuit::new(instructions);
    let a1 = if let Some(Raw(value)) = circuit.run().get(&Wire("a".to_owned())) {
        *value
    } else {
        panic!("invalid input");
    };
    circuit.insert(Instruction(Gate::Set(Value::Raw(a1)), Wire("b".to_owned())));
    let a2 = if let Some(Raw(value)) = circuit.run().get(&Wire("a".to_owned())) {
        *value
    } else {
        panic!("invalid input");
    };

    println!("Answer 1: {}, Answer 2: {}", a1, a2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_try_from_string() {
        assert_eq!(Wire::try_from("x".to_owned()), Ok(Wire("x".to_owned())));
        assert_eq!(Wire::try_from("y".to_owned()), Ok(Wire("y".to_owned())));
        assert_eq!(Wire::try_from("d".to_owned()), Ok(Wire("d".to_owned())));
        assert_eq!(Wire::try_from("e".to_owned()), Ok(Wire("e".to_owned())));
        assert_eq!(Wire::try_from("f".to_owned()), Ok(Wire("f".to_owned())));
        assert_eq!(Wire::try_from("g".to_owned()), Ok(Wire("g".to_owned())));
        assert_eq!(Wire::try_from("h".to_owned()), Ok(Wire("h".to_owned())));
        assert_eq!(Wire::try_from("i".to_owned()), Ok(Wire("i".to_owned())));

        assert_eq!(Wire::try_from("az".to_owned()), Ok(Wire("az".to_owned())));
        assert_eq!(Wire::try_from("ka".to_owned()), Ok(Wire("ka".to_owned())));
        assert_eq!(Wire::try_from("gn".to_owned()), Ok(Wire("gn".to_owned())));
        assert_eq!(Wire::try_from("cm".to_owned()), Ok(Wire("cm".to_owned())));
        assert_eq!(Wire::try_from("bn".to_owned()), Ok(Wire("bn".to_owned())));

        assert_eq!(
            Wire::try_from("INVALID".to_owned()),
            Err(ParseError::Wire("INVALID".to_owned()))
        );
    }

    #[test]
    fn test_wire_try_from_value() {
        assert_eq!(
            Wire::try_from(Value::Wire("x".to_owned())),
            Ok(Wire("x".to_owned()))
        );

        assert_eq!(
            Wire::try_from(Value::Raw(123)),
            Err(ParseError::Wire("123".to_owned()))
        );
    }

    #[test]
    fn test_raw_bitwise() {
        assert_eq!(Raw(123) & Raw(2), Raw(2));
        assert_eq!(Raw(123) | Raw(2), Raw(123));
        assert_eq!(Raw(123) << Raw(2), Raw(492));
        assert_eq!(Raw(123) >> Raw(2), Raw(30));
        assert_eq!(!Raw(123), Raw(65412));
    }

    #[test]
    fn test_raw_try_from_string() {
        assert_eq!(Raw::try_from("72".to_owned()), Ok(Raw(72)));
        assert_eq!(Raw::try_from("114".to_owned()), Ok(Raw(114)));
        assert_eq!(Raw::try_from("123".to_owned()), Ok(Raw(123)));
        assert_eq!(Raw::try_from("456".to_owned()), Ok(Raw(456)));
        assert_eq!(Raw::try_from("492".to_owned()), Ok(Raw(492)));
        assert_eq!(Raw::try_from("507".to_owned()), Ok(Raw(507)));
        assert_eq!(Raw::try_from("65079".to_owned()), Ok(Raw(65079)));
        assert_eq!(Raw::try_from("65412".to_owned()), Ok(Raw(65412)));

        assert_eq!(
            Raw::try_from("INVALID".to_owned()),
            Err(ParseError::Raw("INVALID".to_owned()))
        );
    }

    #[test]
    fn test_raw_try_from_value() {
        assert_eq!(Raw::try_from(Value::Raw(123)), Ok(Raw(123)));

        assert_eq!(
            Raw::try_from(Value::Wire("x".to_owned())),
            Err(ParseError::Raw("x".to_owned()))
        );
    }

    #[test]
    fn test_value_from_wire() {
        assert_eq!(
            Value::from(Wire("x".to_owned())),
            Value::Wire("x".to_owned())
        );
    }

    #[test]
    fn test_value_from_raw() {
        assert_eq!(Value::from(Raw(123)), Value::Raw(123));
    }

    #[test]
    fn test_gate_try_from_string() {
        assert_eq!(
            Gate::try_from("123".to_owned()),
            Ok(Gate::Set(Value::Raw(123)))
        );
        assert_eq!(
            Gate::try_from("456".to_owned()),
            Ok(Gate::Set(Value::Raw(456)))
        );
        assert_eq!(
            Gate::try_from("x AND y".to_owned()),
            Ok(Gate::And(
                Value::Wire("x".to_owned()),
                Value::Wire("y".to_owned())
            ))
        );
        assert_eq!(
            Gate::try_from("x OR y".to_owned()),
            Ok(Gate::Or(
                Value::Wire("x".to_owned()),
                Value::Wire("y".to_owned())
            ))
        );
        assert_eq!(
            Gate::try_from("x LSHIFT 2".to_owned()),
            Ok(Gate::LeftShift(Value::Wire("x".to_owned()), Value::Raw(2)))
        );
        assert_eq!(
            Gate::try_from("y RSHIFT 2".to_owned()),
            Ok(Gate::RightShift(Value::Wire("y".to_owned()), Value::Raw(2)))
        );
        assert_eq!(
            Gate::try_from("NOT x".to_owned()),
            Ok(Gate::Not(Value::Wire("x".to_owned())))
        );
        assert_eq!(
            Gate::try_from("NOT y".to_owned()),
            Ok(Gate::Not(Value::Wire("y".to_owned())))
        );

        assert_eq!(
            Gate::try_from("INVALID".to_owned()),
            Err(ParseError::Gate("INVALID".to_owned()))
        );
    }

    #[test]
    fn test_instruction_try_from_string() {
        assert_eq!(
            Instruction::try_from("123 -> x".to_owned()),
            Ok(Instruction(
                Gate::Set(Value::Raw(123)),
                Wire("x".to_string())
            ))
        );
        assert_eq!(
            Instruction::try_from("456 -> y".to_owned()),
            Ok(Instruction(
                Gate::Set(Value::Raw(456)),
                Wire("y".to_string())
            ))
        );
        assert_eq!(
            Instruction::try_from("x AND y -> d".to_owned()),
            Ok(Instruction(
                Gate::And(Value::Wire("x".to_string()), Value::Wire("y".to_string())),
                Wire("d".to_string())
            ))
        );
        assert_eq!(
            Instruction::try_from("x OR y -> e".to_owned()),
            Ok(Instruction(
                Gate::Or(Value::Wire("x".to_string()), Value::Wire("y".to_string())),
                Wire("e".to_string())
            ))
        );
        assert_eq!(
            Instruction::try_from("x LSHIFT 2 -> f".to_owned()),
            Ok(Instruction(
                Gate::LeftShift(Value::Wire("x".to_string()), Value::Raw(2)),
                Wire("f".to_string())
            ))
        );
        assert_eq!(
            Instruction::try_from("y RSHIFT 2 -> g".to_owned()),
            Ok(Instruction(
                Gate::RightShift(Value::Wire("y".to_string()), Value::Raw(2)),
                Wire("g".to_string())
            ))
        );
        assert_eq!(
            Instruction::try_from("NOT x -> h".to_owned()),
            Ok(Instruction(
                Gate::Not(Value::Wire("x".to_owned())),
                Wire("h".to_string())
            ))
        );
        assert_eq!(
            Instruction::try_from("NOT y -> i".to_owned()),
            Ok(Instruction(
                Gate::Not(Value::Wire("y".to_owned())),
                Wire("i".to_string())
            ))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = vec![
            "123 -> x".to_owned(),
            "456 -> y".to_owned(),
            "x AND y -> d".to_owned(),
            "x OR y -> e".to_owned(),
            "x LSHIFT 2 -> f".to_owned(),
            "y RSHIFT 2 -> g".to_owned(),
            "NOT x -> h".to_owned(),
            "NOT y -> i".to_owned(),
        ];

        assert_eq!(
            parse_input(&input).collect::<Vec<_>>(),
            vec![
                Instruction(Gate::Set(Value::Raw(123)), Wire("x".to_string())),
                Instruction(Gate::Set(Value::Raw(456)), Wire("y".to_string())),
                Instruction(
                    Gate::And(Value::Wire("x".to_string()), Value::Wire("y".to_string())),
                    Wire("d".to_string())
                ),
                Instruction(
                    Gate::Or(Value::Wire("x".to_string()), Value::Wire("y".to_string())),
                    Wire("e".to_string())
                ),
                Instruction(
                    Gate::LeftShift(Value::Wire("x".to_string()), Value::Raw(2)),
                    Wire("f".to_string())
                ),
                Instruction(
                    Gate::RightShift(Value::Wire("y".to_string()), Value::Raw(2)),
                    Wire("g".to_string())
                ),
                Instruction(
                    Gate::Not(Value::Wire("x".to_owned())),
                    Wire("h".to_string())
                ),
                Instruction(
                    Gate::Not(Value::Wire("y".to_owned())),
                    Wire("i".to_string())
                )
            ]
        );
    }

    #[test]
    fn test_circuit_new() {
        let input = vec![
            "123 -> x".to_owned(),
            "456 -> y".to_owned(),
            "x AND y -> d".to_owned(),
            "x OR y -> e".to_owned(),
            "x LSHIFT 2 -> f".to_owned(),
            "y RSHIFT 2 -> g".to_owned(),
            "NOT x -> h".to_owned(),
            "NOT y -> i".to_owned(),
        ];
        let instructions = parse_input(&input);
        let circuit = Circuit::new(instructions);

        assert_eq!(
            circuit.circuit,
            HashMap::from([
                (Wire("x".to_owned()), Gate::Set(Value::Raw(123))),
                (Wire("y".to_owned()), Gate::Set(Value::Raw(456))),
                (
                    Wire("d".to_owned()),
                    Gate::And(Value::Wire("x".to_owned()), Value::Wire("y".to_owned()))
                ),
                (
                    Wire("e".to_owned()),
                    Gate::Or(Value::Wire("x".to_owned()), Value::Wire("y".to_owned()))
                ),
                (
                    Wire("f".to_owned()),
                    Gate::LeftShift(Value::Wire("x".to_owned()), Value::Raw(2))
                ),
                (
                    Wire("g".to_owned()),
                    Gate::RightShift(Value::Wire("y".to_owned()), Value::Raw(2))
                ),
                (Wire("h".to_owned()), Gate::Not(Value::Wire("x".to_owned()))),
                (Wire("i".to_owned()), Gate::Not(Value::Wire("y".to_owned()))),
            ])
        );
    }

    #[test]
    fn test_circuit_run() {
        let input = vec![
            "123 -> x".to_owned(),
            "456 -> y".to_owned(),
            "x AND y -> d".to_owned(),
            "x OR y -> e".to_owned(),
            "x LSHIFT 2 -> f".to_owned(),
            "y RSHIFT 2 -> g".to_owned(),
            "NOT x -> h".to_owned(),
            "NOT y -> i".to_owned(),
        ];
        let instructions = parse_input(&input);
        let mut circuit = Circuit::new(instructions);

        assert_eq!(
            circuit.run(),
            &HashMap::from([
                (Wire("d".to_owned()), Raw(72)),
                (Wire("e".to_owned()), Raw(507)),
                (Wire("f".to_owned()), Raw(492)),
                (Wire("g".to_owned()), Raw(114)),
                (Wire("h".to_owned()), Raw(65412)),
                (Wire("i".to_owned()), Raw(65079)),
                (Wire("x".to_owned()), Raw(123)),
                (Wire("y".to_owned()), Raw(456)),
            ])
        );
    }
}
