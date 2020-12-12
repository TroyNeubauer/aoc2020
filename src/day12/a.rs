use std::io::Read;
use std::collections::HashMap;



#[test]
fn test() {
let input = String::from(
r#"F10
N3
F7
R90
F11
"#);
    let lines = input.lines().collect();
    assert_eq!(execute(lines), 25);

}

fn execute(lines: Vec<&str>) -> i32 {
    let mut east = 0;
    let mut north = 0;
    let mut turn = 0;
    for line in lines {
        println!("ship at {}, {}", east, north);
        let param = &line[1..].parse::<i32>().unwrap();
        match line.chars().next().unwrap() {
            'N' => north += param,
            'S' => north -= param,
            'E' => east += param,
            'W' => east -= param,
            'L' => turn += param,
            'R' => turn += 360 - param,
            'F' => {
                match turn % 360 {
                    0 => east += param,
                    90 => north += param,
                    180 => east -= param,
                    270 => north -= param,
                    _ => panic!("bad angle"),
                }
            },
            _ => panic!(),
        }

    }
    println!("ship at {}, {}", east, north);
    return east.abs() + north.abs();
}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let lines: Vec<&str> = input.lines().collect();
    println!("Result {}", execute(lines));
}
