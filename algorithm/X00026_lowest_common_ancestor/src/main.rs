use std::rc::Rc;
use std::mem::swap;
use std::cmp::max;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;

struct TreeNode {
    value: u32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: u32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
        }))
    }
}

fn lowest_common_ancestor(node: &Option<Rc<RefCell<TreeNode>>>, p:&TreeNode, q: &TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    match node {
        Some(raw_node) => {
            if raw_node.borrow().value == p.value || raw_node.borrow().value == q.value {
                return Some(raw_node.clone())
            }
            let left_ancestor = lowest_common_ancestor(&raw_node.borrow().left, p, q);
            let right_ancestor = lowest_common_ancestor(&raw_node.borrow().right, p, q);
            match left_ancestor {
                Some(raw_left_ancestor) => {
                    match right_ancestor {
                        Some(raw_right_ancestor) => {
                            println!("left {}, right {}", raw_left_ancestor.borrow().value, raw_right_ancestor.borrow().value);
                            Some(raw_node.clone())
                        }
                        None => {
                            Some(raw_left_ancestor.clone())
                        }
                    }
                }
                None => {
                    match right_ancestor {
                        Some(raw_right_ancestor) => {
                            Some(raw_right_ancestor.clone())
                        }
                        None => {
                            None
                        }
                    }
                }
            }
        }
        None => {
            None
        }
    }
}

fn init_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = TreeNode::new(2);
    
    {
        let left_node = TreeNode::new(7);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(4);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        root.borrow_mut().left = Some(left_node);
        root.borrow_mut().right = Some(right_node);
    }

    {
        let left_node = TreeNode::new(6);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = root;

        root = TreeNode::new(5);
        root.borrow_mut().left = Some(left_node);
        root.borrow_mut().right = Some(right_node);
    }

    {
        let left_node = TreeNode::new(0);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(8);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let node = TreeNode::new(1);
        node.borrow_mut().left = Some(left_node);
        node.borrow_mut().right = Some(right_node);


        let left_node = root;
        let right_node = node;
        root = TreeNode::new(3);
        root.borrow_mut().left = Some(left_node);
        root.borrow_mut().right = Some(right_node);
    }

    return Some(root)
}

fn test_lowest_common_ancestor(mark: &str, root: Option<Rc<RefCell<TreeNode>>>, p: u32, q: u32, predicate_value: u32) {
    println!("=====start {} tree", mark);
    let ancestor = lowest_common_ancestor(&root, &TreeNode {
        value:p,
        left: None,
        right: None,
    }, &TreeNode{
        value:q,
        left: None,
        right: None,
    });
    match ancestor {
        Some(raw_ancestor) => {
            println!("{} {} result: {} predicate: {}", p, q, raw_ancestor.borrow().value, predicate_value);
        }
        None => {
            println!("{} {} no result", p, q);
        }
    }
    println!("=====end {} tree", mark);
}

fn main() {
    test_lowest_common_ancestor("common ancestor", init_tree(), 5, 1, 3);
    test_lowest_common_ancestor("common ancestor", init_tree(), 5, 4, 5);
    test_lowest_common_ancestor("common ancestor", init_tree(), 2, 8, 3);
    test_lowest_common_ancestor("common ancestor", init_tree(), 4, 6, 5);
}
