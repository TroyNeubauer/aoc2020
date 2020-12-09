use std::io::Read;
use std::collections::HashMap;
use std::str::FromStr;



#[test]
fn test() {
let input = String::from(
r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#);
    assert_eq!(get_bad_number(parse(input), 5, 5).unwrap(), 127);

}

fn get_bad_number(lines: Vec<usize>, pre_length: usize, index: usize) -> Option<usize> {
    if lines.len() <= index as usize {
        return None;
    }
    println!("eval index {}", index);
    let value = lines[index as usize];
    for i in 0..pre_length {
        for j in 0..pre_length {
            if i == j {
                continue;
            }
            let a = lines[(i + index - pre_length) as usize];
            let b = lines[(j + index - pre_length) as usize];
            println!("{} + {} and value: {}", a, b, value);
            if a + b == value {
                return get_bad_number(lines, pre_length, index + 1);
            }


        }
    }
    return Some(value);
}

fn parse(input: String) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    for line in input.lines() {
        result.push(FromStr::from_str(line).unwrap());

    }
    result
}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let numbers = parse(input);
    println!("Loop exited. Acc {}", get_bad_number(numbers, 25, 25).unwrap());
}
