use truth_table::*;
use std::env::args;
fn main()
{
	let args: Vec<String> = args().collect();
	if args.len() != 2
	{
		return ;
	}
	let res = truth_table(&args[1]);

	match res
	{
		Ok(s) => println!("{s}"),
		Err(e) => println!("Error: {:?}", e),
	}


}