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

fn tree_node_to_string(node: &Option<Rc<RefCell<TreeNode>>>) -> String {
    match node {
        Some(rc_node) => {
            rc_node.borrow().value.to_string()
        }
        None => {
            "#".to_string()
        }
    }
}

fn same_subtree<F>(node: &Option<Rc<RefCell<TreeNode>>>, receive:&mut F) -> String where F: FnMut(String) {
    let mut mark = tree_node_to_string(node);
    match node {
        Some(rc_node) => {
            let left = same_subtree(&rc_node.borrow().left, receive);
            let right = same_subtree(&rc_node.borrow().right, receive);
            mark = mark + &left;
            mark = mark + &right;
        }
        None => {
        }
    }
    if mark != "#" {
        receive(mark.clone());
    }
    mark
}

fn init_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let root = TreeNode::new(1);
    
    {
        let left_node = TreeNode::new(2);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(3);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let mut node = root.borrow_mut();
        node.left = Some(left_node);
        node.right = Some(right_node);
    }

    {
        let left_node = TreeNode::new(4);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let node = root.borrow();
        let mut node = node.left.as_ref().unwrap().borrow_mut();
        node.left = Some(left_node);
        node.right = None;
    }

    {
        let left_node = TreeNode::new(2);
        left_node.borrow_mut().left = Some(TreeNode::new(4));
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(4);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let node = root.borrow();
        let mut node = node.right.as_ref().unwrap().borrow_mut();
        node.left = Some(left_node);
        node.right = Some(right_node);
    }

    return Some(root)
}

fn test_same_subtree(mark: &str, tree: Option<Rc<RefCell<TreeNode>>>){
    println!("=====start {} tree", mark);
    let mut repeat:HashMap<String, u32> = HashMap::new();
    let mut receive = |mark: String| {
        if repeat.contains_key(&mark) {
            let acc = repeat[&mark];
            repeat.insert(mark, acc+1);
        } else {
            repeat.insert(mark, 1);
        }
    };
    same_subtree(&tree, &mut receive);
    for (key, value) in repeat {
        if value > 1 {
            println!("{}", key);
        }
    }
    println!("=====end {} tree", mark);
}

fn main() {
    test_same_subtree("same subtree", init_tree());
}
