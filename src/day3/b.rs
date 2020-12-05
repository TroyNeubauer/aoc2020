use std::io::Read;
use std::str::Lines;

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut terrain: Vec<&str> = Vec::new();
    let const_input = input;

    for line in const_input.lines() {
        terrain.push(line);
    }

    let params = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut total = -1i64;
    for param in params {
        println!("Using params rowinc {}, colinc {}", param.1, param.0);
        let mut row = 0;

        let mut col = 0;
        let mut count = 0;

        while row < terrain.len() {
            let line = terrain[row].as_bytes();
            if *line.get(col % line.len()).unwrap() == '#' as u8 {
                count += 1;
            }

            col += param.0;
            row += param.1;
        }
        if total == -1 {
            total = count;
        } else {
            total *= count;
        }
    }
    println!("Total: {}", total);
}
