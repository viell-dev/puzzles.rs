fn main() {
    let elevator = common::Elevator::new();
    let result = elevator.last().unwrap();
    println!("Result: {}", result);
}
