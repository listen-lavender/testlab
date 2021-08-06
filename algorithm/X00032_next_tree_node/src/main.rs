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
    next: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: u32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
            next: None,
        }))
    }
}

fn next_tree_node(node_left: Option<Rc<RefCell<TreeNode>>>, node_right: Option<Rc<RefCell<TreeNode>>>) {
    if node_left.is_none() && node_right.is_none() {
        return
    }
    if node_left.is_none() {
        let tree = node_right.unwrap();
        let mut left_node = tree.borrow_mut().left.take();
        let mut right_node = tree.borrow_mut().right.take();
        if left_node.is_some() {
            let rc_node = left_node.unwrap();
            tree.borrow_mut().left = Some(rc_node.clone());
            left_node = Some(rc_node);
        }
        if right_node.is_some() {
            let rc_node = right_node.unwrap();
            tree.borrow_mut().right = Some(rc_node.clone());
            right_node = Some(rc_node);
        }
        next_tree_node(left_node, right_node);
    } else {
        let node_left = node_left.unwrap();
        let node_right = node_right.unwrap();
        node_left.borrow_mut().next = Some(node_right.clone());
        

        let mut node_left_left = node_left.borrow_mut().left.take();
        if node_left_left.is_some() {
            let rc_node = node_left_left.unwrap();
            node_left.borrow_mut().left = Some(rc_node.clone());
            node_left_left = Some(rc_node);
        }
        let mut node_left_right =  node_left.borrow_mut().right.take();
        let mut shadow_node_left_right = None;
        if node_left_right.is_some() {
            let rc_node = node_left_right.unwrap();
            node_left.borrow_mut().right = Some(rc_node.clone());
            node_left_right = Some(rc_node.clone());
            shadow_node_left_right = Some(rc_node);
        }
        let mut node_right_left = node_right.borrow_mut().left.take();
        let mut shadow_node_right_left = None;
        if node_right_left.is_some() {
            let rc_node = node_right_left.unwrap();
            node_right.borrow_mut().left = Some(rc_node.clone());
            node_right_left = Some(rc_node.clone());
            shadow_node_right_left = Some(rc_node);
        }
        let mut node_right_right = node_right.borrow_mut().right.take();
        if node_right_right.is_some() {
            let rc_node = node_right_right.unwrap();
            node_right.borrow_mut().right = Some(rc_node.clone());
            node_right_right = Some(rc_node);
        }

        next_tree_node(node_left_left, node_left_right);
        next_tree_node(node_right_left, node_right_right);
        next_tree_node(shadow_node_left_right, shadow_node_right_left);
    }
}

fn init_tree() -> Rc<RefCell<TreeNode>> {
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

    return root
}

// fn inorder_traverse(node: &Option<Rc<RefCell<TreeNode>>>) {
//     match node {
//         Some(raw_node) => {
//             inorder_traverse(&raw_node.borrow().left);
//             let value = raw_node.borrow().value;
//             print!("{} ", value);
//             inorder_traverse(&raw_node.borrow().right);
//         }
//         None => {
//         }
//     }
// }

fn test_next_tree_node(mark: &str, tree: Rc<RefCell<TreeNode>>){
    println!("=====start max_path {} tree", mark);
    let mut left_node = tree.borrow_mut().left.take();
    let mut right_node = tree.borrow_mut().right.take();
    if left_node.is_some() {
        let rc_node = left_node.unwrap();
        tree.borrow_mut().left = Some(rc_node.clone());
        left_node = Some(rc_node);
    }
    if right_node.is_some() {
        let rc_node = right_node.unwrap();
        tree.borrow_mut().right = Some(rc_node.clone());
        right_node = Some(rc_node);
    }
    next_tree_node(left_node, right_node);

    let mut tree = Some(tree);
    while let Some(rc_node) = tree {
        let mut list = Some(rc_node.clone());
        while let Some(next_rc_node) = list {
            print!("{} ", next_rc_node.borrow().value);
            list = next_rc_node.borrow_mut().next.take();
            if list.is_some() {
                let node = list.unwrap();
                list = Some(node.clone());
                next_rc_node.borrow_mut().next = Some(node.clone());
            }
        }
        tree = rc_node.borrow_mut().left.take();
        if tree.is_some() {
            let node = tree.unwrap();
            tree = Some(node.clone());
            rc_node.borrow_mut().left = Some(node.clone());
        }
    }
    println!("");
    println!("=====end {} tree", mark);
}

fn main() {
    test_next_tree_node("flatten tree", init_tree());
}
