use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;
use text_trees::TreeNode;


pub type Leave<T> = Rc<RefCell<Node<T>>>;
pub type Branch<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(Debug, Clone)]
pub struct Node<T : Display + Clone>
{

    pub elem : T,
    pub left: Branch<T>,
    pub right: Branch<T>,
}

impl<T> Node<T>
where T : Display + Clone,
{
    pub fn new(elem: T, left : Branch<T>, right : Branch<T>) -> Rc<RefCell<Self>>
    {
        Rc::new(RefCell::new(Node { elem:elem, left: left, right: right }))
    }

    pub fn modify(&mut self, new_value : T)
    {
        let node = self;
        node.elem = new_value;
    }

    pub fn add_left(&mut self, left_value : T)
    {
        let node = self;
        node.left.take();
        node.left = Some(Node::new(left_value, None, None));
    }

    pub fn add_right(&mut self, right_value : T)
    {
        let node = self;
        node.right.take();
        node.right = Some(Node::new(right_value, None, None));
    }
}


pub fn make_print_tree<T: Display + Clone>(root : Leave<T>) -> TreeNode<T>
{
    let mut curr_node = TreeNode::new(root.borrow().elem.clone());
	let curr_leave = root.borrow();
	if let Some(left_leave) = &curr_leave.left
	{
		let node = make_print_tree(left_leave.clone());
		curr_node.push_node(node);
	}
	if let Some(right_leave) = &curr_leave.right
	{
		let node = make_print_tree(right_leave.clone());
		curr_node.push_node(node);
	}
	curr_node
}

pub fn print_tree<T : Display + Clone>(root: Leave<T>)
{
    let tree = make_print_tree(root);
	tree.write(&mut std::io::stdout()).unwrap();
}
