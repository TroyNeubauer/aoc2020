
extern crate nom;

use std::io::Read;

fn from_base_10_str(input: &str) -> Result<i32, std::num::ParseIntError> {
    std::str::FromStr::from_str(input)

}

fn int32(inupt: &str) -> nom::IResult<&str, i32> {
    nom::combinator::map_res(
        nom::character::complete::digit1,
        from_base_10_str
    )(inupt)
}


//named!(line_impl<&str, i32>,
//    nom::fn tuple!(int32, nom::tag!("-"), int32, nom::tag!(" "), 
//        nom::character::complete::char, nom::tag!(": "), many!(nom::character::complete::char)
//    )
//);




fn get_line(input: &str) -> nom::IResult<&str, bool> {
    let (input, (min, _, max, _, c, _, password, _)) =
        nom::sequence::tuple(
            (int32,
            nom::bytes::complete::tag("-"),
            int32,
            nom::bytes::complete::tag(" "),
            nom::character::complete::anychar,
            nom::bytes::complete::tag(": "),
            nom::character::complete::not_line_ending,
            nom::character::complete::line_ending)
        )
    (input)?;
    let mut char_count = 0;
    for pass_char in password.chars() {
        if pass_char == c {
            char_count += 1;
        } 
    }
    println!("min {}, max {}, c {}, password {}", min, max, c, password);
    Ok((input, char_count >= min && char_count <= max))
}

fn main()
{
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");
    
    let mut input = input.as_str();
    let mut count = 0;
    while input.len() > 0 {
        let result = get_line(input).expect("Failed to parse input");
        if result.1 {
            count += 1;
        }
        input = result.0;
    }
    println!("Counted {} valid passwords!", count);
}

