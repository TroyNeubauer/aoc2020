use std::io::Read;
use std::collections::HashMap;


fn get_number(line: &str) -> i32 {
    let mut number = 0;
    for c in line.chars() {
        number *= 2;
        if c == 'B' || c == 'R' {
            number += 1;
        }
    }
    number
}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let max_seat = 908;
    let mut map = HashMap::new();
    for i in 0..max_seat {
        map.insert(i, false);
    }
    for line in input.lines() {
        let number = get_number(line);
        map.insert(number, true);
    }
    for (number, present) in &map {
        if !present {
            println!("{}", number);
        }


    }
}
