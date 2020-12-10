use std::io::Read;
use std::collections::HashMap;
use core::str::FromStr;


#[test]
fn test() {
let input = String::from(
r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#);
    let mut lines = parse(input);
    assert_eq!(execute(&mut lines), 22 * 10);

}

fn parse(input: String) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for line in input.lines() {
        result.push(FromStr::from_str(line).unwrap());

    }
    result
}


fn execute(lines: &mut Vec<u32>) -> u32 {
    lines.push(lines.iter().max().unwrap() + 3);
    lines.push(0);
    lines.sort();
    let mut map: HashMap<u32, u32> = HashMap::new();
    for i in 1..lines.len() {
        let now = lines[i];
        let last = lines[i - 1];
        let diff = now - last;
        println!("diff {}", diff);
        match map.get(&diff) {
            Some(value) => map.insert(diff, value + 1),
            None => map.insert(diff, 1),
        };

    }
    println!("1 {}, 3 {}", map.get(&1).unwrap(), map.get(&3).unwrap()); 
    map.get(&1).unwrap() * map.get(&3).unwrap()
}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let mut lines: Vec<u32> = parse(input);
    println!("Loop exited. Acc {}", execute(&mut lines));
}
