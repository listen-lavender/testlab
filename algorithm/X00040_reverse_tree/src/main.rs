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

// fn reverse_tree(node: Option<Rc<RefCell<TreeNode>>>) {
//     match node {
//         Some(raw_node) => {
//             println!("{} ====", &raw_node.borrow().value);
//             match &raw_node.borrow().left {
//                 Some(left_rc_node) => {
//                     reverse_tree(Some(left_rc_node.clone()));

//                     match &raw_node.borrow().right {
//                         Some(right_rc_node) => {
//                             reverse_tree(Some(right_rc_node.clone()));
                            
//                             raw_node.borrow_mut().left = Some(right_rc_node.clone());
//                             raw_node.borrow_mut().right = Some(left_rc_node.clone());
//                         }
//                         None => {
//                             raw_node.borrow_mut().left = None;
//                             raw_node.borrow_mut().right = Some(left_rc_node.clone());
//                         }
//                     }
//                 }
//                 None => {
//                     match &raw_node.borrow().right {
//                         Some(right_rc_node) => {
//                             reverse_tree(Some(right_rc_node.clone()));
//                             raw_node.borrow_mut().left = Some(right_rc_node.clone());
//                             raw_node.borrow_mut().right = None;
//                         }
//                         None => {
//                         }
//                     }
//                 }
//             }
//         }
//         None => {
//         }
//     }
// }

fn reverse_tree(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if node.is_none() {
        return None
    }
    let node = node.unwrap();
    let l = reverse_tree(node.borrow_mut().left.take());
    let r = reverse_tree(node.borrow_mut().right.take());
    node.borrow_mut().left = r;
    node.borrow_mut().right = l;
    Some(node)
}

fn inorder_traverse(node: &Option<Rc<RefCell<TreeNode>>>) {
    match node {
        Some(raw_node) => {
            inorder_traverse(&raw_node.borrow().left);
            let value = raw_node.borrow().value;
            print!("{} ", value);
            inorder_traverse(&raw_node.borrow().right);
        }
        None => {
        }
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

fn test_reverse_tree(mark: &str, tree: Rc<RefCell<TreeNode>>){
    println!("=====start max_path {} tree", mark);
    let root = tree.clone();
    inorder_traverse(&Some(root));
    println!("");
    let root = tree.clone();
    reverse_tree(Some(root));
    let root = tree.clone();
    inorder_traverse(&Some(root));
    println!("");
    println!("=====end {} tree", mark);
}

fn main() {
    test_reverse_tree("reverse tree", init_tree());
}
