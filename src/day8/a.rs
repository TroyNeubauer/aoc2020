use std::io::Read;
use std::collections::HashMap;



#[test]
fn test() {
let input = String::from(
r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"#);
    let lines = input.lines().collect();
    assert_eq!(execute(lines), 5);

}

fn execute(lines: Vec<&str>) -> i32 {
    let mut visit_count: HashMap<i32, u32> = HashMap::new();
    let mut rip: i32 = 0;
    let mut acc: i32 = 0;

    while rip >= 0 && (rip as usize) < lines.len() {
        if visit_count.get(&rip).is_some() {
            println!("reached inst {} once already", rip);
            return acc;
        } else {
            visit_count.insert(rip, 1);
        }
        let line = lines[rip as usize];
        let split_index = line.chars().position(|c| c == ' ').unwrap();
        let inst = &line[..split_index];
        let arg: i32 = line[split_index + 1..].trim().parse().unwrap();
        println!("inst {}, arg {}", inst, arg);
        match inst {
            "nop" => rip += 1,
            "acc" => { rip += 1; acc += arg },
            "jmp" => { rip += arg },
            _ => panic!("bad instruction!")
        }

    }


    acc
}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let lines: Vec<&str> = input.lines().collect();
    println!("Loop exited. Acc {}", execute(lines));
}
