use negation_normal_form::*;
use std::env::args;

fn main()
{
	let args : Vec<String> = args().skip(1).collect();
	if args.len() != 1
	{
		println!("Enter a logical formula that will be taken to negation normal form eg \'A!!\' -> \'A\'");
		return;
	}
	let nnf = negation_normal_form(&args[0]);
	match nnf
	{
		Ok(s) => println!("{s}"),
		Err(e) => println!("Could not build a nnf from imput : {:?}", e),
	}
	

}


/*
strings to test
A!! => A
AB&!
AB|!
AB=  => AB&A!B!&|
*/
