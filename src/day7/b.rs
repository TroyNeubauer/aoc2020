use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_name<'a>(color: u32, names: &'a HashMap<&str, u32>) -> Option<&'a str> {
    names.iter()
        .find_map(|(key, &val)| if color == val { Some(*key) } else { None }) 

}

fn sum_deep(color: u32, graph: &Vec<Vec<(u32, u32)>>, names: &HashMap<&str, u32>, indent: u32) -> usize {
    let indent_str = (0..indent * 3).map(|_| " ").collect::<String>();
    println!("{}checking {}", indent_str, get_name(color, names).unwrap());

    let mut result: usize = 0;
    for (suspect, count) in &graph[color as usize] {
        result += sum_deep(*suspect, graph, &names, indent + 1) * (*count as usize) + *count as usize;

    }
    return result;
}

fn parse(input: &String) -> (Vec<Vec<(u32, u32)>>, HashMap<&str, u32>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut graph: Vec<Vec<(u32, u32)>> = vec![Vec::new(); lines.len()];
    let mut names: HashMap<&str, u32> = HashMap::new();

    for line in &lines {
        let color = line.split(" bags").next().unwrap();
        names.insert(color, names.len() as u32);
    }


    for line in &lines {

        let mut it = line.split(" bags contain ");
        let color_str = it.next().unwrap();
        let rest = it.next().unwrap();
        let color = names.get(color_str).unwrap();
        for child_color_str in rest.split(", ") {
            if child_color_str == "no other bags." {
                continue;
            }
            let mut sub = 4;
            if child_color_str.ends_with(".") {
                sub = 5;
            }
            let number_pos = child_color_str.chars().position(|c| c == ' ').unwrap();
            let count = &child_color_str[..number_pos].parse::<u32>().unwrap();
            let child_color_str_better = &child_color_str[number_pos + 1..child_color_str.len()-sub].trim();
            let child_color = names.get(child_color_str_better).unwrap();
            println!("child {} \"{}\" - \"{}\" count {}", child_color, child_color_str_better, child_color_str, count);
            graph[*color as usize].push((*child_color, *count));


        }
    }
    (graph, names)

}

#[test]
fn test() {
    let input = String::from(
r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#);
    let (mut graph, names) = parse(&input);

    let target_color = *names.get("shiny gold").unwrap();
    let count = sum_deep(target_color, &graph, &names, 0);
    assert_eq!(count, 32);

}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let lines: Vec<&str> = input.lines().collect();
    let (mut graph, names) = parse(&input);

    let target_color = *names.get("shiny gold").unwrap() as u32;
    let count = sum_deep(target_color, &graph, &names, 0);
    //println!("names {:?}", graph);
    println!("Total: {}", count);

}
