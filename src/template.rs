use std::io::Read;
use std::collections::HashMap;



#[test]
fn test() {
let input = String::from(
r#""#);
    let lines = input.lines().collect();
    assert_eq!(execute(lines), 5);

}

fn execute(lines: Vec<&str>) -> i32 {
}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let lines: Vec<&str> = input.lines().collect();
    println!("Loop exited. Acc {}", execute(lines));
}
