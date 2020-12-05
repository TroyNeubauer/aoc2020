use std::io::Read;


fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let mut counter = 0;
    for passport in input.split("\n\n") {
        println!("pass port ||{}||", passport);
        if passport.contains("byr:") && passport.contains("iyr:") && passport.contains("eyr:") && passport.contains("hgt:") && passport.contains("hcl:") && passport.contains("ecl:") && passport.contains("pid:") {
            counter += 1;

        }
    }
    println!("Total: {}", counter);

}
