
use boolean_evaluator::eval_formula;
use std::collections::HashMap;
#[derive(Debug)]
pub enum CnfError
{
    NoVarError,
    FormulaError,
}

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

fn generate_cnf_elem(vars : &Vec<char>, values : &Vec<u8>) -> String
{
    let mut res = vars.iter().zip(values.iter()).map(|(a,b)|{
        if *b == 0
        {
            a.to_string()
        }
        else{
            a.to_string() + "!"
        }
    }).collect::<Vec<String>>().join("");
    for _ in 1..vars.len()
    {
        res.push('|');
    }
    res
}

pub fn conjunctive_normal_form_brute_force(formula :&str) -> Result<String, CnfError>
{
    let vars = parse_input(formula);
    if vars.is_empty(){
        return Err(CnfError::NoVarError);
    }
    let comb = generate_combinations(vars.len());
    let mut v : Vec<String> = Vec::new();
    for c in comb
    {
        let mut assignation = String::from(formula);
        for n in 0..vars.len(){
            assignation = assignation.replace(&vars[n].to_string(), &c[n].to_string());
        }
        match eval_formula(&assignation)
        {
            Ok(false) => v.push(generate_cnf_elem(&vars, &c)),
            Ok(true) => (),
            Err(_e) => return Err(CnfError::FormulaError),
        }
    }
    for _ in 1..v.len()
    {
        v.push("&".to_string());
    }

    Ok(v.join(""))
}


fn check_mergable(first : &Vec<u8> , second : &Vec<u8>) -> bool
{
    for (a,b) in first.iter().zip(second.iter())
    {
        if *a == '-' as u8 && *b != '-' as u8{
            return false;
        }
        else if *a != '-' as u8 && *b == '-' as u8 {
            return false;
        }
    }
    let diff = first.iter().zip(second.iter()).fold(0, |a, (x,y)|
    {
        if *x != *y {a + 1}
        else {a}
    });
    diff == 1
}

fn merge(u : &Vec<u8>, v : &Vec<u8>) -> Vec<u8>
{
    let mut merged : Vec<u8> = Vec::new();
    for (x, y) in u.iter().zip(v.iter())
    {
        if *x != *y{ merged.push('-' as u8);}
        else {merged.push(*x );}
    }
    merged
}

fn prime_implicants(minterm: &Vec<Vec<u8>>) -> Vec<Vec<u8>>
{
    let mut implicants : Vec<Vec<u8>> = Vec::new();
    let mut merges : Vec<bool> = vec![false; minterm.len()];
    let mut merge_count = 0;

    for i in 0..minterm.len(){
        for j in i+1..minterm.len()
        {
            if check_mergable(&minterm[i], &minterm[j])
            {
                let merged = merge(&minterm[i], &minterm[j]);
                if !implicants.contains(&merged){
                    implicants.push(merge(&minterm[i], &minterm[j]));
                    merge_count += 1;
                    merges[i] = true;
                    merges[j] = true;
                }
            }
        }
    }
    for i in 0..minterm.len()
    {
        if merges[i] == false && !implicants.contains(&minterm[i])
        {
            implicants.push(minterm[i].clone());
        }
    }
    if merge_count == 0 { implicants}
    else{
        prime_implicants(&implicants)
    }
    

}

fn check_match(implicant: &Vec<u8>, minterm : &Vec<u8>) -> bool
{
    for (s,t) in implicant.iter().zip(minterm.iter())
    {
        if *s != *t && *s != '-' as u8{
            return false;
        }
    }
    true
}
fn build_hashmap(implicants : &Vec<Vec<u8>>, minterm : &Vec<Vec<u8>>) -> HashMap<Vec<u8>, Vec<usize>>
{
    let mut hash : HashMap<Vec<u8>, Vec<usize>> = implicants.iter().map(|key| (key.clone(), Vec::new())).collect();
    for i in 0..minterm.len()
    {
        for (key, value) in &mut hash
        {
            if check_match(key, &minterm[i])
            {
                value.push(i);
            }
        }
    }
    hash
}

pub fn count_dash(v : &Vec<u8>) -> usize
{
    v.iter().filter(|&c| *c == 45).count()
}

pub fn get_essentials(mut map : HashMap<Vec<u8>, Vec<usize>>, minterms : &Vec<Vec<u8>>) -> Vec<Vec<u8>>
{
    let mut essentials : Vec<Vec<u8>> = Vec::new();
    let mut index : Vec<usize>= (0..minterms.len()).collect();

    for i in 0..minterms.len()
    {
        let mut count = 0;
        let mut candidate : Vec<u8> = Vec::new();
        let mut candidate_index : Vec<usize> = Vec::new();
        for (key, values) in &mut map
        {
            if values.contains(&i)
            {
                if count == 0 {
                    candidate = key.clone();
                    candidate_index = values.clone();
                }
                count += 1;
            }            
        }
        if count == 1{
            map.remove(&candidate);
            essentials.push(candidate);
            index.retain(|&x| !candidate_index.contains(&x));
            for values in map.values_mut()
            {
                values.retain(|&x| x != i);
            }
        }
        if index.is_empty() {break;}
    }
    let mut best = 0;
    while !index.is_empty()
    {
        let mut candidate : Vec<u8> = Vec::new();
        let mut candidate_index : Vec<usize> = Vec::new();
        for (key, value) in &mut map
        {
            let mut actual = 0;
            for i in &index
            {
                if value.contains(&i)
                {
                    actual += 1;
                }
            }
            if actual > best || (actual == best && count_dash(key) > count_dash(&candidate)){
                best = actual;
                candidate = key.clone();
                candidate_index = value.clone();
            }
        }
        map.remove(&candidate);
        essentials.push(candidate);
        best = 0;
        for values in map.values_mut()
        {
            values.retain(|x| !candidate_index.contains(x));
        }
        index.retain(|x| !candidate_index.contains(x));
    }
    essentials
}

fn string_representation(v : &Vec<u8>, vars : &Vec<char>) -> String
{
    let mut res = String::new();
    let mut count = 0;
    for i in 0..v.len()
    {
        match v[i]{
            1 => {res.push(vars[i]); res.push('!'); count+=1;},
            0 => {res.push(vars[i]); count+=1;},
            _ => (),
        }
    }
    for _ in 1..count
    {
        res.push('|');
    }
    res
}

pub fn quine_mccluskey(formula :&str) -> Result<String, CnfError>
{
    let vars = parse_input(formula);
    if vars.is_empty(){
        return Err(CnfError::NoVarError);
    }
    let comb = generate_combinations(vars.len());
    let mut v : Vec<Vec<u8>> = Vec::new();
    for c in comb
    {
        let mut assignation = String::from(formula);
        for n in 0..vars.len(){
            assignation = assignation.replace(&vars[n].to_string(), &c[n].to_string());
        }
        match eval_formula(&assignation)
        {
            Ok(true) => v.push(c.clone()),
            Ok(false) => (),
            Err(_e) => return Err(CnfError::FormulaError),
        }
    }
    let prime_implicants = prime_implicants(&v);
    let map = build_hashmap(&prime_implicants, &v);
    let essentials = get_essentials(map, &v);
    let mut dnf = String::new();
    for v in essentials.iter(){
        dnf += &string_representation(v, &vars);
    }
    for _ in 1..essentials.len()
    {
        dnf.push('&');
    }
    Ok(dnf)
}


pub fn conjunctive_normal_form(formula : &str) -> Result<String, CnfError>
{
    let negated = String::from(formula) + &"!";
    quine_mccluskey(&negated)
}