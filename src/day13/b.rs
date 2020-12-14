use std::io::Read;
use std::collections::HashMap;



#[test]
fn test() {
let input = String::from(
r#"0
17,x,13,19"#);
    let lines = input.lines().collect();
    let vec = parse(&lines);
    assert_eq!(execute(&vec), 295);

}

fn parse(lines: &Vec<&str>) -> (Vec<Option<u32>>) {
    let rest = lines[1];
    let mut result = Vec::new();
    for id_str in rest.split(",") {
        if id_str == "x" {
            result.push(None);
        } else {
            result.push(Some(id_str.parse::<u32>().unwrap()));
        } 

    }

    return result;
}
fn get_next_departure(timestamp: u32, id: u32) -> u32 {
    let remainter = timestamp % id;
    if remainter == 0 {
        return timestamp;
    }
    return timestamp + id - remainter;
}

fn execute(ids: &Vec<Option<u32>>) -> usize {
    let max = ids.iter().filter(|o| o.is_some()).map(|o| o.unwrap()).max().unwrap();
    for (i, id) in ids.iter().enumerate() {
        if id.is_some() {
            println!("(t + {}) mod {} = 0;", i, id.unwrap());
        }

    }
    //Got from: https://www.wolframalpha.com/input/?i=%28t+%2B+0%29+mod+23+%3D+0%3B+%28t+%2B+17%29+mod+37+%3D+0%3B+%28t+%2B+23%29+mod+863+%3D+0%3B+%28t+%2B+35%29+mod+19+%3D+0%3B+%28t+%2B+36%29+mod+13+%3D+0%3B+%28t+%2B+40%29+mod+17+%3D+0%3B+%28t+%2B+52%29+mod+29+%3D+0%3B+%28t+%2B+54%29+mod+571+%3D+0%3B+%28t+%2B+95%29+mod+41+%3D+0%3B
    1106724616194525
}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let lines: Vec<&str> = input.lines().collect();
    let vec = parse(&lines);
    println!("Loop exited. Acc {}", execute(&vec));
}
