use std::env;

fn main() {
    //println!("Running program with args: {:?}", env::args());
    let args: Vec<i32> = env::args()
        .skip(1)
        .filter(|x| x.len() > 0)
        .map(|x| x.parse().unwrap())
        .collect();

    for i in &args {
        for j in &args {
            let compliment: i32 = 2020 - i - j;
            if args.contains(&compliment) {
                println!("Found anwser {}", i * j * compliment);
                return;
            }
        }
    }

    println!("Unable to find solution with args: {:?}", args);
}
