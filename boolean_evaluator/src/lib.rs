#![allow(dead_code)]
use std::{fmt::Display, rc::Rc};
use std::fmt::Write;
use bin_tree::*;
use std::env;

pub const NEG: char = '!';
pub const AND: char = '&';
pub const OR: char = '|';
pub const XOR: char = '^';
pub const EQUIV: char = '=';
pub const IMPL: char = '>';
pub const T: char = '1';
pub const F: char = '0';

pub const OPERATOR_LIST: [char; 6] = [NEG, AND, OR, XOR, EQUIV, IMPL];

type NodeRef = Rc<Node>;
#[derive(Debug, Clone, PartialEq)]
pub enum Token
{
    Boolean(bool),
    Operator(char),
}
#[derive(Debug, PartialEq)]
pub enum ParseError
{
    InvalidFormula,
    InvalidToken,
}
#[derive(Debug, PartialEq)]
pub enum  EvalError{
    InvalidTree(String)
}

impl Display for Token
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self{
            Self::Boolean(bool) => {write!(f, "{bool}")},
            Self::Operator(char) => {write!(f, "{char}")}
        }
    }
}

impl Display for EvalError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self{
            EvalError::InvalidTree(message) => {write!(f, "Invalid Tree: {}", message)}
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node
{
    val: Token,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
}

impl Node
{
    pub fn new_node(val: Token, left: Option<NodeRef>, right: Option<NodeRef>) -> Self
    {
        Self{val: val, left: left, right: right}
    }
}

pub fn build_tree(formula: &str) -> Result<Leave<Token>, ParseError>
{
    let mut chars = formula.chars();
    let mut stack: Vec<Leave<Token>> = Vec::new();

    while let Some(c) = chars.next()
    {
        match c{
            T => stack.push(bin_tree::Node::new(Token::Boolean(true), None, None)),
            F => stack.push(bin_tree::Node::new(Token::Boolean(false), None, None)),
            x if OPERATOR_LIST.contains(&x) && x != NEG => {if stack.len() < 2
                {
                    return Err(ParseError::InvalidFormula);
                }
                let right = stack .pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(bin_tree::Node::new(Token::Operator(x), Some(left), Some(right)));
            },
            NEG => {if stack.len() < 1{
                return Err(ParseError::InvalidFormula);
            }
            let left = stack.pop().unwrap();
            stack.push(bin_tree::Node::new(Token::Operator(NEG), Some(left), None));
            },
            _ => {println!("Invalid token {c}"); return Err(ParseError::InvalidToken);},
        }
    }
    if stack.len() != 1
    {
        Err(ParseError::InvalidFormula)
    }
    else {
        Ok(stack.pop().unwrap())
    }

}

fn implication(p: bool, q: bool) -> bool
{
    match (p, q)
    {
        (true, false) => false,
        _ => true,
    }
}

fn equivalence(p: bool, q: bool) -> bool
{
    p == q
}


fn unfold(tree: bin_tree::Leave<Token>) -> Result<bool, EvalError>
{
    let root = tree.borrow();
    if root.left.is_none() && root.right.is_none()
    {
        match root.elem{
            Token::Boolean(true) => return Ok(true),
            Token::Boolean(false) => return Ok(false),
            _ => {return Err(EvalError::InvalidTree(String::from("Tree does not produce a boolean result")))}
        }
    }
    else if root.left.is_some() && root.right.is_none()
    {
        if root.elem != Token::Operator(NEG)
        {
            let mut s = String::new();
            write!(& mut s, "Logical erro: operator {:?} has only one leaf", root.elem).unwrap();
            return Err(EvalError::InvalidTree(s));
        }
        let left_leaf = unfold(root.left.as_ref().unwrap().clone());
        match left_leaf
        {
            Ok(b) => return Ok(!b),
            Err(e) => return Err(e),
        }
    }

    let left_leaf = unfold((root.left.as_ref().unwrap()).clone());
    let right_leaf = unfold(root.right.as_ref().unwrap().clone());
    if left_leaf.is_err()
    {
        return Err(left_leaf.unwrap_err());
    }
    if right_leaf.is_err()
    {
        return Err(right_leaf.unwrap_err());
    }
    match root.elem
    {
        Token::Operator(AND) => return Ok(left_leaf.unwrap() && right_leaf.unwrap()),
        Token::Operator(OR) => return Ok(left_leaf.unwrap() || right_leaf.unwrap()),
        Token::Operator(XOR) => Ok(left_leaf.unwrap() ^ right_leaf.unwrap()),
        Token::Operator(EQUIV) => return Ok(equivalence(left_leaf.unwrap(), right_leaf.unwrap())),
        Token::Operator(IMPL) => return Ok(implication(left_leaf.unwrap(), right_leaf.unwrap())),
        _ => {let mut s = String::new(); write!(&mut s, "Operator {:?} has two leeves", root.elem).unwrap();
                    return Err(EvalError::InvalidTree(s));}
    }


}

pub fn eval_formula(formula:  &str) -> Result<bool, EvalError>
{
    let tree = match build_tree(formula)
    {
        Ok(t) => t,
        Err(_e) => {return Err(EvalError::InvalidTree(String::from("Could not build tree from input")));},
    };
    let print = env::var("RUST_PRINT").unwrap_or_else(|_e| "false".to_string()) == "true";
    if print{
        println!("The logical tree is: ");
        bin_tree::print_tree(tree.clone());
    }
    unfold(tree)
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn bool_eval_test()
    {
        assert_eq!(eval_formula("10&"), Ok(false));
        assert_eq!(eval_formula("11!|"), Ok(true));
        assert_eq!(eval_formula("1w&"), Err(EvalError::InvalidTree("Could not build tree from input".to_string())));
        assert_eq!(eval_formula("1011||="), Ok(true));
    }
}
