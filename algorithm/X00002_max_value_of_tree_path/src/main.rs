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

fn preorder_traverse<F>(node: &Option<Rc<RefCell<TreeNode>>>, receive:&mut F) where F: FnMut(u32) {
    match node {
        Some(raw_node) => {
            let value = raw_node.borrow().value;
            println!("{}", value);
            receive(value);
            preorder_traverse(&raw_node.borrow().left, receive);
            preorder_traverse(&raw_node.borrow().right, receive);
        }
        None => {
        }
    }
}

fn inorder_traverse<F>(node: &Option<Rc<RefCell<TreeNode>>>, receive:&mut F) where F: FnMut(u32) {
    match node {
        Some(raw_node) => {
            inorder_traverse(&raw_node.borrow().left, receive);
            let value = raw_node.borrow().value;
            receive(value);
            inorder_traverse(&raw_node.borrow().right, receive);
        }
        None => {
        }
    }
}

fn postorder_traverse<F>(node: &Option<Rc<RefCell<TreeNode>>>, receive:&mut F) where F: FnMut(u32) {
    match node {
        Some(raw_node) => {
            postorder_traverse(&raw_node.borrow().left, receive);
            postorder_traverse(&raw_node.borrow().right, receive);
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

fn init_balance_tree() -> Option<Rc<RefCell<TreeNode>>> {
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
    preorder_traverse(&root, &mut prereceive);
    if ts.preorder.len() > 0 {
        ts.pre_end = ts.preorder.len()-1;
    }

    let mut inreceive = |x: u32| ts.inorder.push(x);
    inorder_traverse(&root, &mut inreceive);
    if ts.inorder.len() > 0 {
        ts.in_end = ts.inorder.len()-1;
    }

    for (p, v) in ts.inorder.iter().enumerate() {
        ts.inmap.insert(*v, p);
    }

    ts
}

// TreeNode buildTree(int[] preorder, int preStart, int preEnd,
//     int[] inorder, int inStart, int inEnd, Map<Integer, Integer> inMa
//     p) {

// fn bst_search(node:&Rc<RefCell<TreeNode>>,key:u32) -> Option<String>{
//     let mut result:Option<String> = None;
//     //println!("search {} {}",key,node.borrow().key);
//     if key == node.borrow().key {
//         if node.borrow().vaild {
//             return Some(node.borrow().value.clone());
//         }else {
//             return None;
//         }
//     }
//     else if key < node.borrow().key{
//         if let Some(ref n) = node.borrow().left {
//             result = bst_search(n,key);
//         }
//     }
//     else {
//         if let Some(ref n) = node.borrow().right {
//             result = bst_search(n,key);
//         }
//     }
//     //println!("res {:?}",result);
//     result
// }

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
    store_tree(tree);
    println!("=====end {} tree", mark);
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
    let foo = Foo { x: 1, y: Cell::new(3)};
    assert_eq!(1, foo.x);
    assert_eq!(3, foo.y.get());
    foo.y.set(5);
    assert_eq!(5, foo.y.get());

    let x = RefCell::new(vec![1, 2, 3]);
    println!("{:?}", x.borrow());
    x.borrow_mut().push(5);
    println!("{:?}", x.borrow());

    let mut y = vec![1, 2, 3];
    println!("{:?}", y);
    y.push(5);
    println!("{:?}", y);

    let v = [1,2,3];
    assert_eq!([3,2,3], bar(v));
    assert_eq!([1,2,3], v);

    let x = 5;
    let y = x;
    println!("x: {}", x);

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &s;
    // println!("{}", s);
    // println!("{}", s);
    println!("{}", r1);
    println!("{}", r1);
    println!("{}", s);

    let mut a = String::from("hello a");
    let mut b = String::from("hello b");
    a = b;
    println!("{}", a);

    let a = 1;
    let b = 2;
    println!("{} {}", a, b);

    let mut a = true;
    let mut b = false;
    println!("{} {}", a, b);

    // let reference_to_nothing = dangle();
    // println!("{}", reference_to_nothing);

    let x = Rc::new("hello");
    println!("{:?}", x.chars());

    println!("Hello, world!");

    test_build_tree("empty", init_empty_tree());
    test_build_tree("onenode", init_onenode_tree());
    test_build_tree("leftside", init_leftside_tree());
    test_build_tree("rightside", init_rightside_tree());
    test_build_tree("bothside", init_bothside_tree());
    test_build_tree("balance", init_balance_tree());
    test_tree_max_value_path("empty", init_empty_tree());
    test_tree_max_value_path("onenode", init_onenode_tree());
    test_tree_max_value_path("leftside", init_leftside_tree());
    test_tree_max_value_path("rightside", init_rightside_tree());
    test_tree_max_value_path("bothside", init_bothside_tree());
    test_tree_max_value_path("balance", init_balance_tree());
}

// This paper summarizes the relation of the three different array through the analysis of getting three different array from the same tree using three different algorithm: preorder traversal, inorder traversal and postorder traversal. 

// fn bst_search(node:&Rc<RefCell<TreeNode>>,key:u32) -> Option<String>{
//     let mut result:Option<String> = None;
//     //println!("search {} {}",key,node.borrow().key);
//     if key == node.borrow().key {
//         if node.borrow().vaild {
//             return Some(node.borrow().value.clone());
//         }else {
//             return None;
//         }
//     }
//     else if key < node.borrow().key{
//         if let Some(ref n) = node.borrow().left {
//             result = bst_search(n,key);
//         }
//     }
//     else {
//         if let Some(ref n) = node.borrow().right {
//             result = bst_search(n,key);
//         }
//     }
//     //println!("res {:?}",result);
//     result
// }

// fn do_delete(node:&Rc<RefCell<TreeNode>>,key:u32){
//     let mut node1 = node.borrow_mut();
//     if key == node1.key {
//         node1.vaild = false;
//     }
//     else if key < node1.key{
//         if let Some(ref n) = node1.left {
//             do_delete(n,key);
//         }
//     }
//     else {
//         if let Some(ref n) = node1.right {
//             do_delete(n,key);
//         }
//     }
// }


// fn do_insert(node:&Rc<RefCell<TreeNode>>,key:u32,value:String) {
//     let mut node1 = node.borrow_mut();
//     //println!("doinsert {} {} {}",key,value,node1.key);
//     if key < node1.key {
//         match node1.left {
//             None => {
//                 node1.left = Some(TreeNode::new(key,value));
//             },
//             Some(ref n) => {
//                 do_insert(n,key,value);
//             }
//         }
//     }
//     else if key > node1.key {
//         match node1.right {
//             None => {
//                 node1.right = Some(TreeNode::new(key,value));
//             },
//             Some(ref n) => {
//                 do_insert(n,key,value);
//             }
//         }
//     }else{
//         node1.vaild = true;
//         node1.value = value;
//     }
// }

// impl Bst {

//     pub fn new() -> Bst {
//         Bst {
//             count:0,
//             root:None
//         }
//     }

//     pub fn insert(&mut self,key:u32,value:String){
//         if let Some(_) = self.bst_get(key) {
//             return;
//         }
//         match self.root {
//             None => {
//                 self.root = Some(TreeNode::new(key,value));
//             },
//             Some(ref n) => {
//                 do_insert(&n,key,value);
//             }
//         }
//         self.count = self.count+1;
//     }

//     pub fn bst_get(&self,key:u32) -> Option<String>{
//         match self.root {
//             None => None,
//             Some(ref n) => bst_search(n,key)
//         }
//     }


//     pub fn bst_delete(&self,key:u32){
//         if let None = self.bst_get(key) {
//             return;
//         }
//         match self.root {
//             None => {},
//             Some(ref n) => {
//                 do_delete(n,key)
//             }
//         };
//     }

// }
