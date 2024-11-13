use std::env::args;
use map::map;

fn main()
{
    let args : Vec<String> = args().skip(1).collect();
    if args.len() < 1
    {
        println!("Please enter two natural number that fit in an u16 or a 64 bits float");
        return;
    }
    if args.len() == 2{
        let x : u16 = args[0].parse::<u16>().unwrap();
        let y : u16 = args[1].parse::<u16>().unwrap();

        let mapped = map(x,y);
        println!("map({x}, {y}) = {mapped}");
        let inverse = 1.0/mapped;

    }
    else if args.len() == 1
    {

    }
    else{

    }
}