use adder::adder;
use std::env::args;

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
    let res = adder(x, y);
    println!("{x} + {y} = {res}");


}