use gray_code::gray_code;
use std::env::args;

fn main()
{
    let args: Vec<String> = args().skip(1).collect();
    if args.len() != 1
    {
        println!("Enter one natural integer as parameter");
        return;
    }
    let x = args[0].parse::<u32>().unwrap();
    let next = x+1;
    let res = gray_code(x);
    let res_next = gray_code(next);
    println!("In Gray code {x} is {res}");
    println!("In Gray code {next} is {res_next}");
    println!("{res} ^ {res_next} = {}", res^res_next);
}