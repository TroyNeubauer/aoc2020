use std::io::Read;

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut counter = 0;
    for passport in input.split("\n\n") {
        //println!("pass port ||{}||", passport);
        println!("NEW passport - {} good", counter);
        let mut valid_fields = 0;
        for raw_pair in passport.split(|c| c == ' ' || c == '\n') {
            let pair = raw_pair.trim();
            let mut it = pair.split(":");
            let key = match it.next() {
                Some(value) => value.trim(),
                None => break,
            };
            let value = match it.next() {
                Some(value) => value.trim(),
                None => break,
            };
            println!("key \"{}\", value \"{}\"", key, value);
            let valid = match key {
                "byr" => {
                    let year = value.parse::<i32>().expect("invalid string value");
                    year >= 1920 && year <= 2002
                }
                "iyr" => {
                    let year = value.parse::<i32>().expect("invalid string value");
                    year >= 2010 && year <= 2020
                }
                "eyr" => {
                    let year = value.parse::<i32>().expect("invalid string value");
                    year >= 2020 && year <= 2030
                }
                "hgt" => {
                    if value.len() <= 2 {
                        false
                    } else {
                        let height = value[..value.len() - 2 as usize]
                            .parse::<i32>()
                            .expect("invalid string value");
                        let unit = &value[value.len() - 2 as usize..];
                        println!(" >>>> height {} - {}", height, unit);
                        if unit == "in" {
                            height >= 59 && height <= 76
                        } else if unit == "cm" {
                            height >= 150 && height <= 193
                        } else {
                            false
                        }
                    }
                }
                "hcl" => {
                    value.chars().nth(0).unwrap() == '#'  && value.chars().skip(1).map(|c| c.is_digit(16)).count() == 6
                }
                "ecl" => {
                    value == "amb"
                        || value == "blu"
                        || value == "brn"
                        || value == "gry"
                        || value == "grn"
                        || value == "hzl"
                        || value == "oth"
                }
                "pid" => value.chars().map(|c| c.is_digit(10)).count() == 9,
                "cid" => false,

                _ => panic!("invalid case"),
            };
            if valid {
                valid_fields += 1;
            } else {
                println!("============ {} INVALID", key);
            }
        }
        if valid_fields == 7 {
            counter += 1;
        }
    }
    println!("Total: {}", counter);
}
