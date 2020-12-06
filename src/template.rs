use std::io::Read;


fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let mut counter = 0;
    for passport in input {
    
    }
    println!("Total: {}", counter);

}
