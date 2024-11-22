use std::fmt::Write;
use boolean_evaluator::*;
use gray_code::generate_gray_combinations;

#[derive(Debug)]
pub enum TableError
{
    NoVarError,
    EvalError,
}

// impl Display for SyntaxError
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
//     {
//         match self
//         {
//             SyntaxError::InvalidChar(c) => {write!(f, "Invalid character {}", c)}
//         }
//     }
// }

fn parse_input(input: &str) -> Vec<char>
{
    let mut var = Vec::new();
    for c in b'A'..= b'Z'
    {
        if input.contains(c as char)
        {
            var.push(c as char)
        }
    }
    var
}

#[allow(dead_code)]
fn generate_combinations(var_num: usize) -> Vec<Vec<u8>>
{
    let mut combinations = Vec::new();
    let total_combinations = 1 << var_num;

    for n in 0..total_combinations
    {
        let mut new_comb: Vec<u8> = Vec::with_capacity(var_num);
        for j in 0..var_num
        {
            new_comb.push(((n >> j) & 1) as u8);
        }
        combinations.push(new_comb);
    }
    combinations
}

pub fn truth_table(input: &str) -> Result<String, TableError>
{
    let mut table = String::new();
    let variables = parse_input(input);
    if variables.is_empty(){
        return Err(TableError::NoVarError);
    }
    let combinations = generate_gray_combinations(variables.len());
    for c in &variables
    {
        write!(&mut table, "| {} ", *c).unwrap();
    }
    write!(&mut table, "| = |\n").unwrap();
    table +=  &"|---".repeat(variables.len() + 1);
    table += "|\n";
    for comb in combinations
    {
        let mut formula = String::from(input);
        let mut formated = String::from("| ");
        for n in 0..variables.len()
        {
            formula = formula.replace(variables[n].to_string().as_str(), &comb[n].to_string());
            formated += &comb[n].to_string();
            formated += " | ";
        }
        match eval_formula(&formula)
        {
            Ok(true) => formated += "1 |\n",
            Ok(false)  => formated += "0 |\n",
            Err(_e) => return Err(TableError::EvalError),
        };
        table += &formated;
    }


    Ok(table)
}
