use boolean_evaluator::*;
use std::env::args;
fn main()
{
    let args: Vec<String> = args().collect();
	if args.len() != 2
	{
		return ;
	}
    let res = eval_formula(&args[1]);
    match res
    {
        Ok(b) => println!("The formula {} is: {}", args[1], b),
        Err(e) => println!("The formula \'{}\' could not be resolved: {}", args[1], e),
    }

}
