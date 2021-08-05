use std::rc::Rc;
use std::mem::swap;
use std::cmp::max;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;

struct ForestNode {
    value: Option<u32>,
    trees_curr_index: usize,
    trees: Vec<Rc<RefCell<ForestNode>>>,
}

impl ForestNode {
    fn new(value: u32) -> Rc<RefCell<ForestNode>> {
        Rc::new(RefCell::new(ForestNode {
            value: Some(value),
            trees_curr_index: 0,
            trees: Vec::new(),
        }))
    }
    fn new_list(value: &[u32]) -> Rc<RefCell<ForestNode>> {
        let node = Rc::new(RefCell::new(ForestNode {
            value: None,
            trees_curr_index: 0,
            trees: Vec::with_capacity(value.len()),
        }));
        for v in value {
            let k = ForestNode::new(*v);
            node.borrow_mut().trees.push(k);
        }
        node
    }
}

struct FlattenNestedList {
    stack: Vec<Rc<RefCell<ForestNode>>>,
    root_node: Option<Rc<RefCell<ForestNode>>>,
}

impl FlattenNestedList {
    fn new(node: Rc<RefCell<ForestNode>>) -> FlattenNestedList {
        let mut l = FlattenNestedList {
            stack: Vec::new(),
            root_node: Some(node.clone()),
        };
        l.stack.push(node);
        l
    }

    fn next(&mut self) -> Option<u32> {
        let mut node = self.stack.pop();
        let mut val:Option<u32> = None;
        while let Some(rc_node)=node {
            let children_total = rc_node.borrow().trees.len();
            if children_total > 0 {
                if children_total > rc_node.borrow().trees_curr_index {
                    let curr_index = rc_node.borrow().trees_curr_index;
                    let next_rc_node = rc_node.borrow().trees[curr_index].clone();
                    node = Some(next_rc_node);
                    rc_node.borrow_mut().trees_curr_index = curr_index + 1;
                    self.stack.push(rc_node);
                } else {
                    node = self.stack.pop();
                }
            } else {
                node = None;
                val = rc_node.borrow().value;
            }
        }
        val
    }
}

// (1)

// (2)               (3)          (4)

// (5)  6  (7)       8  (9)       (10)

// (18)    11  12       13  14    (15)  16

// (19)                           17

// 20

// 20 6 11 12 8 13 14 17 16
fn init_forest() -> Rc<RefCell<ForestNode>> {
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

        let mut level3_node = ForestNode::new(7);
        {
            let mut level4_node = ForestNode::new(11);
            level3_node.borrow_mut().trees.push(level4_node);

            let mut level4_node = ForestNode::new(12);
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

    return level1_node
}

fn test_flatten_nested_list(mark: &str, root: Rc<RefCell<ForestNode>>){
    println!("=====start {}", mark);
    let mut l = FlattenNestedList::new(root);
    while let Some(n) = l.next() {
        print!("{} ", n);
    }
    println!("");
    println!("=====end {}", mark);
}

fn main() {
    test_flatten_nested_list("nested list", init_forest())
}
