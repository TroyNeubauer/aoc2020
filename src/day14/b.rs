use std::io::Read;
use std::collections::HashMap;



#[test]
fn test() {
let input = String::from(
r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
"#);
    let lines = input.lines().collect();
    assert_eq!(execute(lines), 208);

}
fn next_address(count: &mut u64, mask: u64, base_address: u64) -> Option<u64> {
    if *count == 2_u64.pow(mask.count_ones()) {
        return None;
    }
    let mut unset_address = base_address & !mask;
    let mut bits_cloned = 0;
    for i in 0..64 {
        if (mask >> i) & 0b1 == 1 {
            println!("bit {} set", i);
            unset_address |= (*count >> bits_cloned & 0b1) << i;
            bits_cloned += 1;
        }

    }
    *count += 1;
    return Some(unset_address);
}

fn execute(lines: Vec<&str>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask_on: u64 = 0;
    let mut mask_floating: u64 = 0;
    for line in lines {
        let data_str = line.split(" = ").nth(1).unwrap();
        if line.starts_with("mask") {
            assert_eq!(data_str.len(), 36);
            mask_on = 0;
            mask_floating = 0;
            for c in data_str.chars() {
                mask_on <<= 1;
                mask_floating <<= 1;
                match c {
                    'X' => mask_floating |= 1,
                    '1' => mask_on |= 1,
                    '0' => {},
                    _ => panic!("unknown char in mask"),
                }
               
            }
            println!("got mask on {}, floating {}", mask_on, mask_floating);

        } else if line.starts_with("mem") {
            let open = line.chars().position(|c| c == '[').unwrap();
            let close = line.chars().position(|c| c == ']').unwrap();
            let address_str = &line[open + 1..close];
            let mut address = address_str.parse::<u64>().unwrap();
            let data = data_str.parse::<u64>().unwrap();
            address |= mask_on;
            address &= 0xF__FF_FF_FF_FF;
            println!("writing {} to {} - {} set ones", data, address, 2_u64.pow(mask_floating.count_ones()));
            let mut count = 0;
            loop {
                match next_address(&mut count, mask_floating, address) {
                    Some(next_addr) => {
                        println!("writing address {}", next_addr);
                        memory.insert(next_addr, data);
                    },
                    None => break,
                }
            }


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
