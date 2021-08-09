use std::rc::Rc;
use std::mem::swap;
use std::mem::size_of;
use std::cmp::max;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;

struct TreeNode {
    value: u32,
    sum: u32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: u32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            sum: 0,
            left: None,
            right: None,
        }))
    }

    fn left_node(&mut self) -> Option<Rc<RefCell<TreeNode>>> {
        let left_node = self.left.take();
        if left_node.is_some() {
            let left_node = left_node.unwrap();
            self.left = Some(left_node.clone());
            Some(left_node)
        } else {
            None
        }
    }

    fn right_node(&mut self) -> Option<Rc<RefCell<TreeNode>>> {
        let right_node = self.right.take();
        if right_node.is_some() {
            let right_node = right_node.unwrap();
            self.right = Some(right_node.clone());
            Some(right_node)
        } else {
            None
        }
    }
}

fn greater_sum(node: &Option<Rc<RefCell<TreeNode>>>, sum: u32) -> u32 {
    match node {
        Some(raw_node) => {
            let mut sum = sum;
            
            sum = greater_sum(&raw_node.borrow().right, sum);

            
            let value = raw_node.borrow().value;
            sum = sum + value;
            raw_node.borrow_mut().sum = sum;
            println!("{} {}", sum, value);

            sum = greater_sum(&raw_node.borrow().left, sum);

            sum
        }
        None => {
            sum
        }
    }
}

fn traverse(node: &Option<Rc<RefCell<TreeNode>>>) {
    match node {
        Some(raw_node) => {
            traverse(&raw_node.borrow().left);
            let value = raw_node.borrow().value;
            let sum = raw_node.borrow().sum;
            println!("{} {}", sum, value);
            traverse(&raw_node.borrow().right);
        }
        None => {
        }
    }
}

fn init_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let root = TreeNode::new(4);
    
    {
        let left_node = TreeNode::new(2);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(6);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let mut node = root.borrow_mut();
        node.left = Some(left_node);
        node.right = Some(right_node);
    }

    {
        let left_node = TreeNode::new(1);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(3);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let node = root.borrow();
        let mut node = node.left.as_ref().unwrap().borrow_mut();
        node.left = Some(left_node);
        node.right = Some(right_node);
    }

    {
        let left_node = TreeNode::new(5);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(7);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let node = root.borrow();
        let mut node = node.right.as_ref().unwrap().borrow_mut();
        node.left = Some(left_node);
        node.right = Some(right_node);
    }

    return Some(root)
}

fn test_greater_sum(mark: &str, tree: Option<Rc<RefCell<TreeNode>>>){
    println!("=====start {} tree", mark);
    greater_sum(&tree, 0);
    traverse(&tree);
    println!("=====end {} tree", mark);
}

fn main() {
    test_greater_sum("greater sum", init_tree());
}
