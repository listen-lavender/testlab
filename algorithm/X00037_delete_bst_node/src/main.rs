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

    fn left_node(&self) -> Option<Rc<RefCell<TreeNode>>> {
        match &self.left {
            Some(left_node) => {
                Some(left_node.clone())
            }
            None => {None}
        }
    }

    fn right_node(&self) -> Option<Rc<RefCell<TreeNode>>> {
        match &self.right {
            Some(right_node) => {
                Some(right_node.clone())
            }
            None => {None}
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

fn insert_bst_node(node: Option<Rc<RefCell<TreeNode>>>, value: u32) -> Rc<RefCell<TreeNode>> {
    match node {
        Some(rc_node) => {
            if rc_node.borrow().value > value {
                let left_node = rc_node.borrow().left_node();
                let left_rc_node = insert_bst_node(left_node, value);
                rc_node.borrow_mut().left = Some(left_rc_node.clone());
                rc_node
            } else if rc_node.borrow().value < value {
                let right_node = rc_node.borrow().right_node();
                let right_rc_node = insert_bst_node(right_node, value);
                rc_node.borrow_mut().right = Some(right_rc_node.clone());
                rc_node
            } else {
                rc_node
            }
        }
        None => {
            TreeNode::new(value)
        }
    }
}

fn update_bst_node(node: Option<Rc<RefCell<TreeNode>>>, old_value: u32, new_value: u32) -> bool {
    match node {
        Some(rc_node) => {
            let mut updated = false;
            if rc_node.borrow().value == old_value {
                rc_node.borrow_mut().value = new_value;
                updated = true;
            }
            if !updated && rc_node.borrow().value > old_value {
                updated = updated || update_bst_node(rc_node.borrow().left_node(), old_value, new_value);
            }
            if !updated && rc_node.borrow().value < old_value {
                updated = updated || update_bst_node(rc_node.borrow().right_node(), old_value, new_value);
            }
            updated
        }
        None => {
            false
        }
    }
}

fn find_bst_node(node: Option<Rc<RefCell<TreeNode>>>, value: u32) -> Option<Rc<RefCell<TreeNode>>> {
    match node {
        Some(rc_node) => {
            if rc_node.borrow().value > value {
                find_bst_node(rc_node.borrow().left_node(), value)
            } else if rc_node.borrow().value < value  {
                find_bst_node(rc_node.borrow().right_node(), value)
            } else {
                Some(rc_node.clone())
            }
        }
        None => {
            None
        }
    }
}

fn find_right_min_node(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if node.is_none() {
        return node;
    }
    let mut min_node:Option<Rc<RefCell<TreeNode>>> = None;
    let mut root = node;
    while let Some(rc_node) = root {
        min_node = Some(rc_node.clone());
        root = rc_node.borrow().left_node();
    }
    min_node
}

fn delete_bst_node(node: Option<Rc<RefCell<TreeNode>>>, value: u32) -> Option<Rc<RefCell<TreeNode>>> {
    match node {
        Some(rc_node) => {
            if rc_node.borrow().value > value {
                rc_node.borrow_mut().left = delete_bst_node(rc_node.borrow().left_node(), value);
                Some(rc_node.clone())
            } else if rc_node.borrow().value < value {
                rc_node.borrow_mut().right = delete_bst_node(rc_node.borrow().right_node(), value);
                Some(rc_node.clone())
            } else {
                let left_node = rc_node.borrow().left_node();
                let right_node = rc_node.borrow().right_node();
                if left_node.is_none() && right_node.is_none() {
                    None
                } else if left_node.is_none() {
                    right_node
                } else if right_node.is_none() {
                    left_node
                } else {
                    let right_min_node = find_right_min_node(right_node);
                    let right_min_node = right_min_node.unwrap();
                    rc_node.borrow_mut().value = right_min_node.borrow().value;
                    right_min_node.borrow_mut().value = value;
                    rc_node.borrow_mut().right = delete_bst_node(rc_node.borrow().right_node(), value);
                    Some(rc_node.clone())
                }
            }
        }
        None => {
            None
        }
    }
}

fn init_perfect_bst_tree() -> Rc<RefCell<TreeNode>> {
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

fn init_complete_bst_tree() -> Rc<RefCell<TreeNode>> {
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

        let node = root.borrow();
        let mut node = node.right.as_ref().unwrap().borrow_mut();
        node.left = Some(left_node);
        node.right = None;
    }

    return root
}

fn test_insert_bst_node(mark: &str, root: Rc<RefCell<TreeNode>>, value:u32){
    println!("=====start {} tree {}", mark, value);
    inorder_traverse(&Some(root.clone()));
    println!("");
    insert_bst_node(Some(root.clone()), value);
    inorder_traverse(&Some(root.clone()));
    println!("");
    println!("=====end {} tree", mark);
}

fn test_update_bst_node(mark: &str, root: Rc<RefCell<TreeNode>>, old_value:u32, new_value:u32){
    println!("=====start {} tree", mark);
    inorder_traverse(&Some(root.clone()));
    println!("");
    update_bst_node(Some(root.clone()), old_value, new_value);
    inorder_traverse(&Some(root.clone()));
    println!("");
    println!("=====end {} tree", mark);
}

fn test_delete_bst_node(mark: &str, root: Rc<RefCell<TreeNode>>, value:u32){
    println!("=====start {} tree", mark);
    inorder_traverse(&Some(root.clone()));
    println!("");
    delete_bst_node(Some(root.clone()), value);
    inorder_traverse(&Some(root.clone()));
    println!("");
    println!("=====end {} tree", mark);
}

fn main() {
    test_insert_bst_node("insert complete bst count", init_complete_bst_tree(), 7);
    test_update_bst_node("update perfect bst count", init_perfect_bst_tree(), 1, 0);
    test_update_bst_node("update perfect bst count", init_perfect_bst_tree(), 7, 8);
    test_delete_bst_node("delete perfect bst count", init_perfect_bst_tree(), 4);
    test_delete_bst_node("delete perfect bst count", init_perfect_bst_tree(), 7);
}
