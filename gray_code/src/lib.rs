#![allow(dead_code)]

pub fn generate_gray_combinations(n : usize) -> Vec<Vec<u8>>
{
    let mut combinations = Vec::new();
    let total_comb = 1 << n;

    for k in 0..total_comb{
        let mut new_comb : Vec<u8> = Vec::with_capacity(n);
        let gray = gray_code(k as u32);
        for j in 0..n
        {
            new_comb.push(((gray >> j) & 1) as u8);
        }
        combinations.push(new_comb);
    }
    combinations
}

pub fn gray_code(a: u32) -> u32
{
    a ^ (a >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v: Vec<u32> = vec![0,1,3,2,6,7,5,4,12];
        for n in 0..v.len()
        {
            let res = gray_code(n as u32);
            assert_eq!(res, v[n]);
        }
    }
}




