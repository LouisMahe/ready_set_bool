

fn cantor_pairing(x : u64, y: u64) -> u64
{
    (x*x + x + 2*x*y + 3*y + y*y)/2
}

pub fn map(x: u16, y:u16) -> f64
{
    let x : u64 = x as u64;
    let y: u64 = y as u64;

    let n = cantor_pairing(x, y) + 1;
    1.0 / n as f64
}
