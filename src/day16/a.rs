use std::collections::HashMap;
use std::io::Read;
extern crate nom;

#[derive(Debug)]
struct Constraint {
    name: String,
    a: std::ops::Range<u32>,
    b: std::ops::Range<u32>,
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u32>,
}

#[derive(Debug)]
struct Data {
    constraints: Vec<Constraint>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

fn from_base_10_str(input: &str) -> Result<u32, std::num::ParseIntError> {
    std::str::FromStr::from_str(input)
}

fn int32(inupt: &str) -> nom::IResult<&str, u32> {
    nom::combinator::map_res(nom::character::complete::digit1, from_base_10_str)(inupt)
}

fn parse_constraint(input: &str) -> nom::IResult<&str, Constraint> {
    let (input, (name, _, a1, _, a2, _, b1, _, b2, _)) = nom::sequence::tuple((
        nom::bytes::complete::take_until(":"),
        nom::bytes::complete::tag(": "),
        int32,
        nom::bytes::complete::tag("-"),
        int32,
        nom::bytes::complete::tag(" or "),
        int32,
        nom::bytes::complete::tag("-"),
        int32,
        nom::combinator::opt(nom::character::complete::line_ending),
    ))(input)?;

    Ok((
        input,
        Constraint {
            name: String::from(name),
            a: std::ops::Range { start: a1, end: a2 + 1 },
            b: std::ops::Range { start: b1, end: b2 + 1 },
        },
    ))
}

fn parse_constraints(input_raw: &str) -> (&str, Vec<Constraint>) {
    let mut result = Vec::new();
    let mut input = input_raw;

    //Parse constraints until we hit a double newline
    while input.chars().nth(0).unwrap() != '\n' {
        let out = parse_constraint(input).expect("Failed to parse input");
        result.push(out.1);
        input = out.0;
    }

    return (input, result);
}

fn parse_comment(input: &str) -> nom::IResult<&str, &str> {
    let (input, (_, _, comment, _)) = nom::sequence::tuple((
        nom::combinator::opt(nom::character::complete::line_ending),
        nom::combinator::opt(nom::character::complete::line_ending),
        nom::character::complete::not_line_ending,
        nom::character::complete::line_ending,
    ))(input)?;

    Ok((input, comment))
}

fn parse_ticket(input_raw: &str) -> nom::IResult<&str, Ticket> {
    let (input, (line, _)) = nom::sequence::tuple((
        nom::character::complete::not_line_ending,
        nom::character::complete::line_ending,
    ))(input_raw)?;

    Ok((
        input,
        Ticket {
            fields: line
                .split(",")
                .filter(|s| s.len() > 0)
                .map(|s| {
                    println!("porsing {}", s);
                    s.parse::<u32>()
                })
                .map(|res| res.unwrap())
                .collect(),
        },
    ))
}

fn parse(input: &str) -> Data {
    let (input, con) = parse_constraints(input);
    let (input, comment1) = parse_comment(input).unwrap();
    let (input, my_ticket) = parse_ticket(input).unwrap();
    let (mut input, comment2) = parse_comment(input).unwrap();

    let mut other: Vec<Ticket> = Vec::new();
    while input.len() > 0 {
        let (inner_input, ticket) = parse_ticket(input).unwrap();
        other.push(ticket);
        input = inner_input;
    }

    Data {
        constraints: con,
        my_ticket,
        other_tickets: other,
    }
}

#[test]
fn test() {
    let input = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
"#;
    assert_eq!(part1(input), 71);
}

fn part1(input: &str) -> u32 {
    let data = parse(input);
    let mut bad_sum = 0;
    for ticket in &data.other_tickets {
        for value in &ticket.fields {
            let mut invalid = true;
            for con in &data.constraints {
                if con.a.contains(value) || con.b.contains(value) {
                    invalid = false;
                }
            }
            if invalid {
                println!("{} is bad", value);
                bad_sum += value;
            }
        }
    }
    bad_sum
}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("Loop exited. Acc {:?}", part1(input.as_str()));
}
