#![allow(dead_code)]

fn gray_code(a: u32) -> u32
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




