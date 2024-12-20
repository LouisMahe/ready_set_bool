
use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
use std::fmt::Display;
use std::env;
use bin_tree::*;

pub const NEG: char = '!';
pub const AND: char = '&';
pub const OR: char = '|';
pub const XOR: char = '^';
pub const EQUIV: char = '=';
pub const IMPL: char = '>';
pub const T: char = '1';
pub const F: char = '0';

pub const OPERATOR_LIST: [char; 6] = [NEG, AND, OR, XOR, EQUIV, IMPL];


#[derive(Debug, Clone, PartialEq)]
pub enum Token
{
    Var(char),
    Op(char),
}

#[derive(Debug)]
pub enum ParseError
{
    InvalidFormula,
    InvalidToken,
}

#[derive(Debug)]
pub enum EvalError
{
    InvalidTree(String)
}

impl Display for Token
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self{
            Self::Var(c) => {write!(f, "{c}")},
            Self::Op(c) => {write!(f, "{c}")},
        }
    }
}


pub fn build_tree(formula: &str) -> Result<Leaf<Token>, ParseError>
{

    let mut stack : Vec<Leaf<Token>> = Vec::new();

    for c in formula.chars()
    {
        match c
        {
            'A'..='Z' => stack.push(Node::new(Token::Var(c), None, None)),
            x if OPERATOR_LIST.contains(&c) && c != NEG => {
                if stack.len() < 2 {
                    return Err(ParseError::InvalidFormula);
                }
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Node::new(Token::Op(x), Some(left), Some(right)));
            },
            NEG => {if stack.is_empty(){
                return Err(ParseError::InvalidFormula);
            }
            let left = stack.pop().unwrap();
            stack.push(Node::new(Token::Op(NEG), Some(left), None))
        },
        _ => {println!("Invalid Token {c}"); return Err(ParseError::InvalidToken);},
        }
    }
    if stack.len() != 1
    {
        Err(ParseError::InvalidFormula)
    }
    else{
        Ok(stack.pop().unwrap())
    }
}


pub fn string_from_tree(root : Leaf<Token>) -> String
{
    let mut v : Vec<char> = Vec::new();
    let curr_leaf = root.borrow();
    if let Some(left_leaf) = &curr_leaf.left
    {
        let partial_s = string_from_tree(left_leaf.clone());
        v.extend(partial_s.chars());
    }
    if let Some(right_leaf) = &curr_leaf.right
    {
        let partial_s = string_from_tree(right_leaf.clone());
        v.extend(partial_s.chars());
    }
    match curr_leaf.elem
    {
        Token::Op(c) => v.push(c),
        Token::Var(c) => v.push(c),
    }
    v.iter().collect()
}


pub fn remove_equivalent(root :&Leaf<Token>)
{
    let mut node = root.borrow_mut();
    if node.elem == Token::Op(EQUIV)
    {
        node.elem = Token::Op(AND);
        let left = node.left.take().unwrap();
        let right = node.right.take().unwrap();
        let left_copy = Rc::new(RefCell::new(left.borrow().clone()));
        let right_copy = Rc::new(RefCell::new(right.borrow().clone()));
        let insert_left = Node::new(Token::Op(IMPL), Some(left.clone()), Some(right.clone()));
        let insert_right = Node::new(Token::Op(IMPL), Some(right_copy), Some(left_copy));
        node.left = Some(insert_left);
        node.right = Some(insert_right);
    }
    if let Some(left) = &node.left{
        remove_equivalent(left);
    }
    if let Some(right) = &node.right
    {
        remove_equivalent(right);
    }
}

pub fn remove_implies(root : &Leaf<Token>)
{
    let mut node = root.borrow_mut();
    if node.elem == Token::Op(IMPL)
    {
        node.elem = Token::Op(OR);
        let left = node.left.take().unwrap();
        let insert = Node::new(Token::Op(NEG), Some(left), None);
        node.left = Some(insert);
    }
    if let Some(left) = &node.left
    {
        remove_implies(left);
    }
    if let Some(right) = &node.right
    {
        remove_implies(right);
    }
}


pub fn normalize_neg(root : &Leaf<Token>)
{
    let mut node = root.borrow_mut();
    if node.elem == Token::Op(NEG)
    {
      let left = node.left.take().unwrap();
      let mut left_node = left.borrow_mut();
      if Token::Op(NEG) == left_node.elem
      {
        let new_node = left_node.left.take().unwrap();
        node.elem = new_node.borrow().elem.clone();
        node.left = new_node.borrow().left.clone();
        node.right = new_node.borrow().right.clone();
      }
      else if Token::Op(AND) == left_node.elem
      {
        let left_left = left_node.left.take().unwrap();
        let right_right = left_node.right.take().unwrap();
        node.elem = Token::Op(OR);
        node.left = Some(Node::new(Token::Op(NEG), Some(left_left), None));
        node.right = Some(Node::new(Token::Op(NEG), Some(right_right), None));
      }
      else if Token::Op(OR) == left_node.elem
      {
        let left_left = left_node.left.take().unwrap();
        let right_right = left_node.right.take().unwrap();
        node.elem = Token::Op(AND);
        node.left = Some(Node::new(Token::Op(NEG), Some(left_left), None));
        node.right = Some(Node::new(Token::Op(NEG), Some(right_right), None));
      }
      else{
        node.left = Some(left.clone());
      }

    }
    else if node.elem == Token::Op(EQUIV)
    {
        node.elem = Token::Op(OR);
        let left = node.left.take().unwrap();
        let right = node.right.take().unwrap();
        let neg_left = Node::new(Token::Op(NEG), Some(left.clone()), None);
        let neg_right = Node::new(Token::Op(NEG), Some(right.clone()), None);
        node.left = Some(Node::new(Token::Op(AND), Some(left.clone()), Some(right.clone())));
        node.right = Some(Node::new(Token::Op(AND), Some(neg_left), Some(neg_right)))
    }
    if let Some(left) = &node.left
    {
        normalize_neg(left);
    }
    if let Some(right) = &node.right
    {
        normalize_neg(right);
    }
}

pub fn negation_normal_form(formula : &str) -> Result<String , ParseError>
{
    let tree = build_tree(formula)?;
    let print = env::var("RUST_PRINT").unwrap_or_else(|_e| "false".to_string()) == "true";
    if print
    {
        println!("original tree:\n");
        bin_tree::print_tree(tree.clone());
    }
	remove_equivalent(&tree);
    if print
    {
        println!("Equivalences removed \n");
        bin_tree::print_tree(tree.clone());
    }
    remove_implies(&tree);
    if print
    {
        println!("Implies removed \n");
        bin_tree::print_tree(tree.clone());
    }
	normalize_neg(&tree);
	normalize_neg(&tree);
    if print
    {
        println!("\nnegation normal form tree: \n");
	    bin_tree::print_tree(tree.clone());
    }
    Ok(string_from_tree(tree))
}


#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn neg_test()
    {
        let res = negation_normal_form("AB&!").unwrap();
        assert_eq!(res, "A!B!|");
        let res = negation_normal_form("AB|!").unwrap();
        assert_eq!(res, "A!B!&");
        let res = negation_normal_form("AB>").unwrap();
        assert_eq!(res, "A!B|");
        let res = negation_normal_form("AB=").unwrap();
        assert_eq!(res, "A!B|B!A|&");
        let res = negation_normal_form("AB|C&!").unwrap();
        assert_eq!(res, "A!B!&C!|");
        let res = negation_normal_form("PQ&RS|!|!TU!&>").unwrap();
        assert_eq!(res, "PQ&R!S!&|TU!&|");

    }
}
