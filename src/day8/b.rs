use std::io::Read;
use std::collections::HashMap;



#[test]
fn test() {
let mut input = String::from(
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
    assert_eq!(execute(&mut input).unwrap(), 8);

}

fn execute(input: &mut String) -> Option<i32> {
    let mut rip: i32 = 0;
    let mut acc: i32 = 0;

    let lines: Vec<&str> = input.lines().collect();
    for change in 0..lines.len() {
        let mut visit_count: HashMap<i32, u32> = HashMap::new();
        rip = 0;
        acc = 0;
        let mut abort = false;
        while rip >= 0 && (rip as usize) < lines.len() {
            if visit_count.get(&rip).is_some() {
                //println!("reached inst {} once already", rip);
                abort = true;
                break;
            } else {
                visit_count.insert(rip, 1);
            }
            let line = lines[rip as usize];
            let split_index = line.chars().position(|c| c == ' ').unwrap();
            let mut inst = &line[..split_index];
            let arg: i32 = line[split_index + 1..].trim().parse().unwrap();
            if rip as usize == change {
                match inst {
                    "nop" => { inst = "jmp" },
                    "jmp" => { inst = "nop" },
                    _ => (),
                }


            }
            //println!("inst {}, arg {}", inst, arg);
            
            match inst {
                "nop" => rip += 1,
                "acc" => { rip += 1; acc += arg },
                "jmp" => { rip += arg },
            _ => panic!("bad instruction!")
            }

        }
        if abort {
            continue;
        }
            
        println!("EXITED: at exit {}, rip {}", acc, rip);
        return Some(acc);

    }
    println!("Failed brute force {}, rip {}", acc, rip);

    None
}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    println!("Loop exited. Acc {}", execute(&mut input).unwrap());
}
