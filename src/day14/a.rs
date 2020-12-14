use std::io::Read;
use std::collections::HashMap;



#[test]
fn test() {
let input = String::from(
r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#);
    let lines = input.lines().collect();
    assert_eq!(execute(lines), 165);

}

fn execute(lines: Vec<&str>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask_on: u64 = 0;
    let mut mask_off: u64 = 0;
    for line in lines {
        let data_str = line.split(" = ").nth(1).unwrap();
        if line.starts_with("mask") {
            assert_eq!(data_str.len(), 36);
            for c in data_str.chars() {
                mask_on <<= 1;
                mask_off <<= 1;
                match c {
                    'X' => {},
                    '1' => mask_on |= 1,
                    '0' => mask_off |= 1,
                    _ => panic!("unknown char in mask"),
                }
               
            }
            println!("got mask on {}, off {}", mask_on, mask_off);

        } else if line.starts_with("mem") {
            let open = line.chars().position(|c| c == '[').unwrap();
            let close = line.chars().position(|c| c == ']').unwrap();
            let address_str = &line[open + 1..close];
            let address = address_str.parse::<u64>().unwrap();
            let mut data = data_str.parse::<u64>().unwrap();
            data &= !mask_off;
            data |= mask_on;
            data &= 0xF__FF_FF_FF_FF;
            println!("writing {} to {}", data, address);
            memory.insert(address, data);

        } else {
            panic!("unknown command");
        }
    }
    println!("{:?}", memory);
    let mut result = 0;
    for value in memory.values() {
        result += value;
    }
    result
}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let lines: Vec<&str> = input.lines().collect();
    println!("Loop exited. Acc {}", execute(lines));
}
