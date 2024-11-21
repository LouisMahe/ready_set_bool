


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

pub fn powerset(set : Vec<i32>) -> Vec<Vec<i32>>
{
    let mut powerset = Vec::new();
    if set.is_empty() {
        powerset.push(Vec::new());
        return powerset;
    }
    let combinations = generate_combinations(set.len());
    for combi in combinations
    {
        let subset: Vec<i32> = set.iter().enumerate().filter(|(idx, &_val)|{
            combi[*idx] == 1
        }).map(|(_, &val)| val).collect();
        powerset.push(subset);
    }
    powerset
}



#[cfg(test)]
mod test{

    use super::*;
    #[test]
    fn powerset_test()
    {
        let v = Vec::new();
        assert_eq!(powerset(v), vec![vec![]]);
        let v = vec![1,2];
        assert_eq!(powerset(v), vec![vec![], vec![1], vec![2], vec![1,2]]);
    }
}
