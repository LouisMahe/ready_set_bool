use std::collections::HashSet;
use boolean_evaluator::*;

#[derive(Debug)]
pub enum EvalSetError
{
    VarSetMatchError,
    EvalError,
    EmptyError,
}

fn union_set(sets: &Vec<Vec<i32>>) -> Vec<i32>
{
    let mut set = Vec::new();
    let mut elements = HashSet::new();

    for x in sets.iter().flatten()
    {
        if elements.insert(*x){
            set.push(*x);
        }
    }
    set
}

fn get_vars(formula : &str) -> Vec<char>
{
    let mut vars : Vec<char> = Vec::new();
    for c in b'A'..=b'Z'
    {
        if formula.contains(c as char)
        {
            vars.push(c as char);
        }
        else {break;}
    }

    vars
}

pub fn eval_set(formula : &str, sets: Vec<Vec<i32>>) -> Result<Vec<i32>, EvalSetError>
{
    let sets_vars = get_vars(formula);
    if sets_vars.is_empty() || sets.is_empty()
    {
        return Err(EvalSetError::EmptyError);
    }
    if sets_vars.len() != sets.len()
    {
        return Err(EvalSetError::VarSetMatchError);
    }
    let union_set = union_set(&sets);
    let mut res_set = Vec::new();
    for elem in union_set
    {
        let mut formula = String::from(formula);
        for idx in 0..sets_vars.len()
        {
            let var = sets_vars[idx];
            let value = if sets[idx].contains(&elem) {b'1' as char} else {b'0' as char};
            formula = formula.replace(&var.to_string(), &value.to_string());
        }
        if let Ok(res) = eval_formula(&formula)
        {
            match res{
                true => res_set.push(elem),
                false => (),
            }
        }
        else {
            return Err(EvalSetError::EvalError);
        }

    }

    Ok(res_set)

}


#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn eval_set_test()
    {
        let sets = vec![vec![0,1,2], vec![0,3,4]];
        let result = eval_set("AB&", sets);
        assert_eq!(result.unwrap(), vec![0]);
        let sets = vec![vec![0,1,2]];
        let result = eval_set("A!", sets);
        assert_eq!(result.unwrap(), vec![]);
        let sets = vec![vec![1,2], vec![1,3], vec![1,4]];
        let result = eval_set("ABC||", sets).unwrap();
        assert_eq!(result, vec![1,2,3,4]);
        let sets = vec![vec![1,3,5], vec![1,2,3,4,5,6]];
        let result = eval_set("BA!&", sets).unwrap();
        assert_eq!(result, vec![2,4,6]);


    }
}
