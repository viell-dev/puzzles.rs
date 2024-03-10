fn main() {
    let elevator = common::Elevator::new();
    let result = elevator.into_iter().position(|floor| floor == -1).unwrap() + 1;
    println!("Result: {}", result);
}
