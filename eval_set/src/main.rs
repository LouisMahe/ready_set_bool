use eval_set::eval_set;


fn main()
{
    let sets = vec![
        vec![0,2,3,4],
    ];
    let formula = "AA!&";
    let set = eval_set(formula, sets);
    println!("{:?}", set);
}