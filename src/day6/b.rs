use std::io::Read;
use std::collections::HashMap;
 

fn get_count(input: &String) -> usize {
    let mut counter = 0;
    for group in input.split("\n\n") {
        let mut set: HashMap<char, u8> = HashMap::new();
        let mut people = 0;
        for line in group.lines() {
            for c in line.chars() {
                match set.get(&c) {
                    Some(count) => set.insert(c, count.clone() + 1),
                    None => set.insert(c, 1)
                };

            }
            people += 1;
        }
        for (_c, count) in &set {
            if *count as usize == people {
                counter += 1;
            }


        }


    }
    counter
}



#[test]
fn test1() {
    let input = String::from(
r#"abc

a
b
c

ab
ac

a
a
a
a

b
"#);
    assert_eq!(get_count(&input), 6);



}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    println!("Total: {}", get_count(&input));

}
