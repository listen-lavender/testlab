use std::rc::Rc;
use std::mem::swap;
use std::cmp::max;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;

struct Foo {
    x: u32,
    y: Cell<u32>,
}

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

struct Tree {
    count: u32,
    root: Option<Rc<RefCell<TreeNode>>>,
}

struct FixBSTNode {
    node: Option<Rc<RefCell<TreeNode>>>,
    preverr_node: Option<Rc<RefCell<TreeNode>>>,
    nexterr_node: Option<Rc<RefCell<TreeNode>>>,
}

fn plain_recover_bst(fix_bst_node: FixBSTNode) -> FixBSTNode {
    let mut next_fix_bst_node: FixBSTNode;
    // let node = fix_bst_node.node.as_ref();
    let mut preverr_node = fix_bst_node.preverr_node;
    let mut nexterr_node = fix_bst_node.nexterr_node;
    match fix_bst_node.node {
        Some(rc_node) => {
            let raw_node = rc_node.borrow();
            let mut left_fix_bst_node = FixBSTNode{
                node: None,
                preverr_node: None,
                nexterr_node: None,
            };
            let mut right_fix_bst_node = FixBSTNode{
                node: None,
                preverr_node: None,
                nexterr_node: None,
            };
            match &raw_node.left {
                Some(rc_left_node) => {
                    next_fix_bst_node = FixBSTNode{
                        node: Some(rc_left_node.clone()),
                        preverr_node: preverr_node,
                        nexterr_node: nexterr_node,
                    };
                    left_fix_bst_node = plain_recover_bst(next_fix_bst_node);
                    preverr_node = left_fix_bst_node.preverr_node;
                    nexterr_node = left_fix_bst_node.nexterr_node;
                }
                None => {}
            }
            match left_fix_bst_node.node {
                Some(rc_left_node) => {
                    let raw_left_node = rc_left_node.borrow();
                    if raw_left_node.value > raw_node.value {
                        match preverr_node {
                            Some(_) => {
                                nexterr_node = Some(rc_node.clone());
                            }
                            None => {
                                preverr_node = Some(rc_left_node.clone());
                                nexterr_node = Some(rc_node.clone());
                            }
                        }
                    }
                }
                None => {}
            }
            match &raw_node.right {
                Some(rc_right_node) => {
                    let raw_right_node = rc_right_node.borrow();
                    if raw_node.value > raw_right_node.value {
                        match preverr_node {
                            Some(_) => {
                                nexterr_node = Some(rc_right_node.clone());
                            }
                            None => {
                                preverr_node = Some(rc_node.clone());
                                nexterr_node = Some(rc_right_node.clone());
                            }
                        }
                    }

                    next_fix_bst_node = FixBSTNode{
                        node: Some(rc_right_node.clone()),
                        preverr_node: preverr_node,
                        nexterr_node: nexterr_node,
                    };
                    right_fix_bst_node = plain_recover_bst(next_fix_bst_node);
                    preverr_node = right_fix_bst_node.preverr_node;
                    nexterr_node = right_fix_bst_node.nexterr_node;
                }
                None => {}
            }
            match right_fix_bst_node.node {
                Some(rc_right_node) => {
                    FixBSTNode{
                        node: Some(rc_right_node),
                        preverr_node: preverr_node,
                        nexterr_node: nexterr_node,
                    }
                }
                None => {
                    FixBSTNode{
                        node: Some(rc_node.clone()),
                        preverr_node: preverr_node,
                        nexterr_node: nexterr_node,
                    }
                }
            }
        }
        None => {
            FixBSTNode{
                node: None,
                preverr_node: None,
                nexterr_node: None,
            }
        }
    }
}

fn inorder_traverse(node: &Option<Rc<RefCell<TreeNode>>>) {
    match node {
        Some(raw_node) => {
            inorder_traverse(&raw_node.borrow().left);
            let value = raw_node.borrow().value;
            println!("{}", value);
            inorder_traverse(&raw_node.borrow().right);
        }
        None => {
        }
    }
}

fn init_empty_tree() -> Option<Rc<RefCell<TreeNode>>> {
    return None
}

fn init_onenode_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let root = TreeNode::new(1);
    return Some(root)
}


fn init_leftside_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let node = TreeNode::new(1);
    node.borrow_mut().left = None;
    let root = node;

    let node = TreeNode::new(3);
    node.borrow_mut().left = Some(root);
    let root = node;

    let node = TreeNode::new(2);
    node.borrow_mut().left = Some(root);
    let root = node;

    return Some(root)
}

fn init_rightside_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let node = TreeNode::new(6);
    node.borrow_mut().right = None;
    let root = node;

    let node = TreeNode::new(7);
    node.borrow_mut().right = Some(root);
    let root = node;

    let node = TreeNode::new(5);
    node.borrow_mut().right = Some(root);
    let root = node;

    return Some(root)
}

fn init_sideerr_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let root = TreeNode::new(4);
    
    {
        let left_node = TreeNode::new(2);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(7);
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

        let right_node = TreeNode::new(6);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let node = root.borrow();
        let mut node = node.right.as_ref().unwrap().borrow_mut();
        node.left = Some(left_node);
        node.right = Some(right_node);
    }

    return Some(root)
}

fn init_bothsideerr_tree() -> Option<Rc<RefCell<TreeNode>>> {
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
        let left_node = TreeNode::new(7);
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

        let right_node = TreeNode::new(1);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let node = root.borrow();
        let mut node = node.right.as_ref().unwrap().borrow_mut();
        node.left = Some(left_node);
        node.right = Some(right_node);
    }

    return Some(root)
}

fn init_inbalance_tree() -> Option<Rc<RefCell<TreeNode>>> {
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

fn test_plain_recover_bst(mark: &str, node: Option<Rc<RefCell<TreeNode>>>){
    println!("=====start plain recover {} tree", mark);
    match node {
        Some(rc_node) => {
            let root_node = Some(rc_node.clone());
            inorder_traverse(&root_node);
            println!("=============");
            let fix_bst_node = plain_recover_bst(FixBSTNode{
                node: Some(rc_node.clone()),
                preverr_node: None,
                nexterr_node: None,
            });
            match fix_bst_node.preverr_node {
                Some(rc_preverr_node) => {
                    match fix_bst_node.nexterr_node {
                        Some(rc_nexterr_node) => {
                            let mut raw_preverr_node = rc_preverr_node.borrow_mut();
                            let mut raw_nexterr_node = rc_nexterr_node.borrow_mut();
                            println!("=============");
                            let t = raw_preverr_node.value;
                            raw_preverr_node.value = raw_nexterr_node.value;
                            raw_nexterr_node.value = t;
                            // let receive = |u32| {};
                            // inorder_traverse(&root_node, &receive)
                        }
                        None => {}
                    }
                }
                None => {}
            }
            inorder_traverse(&root_node)
        }
        None => {
            println!("Nothing to plain recover bst")
        }
    }
    println!("=====end {} tree", mark);
}

fn main() {
    let tree_node = init_bothsideerr_tree();
    test_plain_recover_bst("bothsideerr", tree_node);
    let tree_node = init_sideerr_tree();
    test_plain_recover_bst("sideerr", tree_node);
    let tree_node = init_leftside_tree();
    test_plain_recover_bst("leftside", tree_node);
    let tree_node = init_rightside_tree();
    test_plain_recover_bst("rightside", tree_node);
}
