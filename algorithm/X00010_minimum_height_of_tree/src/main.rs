use std::rc::Rc;
use std::mem::swap;
use std::cmp::max;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;

struct ForestNode {
    value: u32,
    trees: Vec<Rc<RefCell<ForestNode>>>,
}

impl ForestNode {
    fn new(value: u32) -> Rc<RefCell<ForestNode>> {
        Rc::new(RefCell::new(ForestNode {
            value,
            trees: Vec::new(),
        }))
    }
}

fn min_height(node: Option<Rc<RefCell<ForestNode>>>) -> u32 {
    let mut queue: Vec<Rc<RefCell<ForestNode>>> = Vec::new();
    match node {
        Some(rc_node) => {
            let mut mh:u32 = 1;
            let mut end = false;
            queue.push(rc_node.clone());
            while queue.len() > 0 {
                let size = queue.len();
                println!("walk at size {}", size);
                for _ in 0..size {
                    let path_node = queue.remove(0);
                    if path_node.borrow().trees.len() == 0 {
                        println!("end at {}", path_node.borrow().value);
                        end = true;
                        break;
                    }
                    for adj in &path_node.borrow().trees{
                        queue.push(adj.clone());
                    }
                }
                if end {
                    break
                }
                mh = mh + 1;
            }
            mh
        }
        None => {
            0
        }
    }
}

// 1

// 2             3          4

// 5  6  7       8  9       10

// 18    11  12     13  14  15  16

// 19                       17

// 20
fn init_forest() -> Option<Rc<RefCell<ForestNode>>> {
    let level1_node = ForestNode::new(1);

    let level2_node = ForestNode::new(2);
    {
        let mut level3_node = ForestNode::new(5);
        {
            let mut level4_node = ForestNode::new(18);
            {
                let mut level5_node = ForestNode::new(19);
                level5_node.borrow_mut().trees.push(ForestNode::new(20));
                level4_node.borrow_mut().trees.push(level5_node);
            }
            level3_node.borrow_mut().trees.push(level4_node);
        }
        level2_node.borrow_mut().trees.push(level3_node);

        level2_node.borrow_mut().trees.push(ForestNode::new(6));

        let mut level3_node = ForestNode::new(9);
        {
            let mut level4_node = ForestNode::new(13);
            level3_node.borrow_mut().trees.push(level4_node);

            let mut level4_node = ForestNode::new(14);
            level3_node.borrow_mut().trees.push(level4_node);
        }
        level2_node.borrow_mut().trees.push(level3_node);
    }
    level1_node.borrow_mut().trees.push(level2_node);

    let level2_node = ForestNode::new(3);
    level2_node.borrow_mut().trees.push(ForestNode::new(8));
    {
        let mut level3_node = ForestNode::new(9);
        {
            let mut level4_node = ForestNode::new(13);
            level3_node.borrow_mut().trees.push(level4_node);

            let mut level4_node = ForestNode::new(14);
            level3_node.borrow_mut().trees.push(level4_node);
        }
        level2_node.borrow_mut().trees.push(level3_node);
    }
    level1_node.borrow_mut().trees.push(level2_node);

    let level2_node = ForestNode::new(4);
    {
        let mut level3_node = ForestNode::new(10);
        {
            let mut level4_node = ForestNode::new(15);
            level4_node.borrow_mut().trees.push(ForestNode::new(17));
            level3_node.borrow_mut().trees.push(level4_node);

            let mut level4_node = ForestNode::new(16);
            level3_node.borrow_mut().trees.push(level4_node);
        }
        level2_node.borrow_mut().trees.push(level3_node);
    }
    level1_node.borrow_mut().trees.push(level2_node);

    return Some(level1_node)
}

fn test_min_height_forest(mark: &str, tree: Option<Rc<RefCell<ForestNode>>>){
    println!("=====start max_path {} forest", mark);
    let mh = min_height(tree);
    println!("min height {}", mh);
    println!("=====end {} forest", mark);
}

fn main() {
    test_min_height_forest("forest", init_forest())
}
