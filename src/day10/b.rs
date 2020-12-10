use std::io::Read;
use std::collections::HashMap;
use core::str::FromStr;


#[test]
fn test() {
let input = String::from(
r#"16
10
15
5
1
11
7
19
6
12
4"#);
    let mut lines = parse(input);
    assert_eq!(execute(&mut lines), 8);

}



#[test]
fn test2() {
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
    assert_eq!(execute(&mut lines), 19208);

}


fn parse(input: String) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for line in input.lines() {
        result.push(FromStr::from_str(line).unwrap());

    }
    result
}

fn count_helper(lines: &Vec<u32>, mut index: usize) -> usize {
    if index == lines.len() - 1 {
        return 1;
    }
    let mut count = 0;
    let base = lines[index];
    //println!("processing {} index {}", base, index);
    index += 1;
    while index < lines.len() {
        let now = lines[index];
        let diff = now - base;
        //println!("comp {} and {}. d={}", base, now, diff);
        if diff >= 4 {
            //println!("diff between {} and {} is too large!", base, now);
            break;
        }
        count += count_helper(lines, index);

        index += 1;

    }
    count
}

fn execute(lines: &mut Vec<u32>) -> usize {
    lines.push(lines.iter().max().unwrap() + 3);
    lines.push(0);
    lines.sort();

    count_helper(&lines, 0)
}


fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let mut lines: Vec<u32> = parse(input);
    println!("Loop exited. Acc {}", execute(&mut lines));
}
