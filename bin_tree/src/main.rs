
use bin_tree::*;

enum Token
{
	Num(i32),
	Op(char),
}

impl Display for Token
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self{
            Self::Num(i) => {write!(f, "{i}")},
            Self::Op(char) => {write!(f, "{char}")}
        }
    }
}

fn main()
{
	let root = Node::new(Token::Op('+'), None, None);



}
