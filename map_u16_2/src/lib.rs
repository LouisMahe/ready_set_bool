use std::mem;

const EXP : i16 = 991;
const BIAS: i16 = 1023;

#[derive(Debug,Clone, PartialEq)]
pub struct Mantissa
{
    bits : [u16; 32],
}
#[derive(Debug, Clone, PartialEq)]
pub enum MapError{
    NoAntecendent(String),
}

fn float_from_parts(expo: i16, mantissa : u64) -> f64
{
   let mut bits : u64 = expo as u64;
   bits <<= 52;
   bits = bits | mantissa;
   unsafe {mem::transmute(bits)}
}

fn get_float_parts(f: f64) -> (i8, i16, u64)
{
    let bits : u64 = unsafe {mem::transmute(f)};
    let sign: i8 = if bits>>63 == 0 {1} else {-1};
    let mut expo: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if expo == 0 {
        (bits & 0xfffffffffffff) << 1
    }
    else{
        (bits & 0xfffffffffffff) | 0x10000000000000
    };
    (sign, expo, mantissa)
}

impl Mantissa
{
    fn new() -> Self
    {
        Mantissa {bits : [0;32]}
    }

    fn new_from_pair(mut first : u16, mut second: u16) -> Self
    {
        let mut m = Mantissa::new();
        for i in 0..16
        {
            m.bits[i] = first & 1;
            first >>= 1;
        }
        let offset: usize = 16;
        for i in 0..16
        {
            m.bits[offset + i] = second & 1;
            second >>= 1;
        }
        m.bits.reverse();
        m
    }

    fn to_float(&self) -> f64
    {
        if self.bits == [0;32]
        {
            return 0.0;
        }
        let mut i : usize = 0;
        while self.bits[i] == 0 {i += 1;}
        i += 1;
        let expo : i16 = BIAS - i as i16;
        let mut mantissa : u64 = 0;
        for j in i..32
        {
            mantissa = mantissa | ((self.bits[j] as u64) << (51+i-j));
        }
        float_from_parts(expo, mantissa)
    }

    fn from_float(f : f64) -> Result<Self, MapError>
    {
        let (sign, expo,mut mantissa) = get_float_parts(f);
        if sign < 0
        {
            return Err(MapError::NoAntecendent("Negative float".to_string()));
        }
        if expo == 0 && mantissa == 0
        {
            return Ok(Mantissa {bits: [0;32]});
        }
        if expo > 1022 || expo < 991
        {
            return Err(MapError::NoAntecendent("Out of range".to_string()));
        }
        let zero_bits = 21 + 1022 - expo;
        if mantissa & ((1 << zero_bits) -1) != 0
        {
            return Err(MapError::NoAntecendent("Out of range".to_string()));
        }
        let mut bits = [0 as u16; 32];
        for i in (1022-expo) as usize..32
        {
            bits[i] = (mantissa >> 52 & 1) as u16;
            mantissa <<= 1;
        }
        Ok(Mantissa {bits: bits})
    }


    fn to_pair(&self) -> (u16, u16)
    {
        let mut first = 0;
        let mut second = 0;
        for i in 0..16
        {
            second = 2*second + self.bits[i];
        }
        for i in 16..32
        {
            first = 2*first + self.bits[i];
        }

        (first, second)
    }
    }


    pub fn map(x:u16, y:u16) -> f64
    {
        let m = Mantissa::new_from_pair(x, y);
        m.to_float()
    }

    pub fn reverse_map(f :f64) -> Result<(u16,u16), MapError>
    {
        let m = Mantissa::from_float(f)?;
        Ok(m.to_pair())
    }


#[cfg(test)]
mod test
{
    use std::u16;

    use super::*;

    #[test]
    fn float_from_parts_test()
    {
        let f = float_from_parts(1023, 0);
        assert_eq!(f, 1.0);
        let f = float_from_parts(991, 0);
        assert_eq!(f, 2.32830643653869628906e-10);
        let f = float_from_parts(993, 1 << 50);
        assert_eq!(f, 1.16415321826934814453e-9);
        let f = float_from_parts(992, 1 << 51);
        assert_eq!(f, 6.98491930961608886719e-10);
        let f = float_from_parts(1022,1 << 51);
        assert_eq!(f, 0.75);
        let f = float_from_parts(1002, 901599534776320);
        assert_eq!(f, 5.72297722101211547852e-7);
    }

    #[test]
    fn new_from_pair_test()
    {
        let first : u16 = 0;
        let second : u16 = 1;
        let m = Mantissa::new_from_pair(first, second);
        assert_eq!(m.bits, [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        let first : u16 = 3;
        let second : u16 = u16::MAX;
        let m = Mantissa::new_from_pair(first, second);
        assert_eq!(m.bits, [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1]);
        let first : u16 = 12589;
        let second : u16 = 50052;
        let m = Mantissa::new_from_pair(first, second);
        assert_eq!(m.bits, [1,1,0,0,0,0,1,1,1,0,0,0,0,1,0,0,0,0,1,1,0,0,0,1,0,0,1,0,1,1,0,1]);

    }

    #[test]
    fn to_float_test()
    {
        let first : u16 = 1;
        let second : u16 = 0;
        let m = Mantissa::new_from_pair(first, second);
        let f = m.to_float();
        assert_eq!(f, 2.32830643653869628906e-10);
        let first : u16 = u16::MAX;
        let second : u16 = u16::MAX;
        let m = Mantissa::new_from_pair(first, second);
        let f = m.to_float();
        assert_eq!(f, 0.999999999767169356346);
        let first : u16 = 12589;
        let second : u16 = 50052;
        let m = Mantissa::new_from_pair(first, second);
        let f = m.to_float();
        assert_eq!(f, 0.763735841261222958565);
        let first : u16 = 47;
        let second: u16 = 46321;
        let m = Mantissa::new_from_pair(second, first);
        let f = m.to_float();
        assert_eq!(f, 0.000727948034182190895081);
    }

    #[test]
    fn from_float_test()
    {
        let f = 0.999999999767169356346;
        assert_eq!(Mantissa::from_float(f), Ok(Mantissa {bits: [1;32]}));
        let f = 2.32830643653869628906e-10;
        assert_eq!(Mantissa::from_float(f), Ok(Mantissa::new_from_pair(1,0)));
        let f = 0.75;
        assert_eq!(Mantissa::from_float(f), Ok(Mantissa {bits : [1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}));
        let f = 0.1236;
        assert_eq!((Mantissa::from_float(f)), Err(MapError::NoAntecendent("Out of range".to_string())));
        let f = 0.0;
        assert_eq!(Mantissa::from_float(f), Ok(Mantissa {bits : [0;32]}));
        let f = 1.16415321826934814453e-9;
        assert_eq!(Mantissa::from_float(f), Ok(Mantissa {bits : [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1]}));
        let f = 2.32830643653869628906e-10;
        assert_eq!(Mantissa::from_float(f), Ok(Mantissa {bits : [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1] }));
        let f = 0.0629883110523223876953;
        assert_eq!(Mantissa::from_float(f), Ok(Mantissa {bits : [0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0]}));
        let f = 1.2;
        assert_eq!(Mantissa::from_float(f), Err(MapError::NoAntecendent("Out of range".to_string())));
    }

    #[test]
    fn to_pair_test()
    {
        let m = Mantissa::new_from_pair(3, 4);
        let u = m.to_pair();
        assert_eq!(u, (3,4));
        let m = Mantissa::from_float(0.999999999767169356346).unwrap();
        assert_eq!(m.to_pair(), (u16::MAX, u16::MAX));

    }
}




