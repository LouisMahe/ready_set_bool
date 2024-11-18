use negation_normal_form::*;


fn main()
{
	let s = "AB=CD&!!!&!".to_string();
	let tree = build_tree(&s);
	if tree.is_ok()
	{
		let tree = tree.unwrap();
		bin_tree::print_tree(tree.clone());
		remove_implies(&tree);
		normalize_neg(&tree);
		normalize_neg(&tree);
		bin_tree::print_tree(tree.clone());
		let back = string_from_tree(tree);
		println!("{back}");
	}

}


/*
strings to test
A!! => A
AB&!
AB|!
AB=  => AB&A!B!&|
*/
