use std::env::args;
use multiplier::multiplier;

fn main()
{
    let args: Vec<String> = args().skip(1).collect();
    if args.len() != 2
    {
        println!("Enter two natural integers as parameters");
        return;
    }
    let x = args[0].parse::<u32>().unwrap();
    let y = args[1].parse::<u32>().unwrap();
    let res = multiplier(x, y);
    println!("{x} x {y} = {res}");
}