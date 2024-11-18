
#![allow(unused)]
#![allow(dead_code)]

#[derive(Clone, PartialEq, Debug)]
pub struct BaseFourRepr
{
    digits : [u16;16],

}

#[derive(Clone, PartialEq, Debug)]
pub enum BFRError{

    OutOfRange,
    NoAntecendent,
    NegativeInput,
} 
    

impl BaseFourRepr
{
    fn new() -> Self
    {
        BaseFourRepr {digits : [0; 16]}
    }

    fn register_two_u16(&mut self,mut first: u16, mut second: u16)
    {
        for i in 0..8
        {
            
            self.digits[i] = first & 3;
            first >>= 2;
        }
        let offset : usize = 8;
        for i in 0..8
        {
            self.digits[offset + i] = second & 3;
            second >>= 2;
        }
        self.digits.reverse();

    }

    fn to_int(& self) -> u32
    {
        let mut res: u32 = 0;
        for x in self.digits
        {
            res = res*4 + x as u32;
        }
        res
    }

    fn to_float(& self) -> f64
    {
        let res = "0.".to_string() + &self.to_int().to_string();
        res.parse::<f64>().unwrap()
    }

    fn from_u16(first: u16, second: u16) -> Self
    {
        let mut res = BaseFourRepr::new();
        res.register_two_u16(first, second);
        res
    }
    
    fn from_u32(mut u : u32) -> Result<BaseFourRepr, BFRError>
    {
     
        let mut digits = [0 as u16; 16];
        for i in 0..16
        {
            digits[i] = (u % 4) as u16;
            u /= 4;
        }
        digits.reverse();
        Ok(BaseFourRepr {digits : digits})


    }

    fn from_float(f : f64) -> Result<BaseFourRepr, BFRError>
    {
     
        let str_rep = f.to_string();
        if str_rep == "0".to_string()
        {
            return Ok(BaseFourRepr { digits: [0;16] })
        }
        let string_rep : Vec<&str> = str_rep.split('.').collect();
        if string_rep.len() != 2 || string_rep[0].contains('-')
        {
            return Err(BFRError::OutOfRange);
        }
        if string_rep[0].chars().any(|c| c.is_digit(10) && c != '0')
        {
            return Err(BFRError::OutOfRange);
        }
        let mut frac_part = string_rep[1].to_string();
        if frac_part.len() > 10{
            return Err(BFRError::NoAntecendent);
        }
        let mut u64 = frac_part.parse::<u64>().unwrap();
        if u64 >  4294967295
        {
            return Err(BFRError::NoAntecendent);
        }
        if let Ok(repre) = BaseFourRepr::from_u32(u64 as u32)
        {
            Ok(repre)
        }
        else{
            Err(BFRError::NoAntecendent)
        }

        
    }

    fn retrieve_pair(&self) -> (u16,u16)
    {
        let mut first : u16 = 0;
        let mut second : u16 = 0;
        for i in 0..8
        {
            first = first*4 + self.digits[i];
        }
        for i in 8..16
        {
            second = second*4 + self.digits[i];
        }
        (first, second)
    }

}


#[cfg(test)]
#[allow(unused)]
#[allow(dead_code)]
mod test
{
    use std::u16;

    use super::*;

    #[test]
    fn register_two_test()
    {
        let first : u16 = 0;
        let second : u16 = 1;
        let mut b_4 = BaseFourRepr::new();
        b_4.register_two_u16(first, second);
        assert_eq!(b_4.digits, [0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0]);
        let first : u16 = 3;
        let second : u16 = u16::MAX;
        let mut b_4 = BaseFourRepr::new();
        b_4.register_two_u16(first, second);
        assert_eq!(b_4.digits, [3,3,3,3,3,3,3,3,0,0,0,0,0,0,0,3]);
        let first: u16 = 12589;
        let second : u16 = 50052;
        b_4.register_two_u16(first, second);
        assert_eq!(b_4.digits, [3,0,0,3,2,0,1,0,0,3,0,1,0,2,3,1]);
    }

    #[test]
    fn to_int_test()
    {
        let before = BaseFourRepr {digits : [3,3,3,3,3,3,3,3,0,0,0,0,0,0,0,3]};
        let u = before.to_int();
        assert_eq!(u, 4294901763);
        let before = BaseFourRepr {digits : [3,0,0,3,2,0,1,0,0,3,0,1,0,2,3,1]};
        let u = before.to_int();
        assert_eq!(u, 3280220461);
        let before = BaseFourRepr {digits : [3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3]};
        let u = before.to_int();
        assert_eq!(u, 4294967295);

    }

    #[test]
    fn to_float_test()
    {
        let frep = BaseFourRepr {digits : [3,3,3,3,3,3,3,3,0,0,0,0,0,0,0,3]};
        let f = frep.to_float();
        assert_eq!(f, 0.4294901763);
        let frep = BaseFourRepr {digits : [3,0,0,3,2,0,1,0,0,3,0,1,0,2,3,1]};
        let f = frep.to_float();
        assert_eq!(f, 0.3280220461);
        let mut frep = BaseFourRepr::new();
        frep.register_two_u16(u16::MAX, u16::MAX);
        let f = frep.to_float();
        assert_eq!(f, 0.4294967295)

    }

    #[test]
    fn from_float_test()
    {
        let f = 0.0;
        assert_eq!(BaseFourRepr::from_float(f), Ok(BaseFourRepr {digits : [0;16]}));
        let f = -0.00001;
        assert_eq!(BaseFourRepr::from_float(f), Err(BFRError::OutOfRange) );
        let f = 0.4294967295;
        assert_eq!(BaseFourRepr::from_float(f), Ok(BaseFourRepr {digits : [3;16]}));
        let f = 0.3280220461;
        assert_eq!(BaseFourRepr::from_float(f), Ok(BaseFourRepr {digits : [3,0,0,3,2,0,1,0,0,3,0,1,0,2,3,1]}));
        let f =  0.4294967297;
        assert_eq!(BaseFourRepr::from_float(f), Err(BFRError::NoAntecendent));
        let f = 0.4298567395457;
        assert_eq!(BaseFourRepr::from_float(f), Err(BFRError::NoAntecendent));
        let f = 1.0;
        assert_eq!(BaseFourRepr::from_float(f), Err(BFRError::OutOfRange));
        let f = 0.000000000005;
        assert_eq!(BaseFourRepr::from_float(f), Err(BFRError::NoAntecendent));
    }
}