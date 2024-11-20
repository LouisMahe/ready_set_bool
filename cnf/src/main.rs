use cnf::conjunctive_normal_form;


fn main()
{
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 1
    {
        println!("Enter a logical formula to obtain a conjunctive normal form of it.");
        return;
    }
    let res = conjunctive_normal_form(&args[0]);
    match res
    {
        Ok(s) => println!("A conjunctive normal form is: {s}"),
        Err(e) => println!("Error {:?}", e),
    }
}