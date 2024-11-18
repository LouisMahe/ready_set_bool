use std::env::args;
use cnf::{conjunctive_normal_form, quine_mccluskey};


fn main()
{
    let formula = "AB|C&!";
    let res =quine_mccluskey(formula);
    if let Ok(s) = res
    {
        println!("{s}");
    }
}