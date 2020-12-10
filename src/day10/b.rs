use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
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

#[test]
fn test3() {
let input = String::from(
r#"0
1
4
5
6
7
10
11
12
15
16
19
22"#);
    let mut lines = parse(input);
    assert_eq!(count_helper(&lines, 0), 8);

}



#[test]
fn test4() {
let input = String::from(
r#"0
1
2
3
4
7
8
9
10
11
14
17
18
19
20
23
24
25
28
31
32
33
34
35
38
39
42
45
46
47
48
49
52"#);
    let mut lines = parse(input);
    assert_eq!(count_helper(&lines, 0), 19208);

}

#[test]
fn test5() {
    //Test sub sections of the first example since this is how the main algorithm will operate
    assert_eq!(count_helper(&[4, 5, 6, 7], 0), 4);
    assert_eq!(count_helper(&[10, 11, 12], 0), 2);
}



fn parse(input: String) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for line in input.lines() {
        result.push(FromStr::from_str(line).unwrap());

    }
    result
}

fn count_helper(lines: &[u32], mut index: usize) -> usize {
    if index == lines.len() - 1 {
        return 1;
    }
    let mut count = 0;
    let base = lines[index];
    index += 1;
    while index < lines.len() {
        let now = lines[index];
        let diff = now - base;
        //println!("comp {} and {}. d={}", base, now, diff);
        if diff > 3 {
            //println!("diff between {} and {} is too large!", base, now);
            break;
        }
        count += count_helper(lines, index);

        index += 1;

    }
    count
}


fn execute(lines: &mut Vec<u32>) -> usize {
    lines.push(0);
    lines.sort();
    lines.push(lines[lines.len() - 1] + 3);
    let mut count = 1;
    let mut start = 0;
    let mut end = 1;
    loop {
        if end == lines.len() || lines[end] - lines[end - 1] == 3 {
            //println!("calling count_helper with range {}..{}", start, end);
            count *= count_helper(&lines[start..end], 0);
            start = end;
            if end == lines.len() {
                //println!("reached end of text {}", end);
                break;
            }
        }
        end += 1;

    }

    count
}



fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let mut lines: Vec<u32> = parse(input);
    println!("Loop exited. Acc {}", execute(&mut lines));
}
