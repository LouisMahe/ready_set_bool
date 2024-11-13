use sat::sat;
use std::env::args;

fn main()
{
	let args: Vec<String> = args().collect();
	if args.len() != 2
	{
		return ;
	}
	let res = sat(&args[1]);

	match res
	{
		Ok(true) => println!("The formula can be true"),
        Ok(false) => println!("The formula canno't be true"),
        Err(e) => println!("{:?}", e),
	}


}