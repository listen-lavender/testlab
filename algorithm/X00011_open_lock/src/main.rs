use std::rc::Rc;
use std::mem::swap;
use std::cmp::max;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashSet;

struct CodeNode {
    value: [u32;4],
    // trees: Vec<Rc<RefCell<ForestNode>>>,
}

impl CodeNode {
    fn new(value: [u32;4]) -> Rc<RefCell<CodeNode>> {
        Rc::new(RefCell::new(CodeNode {
            value,
            // trees: Vec::new(),
        }))
    }

    fn trees(&self, pass:&mut HashSet<[u32;4]>) -> Vec<Rc<RefCell<CodeNode>>> {
        let mut adj: Vec<Rc<RefCell<CodeNode>>> = Vec::new();
        for (k, v) in self.value.iter().enumerate() {
            // let mut n: [u32;4] = [0;4];
            // for (j, c) in self.value.iter().enumerate() {
            //     if i != j {
            //         n[j] = *c;
            //     }
            // }
            let mut n = self.value;

            // 顺时针
            if *v == 9 {
                n[k] = 0;
            } else {
                n[k] = *v + 1;
            }

            if !pass.contains(&n) {
                pass.insert(n);
                adj.push(CodeNode::new(n));
            }
            
            // 逆时针
            if *v < 1 {
                n[k] = 9;
            } else {
                n[k] = *v - 1;
            }

            if !pass.contains(&n) {
                pass.insert(n);
                adj.push(CodeNode::new(n));
            }
        }
        adj
    }
}

fn min_rotate(node: Option<Rc<RefCell<CodeNode>>>, deadends:&mut HashSet<[u32;4]>, target:[u32;4]) -> u32 {
    let mut queue: Vec<Rc<RefCell<CodeNode>>> = Vec::new();
    // let mut pass: HashSet<[u32;4]> = HashSet::new();
    match node {
        Some(rc_node) => {
            let mut mh:u32 = 1;
            let mut end = false;
            queue.push(rc_node.clone());
            deadends.insert(rc_node.borrow().value);
            while queue.len() > 0 {
                let size = queue.len();
                println!("walk at size {}", size);
                for _ in 0..size {
                    let path_node = queue.remove(0);
                    if path_node.borrow().value == target {
                        println!("end at {:?}", path_node.borrow().value);
                        end = true;
                        break;
                    }
                    for adj in &path_node.borrow().trees(deadends){
                        queue.push(adj.clone());
                    }
                }
                if end {
                    break
                }
                mh = mh + 1;
            }
            if end {
                mh - 1
            } else {
                0
            }
        }
        None => {
            0
        }
    }
}

fn test_min_rotate(mark: &str, tree: Option<Rc<RefCell<CodeNode>>>, deadends:&mut HashSet<[u32;4]>, target:[u32;4]){
    println!("=====start {} forest", mark);
    let mh = min_rotate(tree, deadends, target);
    println!("min rotate {}", mh);
    println!("=====end {} forest", mark);
}

fn main() {
    let mut deadends:HashSet<[u32;4]> = HashSet::new();
    deadends.insert([0, 2, 0, 1]);
    deadends.insert([0, 1, 0, 1]);
    deadends.insert([0, 1, 0, 2]);
    deadends.insert([1, 2, 1, 2]);
    deadends.insert([2, 0, 0, 2]);
    test_min_rotate("0202 with trouble", Some(CodeNode::new([0, 0, 0, 0])), &mut deadends, [0, 2, 0, 2]);

    let mut deadends:HashSet<[u32;4]> = HashSet::new();
    test_min_rotate("0202 without trouble", Some(CodeNode::new([0, 0, 0, 0])), &mut deadends, [0, 2, 0, 2]);

    let mut deadends:HashSet<[u32;4]> = HashSet::new();
    deadends.insert([8, 8, 8, 8]);
    test_min_rotate("0009 without trouble", Some(CodeNode::new([0, 0, 0, 0])), &mut deadends, [0, 0, 0, 9]);

    let mut deadends:HashSet<[u32;4]> = HashSet::new();
    deadends.insert([8, 8, 8, 7]);
    deadends.insert([8, 8, 8, 9]);
    deadends.insert([8, 8, 7, 8]);
    deadends.insert([8, 8, 9, 8]);
    deadends.insert([8, 7, 8, 8]);
    deadends.insert([8, 9, 8, 8]);
    deadends.insert([7, 8, 8, 8]);
    deadends.insert([9, 8, 8, 8]);
    test_min_rotate("8888 never been", Some(CodeNode::new([0, 0, 0, 0])), &mut deadends, [8, 8, 8, 8]);
}
