use std::io::Read;
use std::collections::HashSet;
 

fn get_count(input: &String) -> usize {
    let mut counter = 0;
    for group in input.split("\n\n") {
        let mut set: HashSet<char> = HashSet::new();
        for line in group.lines() {
            for c in line.chars() {
                set.insert(c);

            }
        }

        counter += set.len();

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
    assert_eq!(get_count(&input), 11);



}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    println!("Total: {}", get_count(&input));

}
