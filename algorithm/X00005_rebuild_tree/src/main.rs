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

struct Stack<T> {
    eles: Vec<T>,
}

impl<T> Stack<T> {
    pub fn with_capacity(init_size: usize) -> Self {
        Self {
            eles:Vec::with_capacity(init_size),
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.eles.pop()
    }
    pub fn push(&mut self, e: T) {
        self.eles.push(e)
    }
    pub fn size(&mut self) -> usize {
        self.eles.len()
    }
    pub fn top(&mut self) -> Option<&T> {
        self.eles.last()
    }
}

// #[derive(Clone)]
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

struct TreeStore {
    preorder: Vec<u32>,
    pre_start: usize,
    pre_end: usize, 
    inorder: Vec<u32>, 
    in_start: usize, 
    in_end: usize, 
    inmap: HashMap<u32, usize>,
}

struct MaxPathValue {
    value: u32,
    path: Vec<u32>,
    color: u32,
}

struct PathNode {
    node_value: u32,
    value: u32,
    color: u32,
    is_leaf: bool,
    is_exist: bool,
}

fn fast_preorder_traverse(node: Option<Rc<RefCell<TreeNode>>>) {
    let mut stack = Stack::<Rc<RefCell<TreeNode>>>::with_capacity(5);
    let mut curr_node = node;
    loop {
        match curr_node {
            Some(_) => {
                while let Some(rc_curr_node) = curr_node.clone() {
                    println!("{}", rc_curr_node.borrow().value);
                    stack.push(rc_curr_node.clone());
                    curr_node = rc_curr_node.borrow().left.clone();
                }
                if stack.size() > 0 {
                    curr_node = stack.pop();
                    match curr_node {
                        Some(rc_curr_node) => {
                            curr_node = rc_curr_node.borrow().right.clone();
                        }
                        None => {}
                    }
                }
            }
            None => {
                if stack.size() > 0 {
                    curr_node = stack.pop();
                    match curr_node {
                        Some(rc_curr_node) => {
                            curr_node = rc_curr_node.borrow().right.clone();
                        }
                        None => {}
                    }
                } else {
                    break;
                }
            }
        }
    }
}

fn fast_inorder_traverse(node: Option<Rc<RefCell<TreeNode>>>) {
    let mut stack = Stack::<Rc<RefCell<TreeNode>>>::with_capacity(5);
    let mut curr_node = node;
    loop {
        match curr_node {
            Some(_) => {
                while let Some(rc_curr_node) = curr_node.clone() {
                    stack.push(rc_curr_node.clone());
                    curr_node = rc_curr_node.borrow().left.clone();
                }
                if stack.size() > 0 {
                    curr_node = stack.pop();
                    match curr_node {
                        Some(rc_curr_node) => {
                            println!("{}", rc_curr_node.borrow().value);
                            curr_node = rc_curr_node.borrow().right.clone();
                        }
                        None => {}
                    }
                }
            }
            None => {
                if stack.size() > 0 {
                    curr_node = stack.pop();
                    match curr_node {
                        Some(rc_curr_node) => {
                            println!("{}", rc_curr_node.borrow().value);
                            curr_node = rc_curr_node.borrow().right.clone();
                        }
                        None => {}
                    }
                } else {
                    break;
                }
            }
        }
    }
}

fn fast_postorder_traverse(node: Option<Rc<RefCell<TreeNode>>>) {
    let mut stack = Stack::<Rc<RefCell<TreeNode>>>::with_capacity(5);
    let mut curr_node = node;
    loop {
        match curr_node {
            Some(_) => {
                while let Some(rc_curr_node) = curr_node.clone() {
                    stack.push(rc_curr_node.clone());
                    curr_node = rc_curr_node.borrow().left.clone();
                }
                if stack.size() > 0 {
                    let top_node = stack.top();
                    match top_node {
                        Some(rc_top_node) => {
                            let right_node = rc_top_node.borrow().right.clone();
                            match right_node {
                                Some(_) => {
                                    rc_top_node.borrow_mut().right = None;
                                    curr_node = right_node;
                                }
                                None => {
                                    println!("{}", rc_top_node.borrow().value);
                                    curr_node = right_node;
                                    stack.pop();
                                }
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {
                if stack.size() > 0 {
                    let top_node = stack.top();
                    match top_node {
                        Some(rc_top_node) => {
                            let right_node = rc_top_node.borrow().right.clone();
                            match right_node {
                                Some(_) => {
                                    rc_top_node.borrow_mut().right = None;
                                    curr_node = right_node;
                                }
                                None => {
                                    println!("{}", rc_top_node.borrow().value);
                                    curr_node = right_node;
                                    stack.pop();
                                }
                            }
                        }
                        None => {}
                    }
                } else {
                    break;
                }
            }
        }
    }
}

fn plain_preorder_traverse<F>(node: &Option<Rc<RefCell<TreeNode>>>, receive:&mut F) where F: FnMut(u32) {
    match node {
        Some(raw_node) => {
            let value = raw_node.borrow().value;
            println!("{}", value);
            receive(value);
            plain_preorder_traverse(&raw_node.borrow().left, receive);
            plain_preorder_traverse(&raw_node.borrow().right, receive);
        }
        None => {
        }
    }
}

fn plain_inorder_traverse<F>(node: &Option<Rc<RefCell<TreeNode>>>, receive:&mut F) where F: FnMut(u32) {
    match node {
        Some(raw_node) => {
            plain_inorder_traverse(&raw_node.borrow().left, receive);
            let value = raw_node.borrow().value;
            // println!("{}", value);
            receive(value);
            plain_inorder_traverse(&raw_node.borrow().right, receive);
        }
        None => {
        }
    }
}

fn plain_postorder_traverse<F>(node: &Option<Rc<RefCell<TreeNode>>>, receive:&mut F) where F: FnMut(u32) {
    match node {
        Some(raw_node) => {
            plain_postorder_traverse(&raw_node.borrow().left, receive);
            plain_postorder_traverse(&raw_node.borrow().right, receive);
            let value = raw_node.borrow().value;
            println!("{}", value);
            receive(value);
        }
        None => {
        }
    }
}

fn max_path_value(node: &Option<Rc<RefCell<TreeNode>>>, pv: &mut MaxPathValue, color: u32) -> PathNode {
    match node {
        Some(raw_node) => {
            let left_path_node = max_path_value(&raw_node.borrow().left, pv, color + 1);
            let right_path_node = max_path_value(&raw_node.borrow().right, pv, left_path_node.color + 1);
            let left = max(0, left_path_node.value);
            let right = max(0, right_path_node.value);
            if left > right && left_path_node.is_leaf {
                pv.path = Vec::new();
                pv.color = left_path_node.color;
                pv.path.push(left_path_node.node_value);
            }
            if left < right && right_path_node.is_leaf {
                pv.path = Vec::new();
                pv.color = right_path_node.color;
                pv.path.push(right_path_node.node_value);
            }

            let mut pn = PathNode{
                node_value: raw_node.borrow().value,
                value: max(left, right) + raw_node.borrow().value,
                color: max(color, right_path_node.color + 1),
                is_leaf: false,
                is_exist: true,
            };
            if right_path_node.is_exist {
                pn.color = max(color, right_path_node.color + 1);
            } else {
                pn.color = color;
            }
            println!("{}, color: {}", raw_node.borrow().value, pn.color);

            if left_path_node.value == 0 && right_path_node.value == 0 {
                pv.color = pn.color;
                pn.is_leaf = true;
            };
            pv.path.push(raw_node.borrow().value);
            pv.value = max(pv.value, pn.value);
            pn
        }
        None => PathNode {
            node_value: 0,
            value: 0,
            color: 0,
            is_leaf: false,
            is_exist: false,
        }
    }
}

fn build_tree(preorder: &Vec<u32>, pre_start: usize, pre_end: usize, inorder: &Vec<u32>, in_start: usize, in_end: usize, inmap: &HashMap<u32, usize>) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.len() == 0 || inorder.len() == 0 || pre_start > pre_end || in_start > in_end {
        return None
    } else {
        let root = TreeNode::new(preorder[pre_start]);
        let in_root = inmap.get(&root.borrow().value).unwrap();
        let nums_left = in_root - in_start;
        if *in_root > 0 {
            root.borrow_mut().left = build_tree(preorder, pre_start + 1, pre_start + nums_left, inorder, in_start, in_root - 1, inmap);
            root.borrow_mut().right = build_tree(preorder, pre_start + nums_left + 1, pre_end, inorder, in_root + 1, in_end, inmap);
        }
        return Some(root)
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
    let node = TreeNode::new(3);
    node.borrow_mut().left = None;
    let root = node;

    let node = TreeNode::new(2);
    node.borrow_mut().left = Some(root);
    let root = node;

    let node = TreeNode::new(1);
    node.borrow_mut().left = Some(root);
    let root = node;

    return Some(root)
}

fn init_rightside_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let node = TreeNode::new(3);
    node.borrow_mut().right = None;
    let root = node;

    let node = TreeNode::new(2);
    node.borrow_mut().right = Some(root);
    let root = node;

    let node = TreeNode::new(1);
    node.borrow_mut().right = Some(root);
    let root = node;

    return Some(root)
}

fn init_prebalance_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let root = TreeNode::new(1);
    
    {
        let left_node = TreeNode::new(2);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(5);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let mut node = root.borrow_mut();
        node.left = Some(left_node);
        node.right = Some(right_node);
    }

    {
        let left_node = TreeNode::new(3);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(4);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let node = root.borrow();
        let mut node = node.left.as_ref().unwrap().borrow_mut();
        node.left = Some(left_node);
        node.right = Some(right_node);
    }

    {
        let left_node = TreeNode::new(6);
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

fn init_bothside_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let root = TreeNode::new(1);
    
    {
        let left_node = TreeNode::new(2);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(5);
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

fn store_tree(root: Option<Rc<RefCell<TreeNode>>>) -> TreeStore {
    let mut ts = TreeStore {
        preorder: Vec::new(),
        pre_start: 0,
        pre_end: 0, 
        inorder: Vec::new(),
        in_start: 0, 
        in_end: 0, 
        inmap: HashMap::new(),
    };
    let mut prereceive = |x: u32| ts.preorder.push(x);
    plain_preorder_traverse(&root, &mut prereceive);
    if ts.preorder.len() > 0 {
        ts.pre_end = ts.preorder.len()-1;
    }

    let mut inreceive = |x: u32| ts.inorder.push(x);
    plain_inorder_traverse(&root, &mut inreceive);
    if ts.inorder.len() > 0 {
        ts.in_end = ts.inorder.len()-1;
    }

    for (p, v) in ts.inorder.iter().enumerate() {
        ts.inmap.insert(*v, p);
    }

    ts
}

fn bar(mut v:[u32;3])->[u32;3]{
    v[0] = 3;
    assert_eq!([3,2,3], v);
    v
}

fn sum(i:u32, y:u32)-> u32{
    i + y
}

fn dangle<'a>() -> &'a str {
    "hello world"
}

fn test_build_tree(mark: &str, tree: Option<Rc<RefCell<TreeNode>>>){
    println!("=====start init {} tree", mark);
    let ts = store_tree(tree);
    println!("=====end {} tree", mark);
    println!("{:#?} {}, {}", ts.preorder, ts.pre_start, ts.pre_end);
    println!("{:#?} {}, {}, {:#?}", ts.inorder, ts.in_start, ts.in_end, ts.inmap);
    // preorder: &Vec<u32>, pre_start: u32, pre_end: u32, inorder: &Vec<u32>, in_start: u32, in_end: u32, inmap: &HashMap<u32, u32>
    let tree = build_tree(&ts.preorder, ts.pre_start, ts.pre_end, &ts.inorder, ts.in_start, ts.in_end, &ts.inmap);
    println!("=====start rebuild {} tree", mark);
    store_tree(tree.clone());
    println!("=====end {} tree", mark);
    let mut receive = |x: u32| {};
    plain_preorder_traverse(&tree, &mut receive);
    println!("----------");
    fast_preorder_traverse(tree.clone());
    println!("pre----------");
    let mut receive = |x: u32| {println!("{}", x)};
    plain_inorder_traverse(&tree, &mut receive);
    println!("----------");
    fast_inorder_traverse(tree.clone());
    println!("in----------");
    let mut receive = |x: u32| {};
    plain_postorder_traverse(&tree, &mut receive);
    println!("----------");
    fast_postorder_traverse(tree.clone());
    println!("post----------");
}

fn test_tree_max_value_path(mark: &str, tree: Option<Rc<RefCell<TreeNode>>>){
    println!("=====start max_path {} tree", mark);
    let mut mpv = MaxPathValue{
        path: Vec::new(),
        value: 0,
        color: 0,
    };
    max_path_value(&tree, &mut mpv, 1);
    println!("value {}, {:#?}, {}", mpv.value, mpv.path, mpv.color);
    println!("=====end {} tree", mark);
}

// fn dangle() -> &String {
//     let s = String::from("hello world");
//     &s
// }

fn main() {
    test_build_tree("empty", init_empty_tree());
    test_build_tree("onenode", init_onenode_tree());
    test_build_tree("leftside", init_leftside_tree());
    test_build_tree("rightside", init_rightside_tree());
    test_build_tree("bothside", init_bothside_tree());
    test_build_tree("balance", init_prebalance_tree());
}
