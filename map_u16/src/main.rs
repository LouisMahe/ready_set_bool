use map_u16::*;
use std::env::args;

fn main()
{
    let args : Vec<String> = args().skip(1).collect();
    if args.len() == 2
    {
        let x = args[0].parse::<u16>().unwrap();
        let y = args[1].parse::<u16>().unwrap();
        let f = map(x,y);
        println!("map({}, {}) = {}", x, y, f);
    }
    else if args.len() == 1
    {
        let f = args[0].parse::<f64>().unwrap();
        let rev = reverse_map(f);
        match rev
        {
            Ok(x) => println!("reverse_map({}) = {:?}", f, x),
            Err(e) => println!("Could not find an antecedent for this float {:?}", e),
        }
    }
    else {
        {
            println!("Enter either two integers or a float");
        }
    }
    
}