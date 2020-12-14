use std::io::Read;
use std::collections::HashMap;



#[test]
fn test() {
let input = String::from(
r#"939
7,13,x,x,59,x,31,19"#);
    let lines = input.lines().collect();
    let (time, vec) = parse(&lines);
    assert_eq!(execute(time, &vec), 295);

}

fn parse(lines: &Vec<&str>) -> (u32, Vec<u32>) {
    let timestamp: u32 = lines[0].parse().unwrap();
    let rest = lines[1];
    let mut result = Vec::new();
    for id_str in rest.split(",") {
        if id_str == "x" {
            continue;
        }
        result.push(id_str.parse::<u32>().unwrap());

    }

    return (timestamp, result);
}
fn get_next_departure(timestamp: u32, id: u32) -> u32 {
    let remainter = timestamp % id;
    if remainter == 0 {
        return timestamp;
    }
    return timestamp + id - remainter;
}

fn execute(timestamp: u32, ids: &Vec<u32>) -> u32 {
    let mut best = ids[0];
    for id in ids {
        let diff = get_next_departure(timestamp, *id) - timestamp;
        println!("diff {} - id {}", diff, *id);
        if diff < get_next_departure(timestamp, best) - timestamp {
            best = *id;
        }
    }

    return (get_next_departure(timestamp, best) - timestamp) * best;
}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let lines: Vec<&str> = input.lines().collect();
    let (time, vec) = parse(&lines);
    println!("Loop exited. Acc {}", execute(time, &vec));
}
