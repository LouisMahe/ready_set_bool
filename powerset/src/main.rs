use powerset::powerset;
use std::env::args;

/* 
The powerset of a set S is the collection of all subset of S including S and 
the empty set.
The powerset function takes a finite integer set and returns its powerset.
Example : 
S = [1,2]
Powerset(S) = [] [1] [2] [1,2]
*/

fn main()
{
    
    let args : Vec<String> = args().skip(1).collect();
    let set : Vec<i32> = args.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    println!("The powerset of {:?} is:", set);
    let mut powerset = powerset(set);
    powerset.sort_by(|a, b| a.len().cmp(&b.len()));
    for subset in powerset
    {
        println!("{:?}", subset);
    }


} 