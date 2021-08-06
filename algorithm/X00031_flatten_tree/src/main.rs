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

// fn flatten_tree(node: Option<Rc<RefCell<TreeNode>>>) {
//     match node {
//         Some(raw_node) => {
//             println!("{} ====", &raw_node.borrow().value);
//             match &raw_node.borrow().left {
//                 Some(left_rc_node) => {
//                     flatten_tree(Some(left_rc_node.clone()));

//                     match &raw_node.borrow().right {
//                         Some(right_rc_node) => {
//                             flatten_tree(Some(right_rc_node.clone()));
//                             println!("{} ====after", &raw_node.borrow().value);

//                             raw_node.borrow_mut().left = None;
//                             raw_node.borrow_mut().right = Some(left_rc_node.clone());

//                             // let mut left_rc_node = left_rc_node.clone();
//                             // let mut shadow_rc_node:Rc<RefCell<TreeNode>>;
//                             // while let Some(rc_node) = &left_rc_node.borrow().right {
//                             //     shadow_rc_node = rc_node.clone();
//                             //     left_rc_node = shadow_rc_node;
//                             // }

//                             let mut left_rc_node = left_rc_node.clone();
//                             let mut left_node = Some(left_rc_node.clone());
//                             let mut shadow_rc_node:Rc<RefCell<TreeNode>>;
//                             while let Some(rc_node) = &left_node {
//                                 shadow_rc_node = rc_node.clone();
//                                 match &shadow_rc_node.borrow().right {
//                                     Some(right_rc_node) => {
//                                         left_rc_node = right_rc_node.clone();
//                                         left_node = Some(left_rc_node.clone());
//                                     }
//                                     None => {
//                                         left_node = None;
//                                     }
//                                 }
//                             }
//                             left_rc_node.borrow_mut().right = Some(right_rc_node.clone());
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
//                             flatten_tree(Some(right_rc_node.clone()));
//                         }
//                         None => {
//                             println!("{} ====before", &raw_node.borrow().value);
//                         }
//                     }
//                 }
//             }
//         }
//         None => {
//         }
//     }
// }

fn flatten_tree(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if node.is_none() {
        return None
    }
    
    let node = node.unwrap();
    // println!("{} ====", &node.borrow().value);
    let left_node = node.borrow_mut().left.take();
    let right_node = node.borrow_mut().right.take();
    let left_node = flatten_tree(left_node);
    let right_node = flatten_tree(right_node);

    if left_node.is_some() && right_node.is_some() {
        let mut left_rc_node = left_node.unwrap();
        let mut left_node = Some(left_rc_node.clone());
        let mut right_rc_node = right_node.unwrap();
        let mut right_node = Some(right_rc_node.clone());
        node.borrow_mut().right = Some(left_rc_node.clone());

        while let Some(rc_node) = left_node {
            left_rc_node = rc_node.clone();
            left_node = left_rc_node.borrow_mut().right.take();
            if left_node.is_some() {
                let rc_node = left_node.unwrap();
                left_node = Some(rc_node.clone());
                left_rc_node.borrow_mut().right = Some(rc_node.clone());
            }
        }

        node.borrow_mut().left = None;
        left_rc_node.borrow_mut().right = right_node;
        // println!("{} ---both {} {}", &node.borrow().value, left_rc_node.borrow().value, right_rc_node.borrow().value);
        // test_flatten_tree(&node.borrow().value.to_string(), node.clone());
        // test_flatten_tree(&left_rc_node.borrow().value.to_string(), left_rc_node.clone());
        // test_flatten_tree(&right_rc_node.borrow().value.to_string(), right_rc_node.clone());
    } else if left_node.is_some() {
        let mut left_rc_node = left_node.unwrap();
        let mut left_node = Some(left_rc_node.clone());
        node.borrow_mut().left = None;
        node.borrow_mut().right = left_node;
        // println!("{} ---side", &node.borrow().value);
        // test_flatten_tree(&node.borrow().value.to_string(), node.clone());
        // test_flatten_tree(&left_rc_node.borrow().value.to_string(), left_rc_node.clone());
    } else {
        // println!("{} ---", &node.borrow().value);
        // test_flatten_tree(&node.borrow().value.to_string(), node.clone());
    }

    Some(node)
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

fn test_flatten_tree(mark: &str, tree: Rc<RefCell<TreeNode>>){
    println!("=====start {} tree", mark);
    let root = tree.clone();
    flatten_tree(Some(root));
    let mut right_rc_node = tree.clone();
    let mut right_node = Some(right_rc_node.clone());
    while let Some(rc_node) = right_node {
        print!("{} ", rc_node.borrow().value);
        right_node = rc_node.borrow_mut().right.take();
    }
    println!("");
    println!("=====end {} tree", mark);
}


struct Foo;

struct Bar { 
    // foo: Foo,
    foo: RefCell<Foo>,
    msg: String,
}

// impl Foo {
//     fn foo(&mut self, bar: &Bar) { 
//         println!("{}", bar.msg);
//     }
// }

impl Foo {
    fn foo(&mut self, bar: &Bar) {
        println!("{}", bar.msg);
        // let _ = bar.foo.borrow_mut();
    }
}

impl Bar {
    // fn new(foo: Foo) -> Self {
    //     Self { 
    //         foo, 
    //         msg: String::from("Baaaaaar"), 
    //     } 
    // }
    // fn bar(&mut self) {
    //     self.foo.foo(self);
    // }
    fn new(foo: Foo) -> Self {
        Self {
            foo: RefCell::new(foo),
            msg: String::from("Baaaaaar"),
        }
    }
    fn bar(&mut self) {
        let foo = &self.foo;
        foo.borrow_mut().foo(self);
    }
}

fn main() {
    // let mut bar = Bar::new(Foo);
    // bar.bar();
    test_flatten_tree("flatten tree", init_tree());
}
