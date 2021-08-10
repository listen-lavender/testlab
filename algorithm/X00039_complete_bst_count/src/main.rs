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

    // fn left_node(&mut self) -> Option<Rc<RefCell<TreeNode>>> {
    //     let left_node = self.left.take();
    //     if left_node.is_some() {
    //         let left_node = left_node.unwrap();
    //         self.left = Some(left_node.clone());
    //         Some(left_node)
    //     } else {
    //         None
    //     }
    // }

    // fn right_node(&mut self) -> Option<Rc<RefCell<TreeNode>>> {
    //     let right_node = self.right.take();
    //     if right_node.is_some() {
    //         let right_node = right_node.unwrap();
    //         self.right = Some(right_node.clone());
    //         Some(right_node)
    //     } else {
    //         None
    //     }
    // }
}

fn complete_bst_count(root: Option<Rc<RefCell<TreeNode>>>) -> u32 {
    if root.is_none() {
        return 0
    }
    let mut left_height:u32 = 0;
    let mut right_height:u32 = 0;
    
    let rc_node = root.unwrap();
    
    let mut root = Some(rc_node.clone());
    while let Some(next_rc_node) = root {
        left_height = left_height + 1;
        root = next_rc_node.borrow().left_node();
    }

    let mut root = Some(rc_node.clone());
    while let Some(next_rc_node) = root {
        right_height = right_height + 1;
        root = next_rc_node.borrow().right_node();
    }

    if left_height == right_height {
        let base: u32 = 2;
        base.pow(left_height) - 1
    } else {
        let left_node = rc_node.borrow().left_node();
        let right_node = rc_node.borrow().right_node();
        complete_bst_count(left_node) + 1 + complete_bst_count(right_node)
    }
}

fn init_perfect_bst_tree() -> Option<Rc<RefCell<TreeNode>>> {
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

fn init_complete_bst_tree() -> Option<Rc<RefCell<TreeNode>>> {
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
        let left_node = TreeNode::new(7);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let node = root.borrow();
        let mut node = node.right.as_ref().unwrap().borrow_mut();
        node.left = Some(left_node);
        node.right = None;
    }

    return Some(root)
}

fn test_complete_bst_count(mark: &str, root: Option<Rc<RefCell<TreeNode>>>){
    println!("=====start total maybe {} tree", mark);
    println!("{}", complete_bst_count(root));
    println!("=====end {} tree", mark);
}

fn main() {
    test_complete_bst_count("complete bst count", init_complete_bst_tree());
    test_complete_bst_count("perfect bst count", init_perfect_bst_tree());
}
