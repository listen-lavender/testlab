use std::rc::Rc;
use std::mem::swap;
use std::cmp::max;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;

// #[derive(Copy, Clone)]
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

fn preorder_traverse(node: &Option<Rc<RefCell<TreeNode>>>) {
    match node {
        Some(raw_node) => {
            let value = raw_node.borrow().value;
            print!("{}", value);
            preorder_traverse(&raw_node.borrow().left);
            preorder_traverse(&raw_node.borrow().right);
        }
        None => {
        }
    }
}

fn inorder_traverse(node: &Option<Rc<RefCell<TreeNode>>>) {
    match node {
        Some(raw_node) => {
            inorder_traverse(&raw_node.borrow().left);
            let value = raw_node.borrow().value;
            print!("{}", value);
            inorder_traverse(&raw_node.borrow().right);
        }
        None => {
        }
    }
}

fn count_maybe_bst(start:i32, end: i32) -> u32 {
    if end < start {
        return 1
    }
    let mut total:u32 = 0;
    for mid in start..(end + 1) {
        total = total + count_maybe_bst(start, mid - 1) * count_maybe_bst(mid + 1, end);
    }
    total
}

fn build_maybe_bst(start:i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if end < start {
        return vec![None]
    }
    let mut total_bst:Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    for mid in start..(end + 1) {
        let mut left_bst = build_maybe_bst(start, mid - 1);
        let mut right_bst = build_maybe_bst(mid + 1, end);
        for (i, left_node) in left_bst.iter_mut().enumerate() {
            for (j, right_node) in right_bst.iter_mut().enumerate() {
                let node = TreeNode::new(mid as u32);

                if left_node.is_none() {
                    node.borrow_mut().left = None;
                } else {
                    let left_node = left_node.take();
                    let left_node = left_node.unwrap();
                    node.borrow_mut().left = Some(left_node.clone());
                }

                if right_node.is_none() {
                    node.borrow_mut().right = None;
                } else {
                    let right_node = right_node.take();
                    let right_node = right_node.unwrap();
                    node.borrow_mut().right = Some(right_node.clone());
                }
                
                total_bst.push(Some(node));
            }
        }
    }
    total_bst
}

fn test_count_maybe_bst(mark: &str, n:i32){
    println!("=====start total maybe {} tree", mark);
    println!("total {}", count_maybe_bst(1, n));
    println!("=====end {} tree", mark);
}

fn test_build_maybe_bst(mark: &str, n:i32){
    println!("=====start total maybe {} tree", mark);
    let total_bst = build_maybe_bst(1, n);
    for bst in total_bst {
        preorder_traverse(&bst);
        println!("");
    }
    println!("=====end {} tree", mark);
}

fn main() {
    test_count_maybe_bst("count maybe bst", 3);
    test_build_maybe_bst("build maybe bst", 3);
}
