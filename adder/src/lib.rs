#![allow(dead_code)]

pub fn adder(a: u32, b: u32) -> u32
{
    let and = (a & b) << 1;
    let xor = a ^ b;
    if and == 0
    {
        xor
    }
    else {
        adder(and, xor)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
#[test]
fn first_test()
{
    let res = adder(6, 45);
    let res2 = adder(u32::MAX, 3);
    let res3 = adder(0, 0);
    assert_eq!(res, 51);
    assert_eq!(res2, 2);
    assert_eq!(res3, 0);
}}