use std::collections::HashMap;
use std::io::Read;


#[test]
fn test4() {
    assert_eq!(execute(vec!(0, 3, 6), 4), 0);
}


#[test]
fn test5() {
    assert_eq!(execute(vec!(0, 3, 6), 5), 3);
}


#[test]
fn test6() {
    assert_eq!(execute(vec!(0, 3, 6), 6), 3);
}


#[test]
fn test7() {
    assert_eq!(execute(vec!(0, 3, 6), 7), 1);
}


#[test]
fn test8() {
    assert_eq!(execute(vec!(0, 3, 6), 8), 0);
}

#[test]
fn test9() {
    assert_eq!(execute(vec!(0, 3, 6), 9), 4);
}


#[test]
fn test10() {
    assert_eq!(execute(vec!(0, 3, 6), 10), 0);
}



//#[test]
fn test2() {
    assert_eq!(execute(vec!(1, 3, 2), 2020), 1);
    assert_eq!(execute(vec!(2, 1, 3), 2020), 10);
    assert_eq!(execute(vec!(1, 1, 3), 2020), 27);
    assert_eq!(execute(vec!(2, 3, 1), 2020), 78);
    assert_eq!(execute(vec!(3, 2, 1), 2020), 1836);
}

fn next(map: &HashMap<u32, u32>, last: u32, turn: u32) -> u32 {
    let next = match map.get(&last) {
        Some(last_turn) => {
            if *last_turn == turn - 1 {
                println!("last number was spoken {}", last);
                0
            } else {
                println!("found {} in turns {}-{}", last, last_turn, turn - 1);
                turn - 1 - last_turn
            }
        }
        None => {
            println!("first non spoken {}", last);
            0
        },
    };
    println!("speaking {} in turn {}", next, turn);
    return next;
}

fn execute(starting_nums: Vec<u32>, end_turn: u32) -> u32 {
    let mut turn = 1;
    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut next_value = 0;
    let mut last = 0;
    for num in starting_nums {
        map.insert(num, turn);
        turn += 1;
        last = num;
    }
    while turn <= end_turn {
        next_value = next(&map, last, turn); 
        map.insert(last, turn - 1);
        last = next_value;
        turn += 1;
    }

    last
}

fn main() {
    println!(
        "Loop exited. Acc {}",
        execute(vec!(12, 1, 16, 3, 11, 0), 2020)
    );
}
