use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_name<'a>(color: u32, names: &'a HashMap<&str, u32>) -> Option<&'a str> {
    names.iter()
        .find_map(|(key, &val)| if color == val { Some(*key) } else { None }) 

}

fn contains_deep(color: u32, target_color: u32, graph: &Vec<Vec<(u32, u32)>>, backtrack: &mut HashMap<u32, u32>, names: &HashMap<&str, u32>, indent: u32, target_count: u32) -> bool {
    let indent_str = (0..indent * 3).map(|_| " ").collect::<String>();
    println!("{}checking {}", indent_str, get_name(color, names).unwrap());
    if target_color == color {
        println!("{}{} cannot check itsef", indent_str, get_name(color, names).unwrap());
        return false;
    }

    //if backtrack.contains_key(&color) {
    //    println!("{}already checked {} - {}", indent_str, get_name(color, names).unwrap(), backtrack.get(&color).unwrap());
    //    return *backtrack.get(&color).unwrap() > 0 && *backtrack.get(&color).unwrap() <= target_count;
    //}
    
    //backtrack.insert(color, true);
    for (suspect, count) in &graph[color as usize] {
        if *suspect == target_color {
            println!("{}{} contains {}!", indent_str, get_name(color, names).unwrap(), get_name(target_color, names).unwrap());
            //backtrack.insert(color, count);
            return true;
        }
        if contains_deep(*suspect, target_color, graph, backtrack, &names, indent + 1, target_count) {
            println!("{}{} contains {} by association", indent_str, get_name(color, names).unwrap(), get_name(target_color, names).unwrap());
            //backtrack.insert(color, true);
            return true;
        }

    }
    return false;
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

    let mut counter = 0;
    let target_color = names.get("shiny gold").unwrap();
    let mut backtrack: HashMap<u32, u32> = HashMap::new();
    for connection in 0..graph.len() {
        if contains_deep(connection as u32, *target_color as u32, &mut graph, &mut backtrack, &names, 0, 1) {
            counter += 1;
        }
    }
    assert_eq!(4, counter);

}

fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let mut counter = 0;
    let lines: Vec<&str> = input.lines().collect();
    let (mut graph, names) = parse(&input);

    let target_color = *names.get("shiny gold").unwrap() as u32;
    let mut backtrack: HashMap<u32, u32> = HashMap::new();
    for connection in 0..graph.len() {
        if contains_deep(connection as u32, target_color as u32, &mut graph, &mut backtrack, &names, 0, 1) {
            counter += 1;
        }
    }
    println!("names {:?}", graph);
    println!("Total: {}", counter);

}
