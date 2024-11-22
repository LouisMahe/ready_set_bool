use boolean_evaluator::*;
use gray_code::generate_gray_combinations;

#[derive(Debug, PartialEq)]
pub enum SatError
{
    NoVarError,
    EvalError,
}

fn parse_input(input: &str) -> Vec<char>
{
    let mut v = Vec::new();

    for c in b'A'..=b'Z'
    {
        if input.contains(c as char)
        {
            v.push(c as char);
        }
    }
    v
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


pub fn sat(formula: &str) -> Result<bool, SatError>
{
    let vars = parse_input(formula);
    if vars.is_empty()
    {
        return Err(SatError::NoVarError);
    }
    let combi = generate_gray_combinations(vars.len());
    for combination in combi
    {
        let mut formula = String::from(formula);
        for n in 0..vars.len()
        {
            formula = formula.replace(&vars[n].to_string(), &combination[n].to_string());
        }
        match eval_formula(&formula)
        {
            Ok(true) => return Ok(true),
            Ok(false) => (),
            Err(_e) => return Err(SatError::EvalError),
        }
    }

    Ok(false)
}


#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn sat_test()
    {
        assert_eq!(sat("AB=AB!&&"), Ok(false));
        assert_eq!(sat("==||"), Err(SatError::NoVarError));
        assert_eq!(sat("AB^CD|"), Err(SatError::EvalError));
        assert_eq!(sat("A1|A!&"), Ok(true));
        assert_eq!(sat("AA!="), Ok(false));
        assert_eq!(sat("PQ=P!R&>"), Ok(true));
        assert_eq!(sat("AA!&"), Ok(false));
    }
}
