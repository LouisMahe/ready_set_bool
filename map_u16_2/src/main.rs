

use map_u16_2::{map, reverse_map};

fn main()
{
	for i in 0..u16::MAX
	{
		for j in 0..u16::MAX
		{
			let f = map(i,j);
			if f < 0.0 || f > 1.0 {
				println!("map error on {i} {j}");
				return;
			}
			if let Ok((a,b)) = reverse_map(f)
			{
				if i != a || j != b
				{
					println!("reverse not matching on {i} {j}");
					return;
				}
			}
			else{
				println!("reverse map error on {i} {j}");
				return;
			}
		}
	}
	println!("OK");
}
