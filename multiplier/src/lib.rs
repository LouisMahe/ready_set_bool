#![allow(dead_code)]

use adder::adder;

pub fn multiplier(a: u32, b:u32) -> u32
{
    let mut res = 0;
    let mut cb = b;
    let mut shift = 0;
    while cb > 0
    {
        if cb & 1 != 0
        {
            res = adder(res,a << shift);
        }
        cb >>= 1;
        shift = adder(shift, 1);
    }
    res
}


#[cfg(test)]
mod test{
    use super::*;
#[test]
fn test()
{
    let r1 = multiplier(3, 6);
    let r2 = multiplier(0, 6);
    let r3 = multiplier(8, 0);
    let r4 = multiplier(147, 75);
    assert_eq!(r1, 18);
    assert_eq!(r2, 0);
    assert_eq!(r3,0);
    assert_eq!(r4, 11025);
}
}