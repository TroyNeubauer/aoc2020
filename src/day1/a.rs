
use std::env;


fn main()
{

    //println!("Running program with args: {:?}", env::args());
    let args: Vec<u32> = env::args()
        .skip(1)
        .filter(|x| x.len() > 0)
        .map(|x| x.parse().unwrap())
        .collect();

    for i in &args
    {
        let compliment: u32 = 2020 - i;
        if args.contains(&compliment)
        {
            println!("Found anwser {}", i * compliment);
            return; 
        }
    }

    println!("Unable to find solution with args: {:?}", args);
}

