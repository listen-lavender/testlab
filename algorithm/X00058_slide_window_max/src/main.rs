use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::min;

struct LinkedlistNode {
    value: i32,
    prev: Option<Rc<RefCell<LinkedlistNode>>>,
    next: Option<Rc<RefCell<LinkedlistNode>>>,
}

impl LinkedlistNode {
    fn new(value: i32) -> Rc<RefCell<LinkedlistNode>> {
        Rc::new(RefCell::new(LinkedlistNode {
            value,
            next: None,
            prev: None,
        }))
    }
    // fn prev_node(&self) -> Option<Rc<RefCell<LinkedlistNode>>> {
    //     match &self.prev {
    //         Some(prev_node) => {
    //             Some(prev_node.clone())
    //         }
    //         None => {None}
    //     }
    // }
    fn next_node(&self) -> Option<Rc<RefCell<LinkedlistNode>>> {
        match &self.next {
            Some(next_node) => {
                Some(next_node.clone())
            }
            None => {None}
        }
    }
}

struct MonotonicQueue {
    head: Option<Rc<RefCell<LinkedlistNode>>>,
    tail: Option<Rc<RefCell<LinkedlistNode>>>,
    total: usize,
    capacity: usize,
}

impl MonotonicQueue{
    fn new(capacity: usize) -> MonotonicQueue {
        MonotonicQueue {
            head: None,
            tail: None,
            total: 0,
            capacity: capacity,
        }
    }
    fn init(node: Rc<RefCell<LinkedlistNode>>, capacity: usize) -> MonotonicQueue {
        node.borrow_mut().prev = None;
        node.borrow_mut().next = None;
        MonotonicQueue {
            head: Some(node.clone()),
            tail: Some(node),
            total: 1,
            capacity: capacity,
        }
    }
    fn get_front(&self) -> i32 {
        match &self.head {
            Some(head_node) => {
                head_node.borrow().value
            }
            None => {
                0 // 有问题，不可能分支，到了就有问题
            }
        }
    }
    fn push(&mut self, node: Rc<RefCell<LinkedlistNode>>, last_front:Option<i32>) {
        if self.total > 0 && last_front.is_some() && self.get_front() == last_front.unwrap() {
             self.pop();
        }
        match &self.tail {
            Some(tail_node) => {
                let mut tail_node = tail_node.clone();
                let mut prev = Some(tail_node.clone());
                while let Some(rc_node) = prev {
                    tail_node = rc_node.clone();
                    if tail_node.borrow().value > node.borrow().value {
                        break;
                    }
                    prev = rc_node.borrow().next_node();
                }
                let prev_value = tail_node.borrow().value;
                let current_value = node.borrow().value;
                if prev_value < current_value {
                    node.borrow_mut().prev = None;
                    node.borrow_mut().next = None;
                    self.head = Some(node.clone());
                    self.tail = Some(node);
                    self.total = 1;
                } else {
                    node.borrow_mut().prev = Some(tail_node.clone());
                    tail_node.borrow_mut().next = Some(node.clone());
                    node.borrow_mut().next = None;
                    self.tail = Some(node);
                    self.total = self.total + 1;
                }
            }
            None => {
                node.borrow_mut().prev = None;
                node.borrow_mut().next = None;
                self.head = Some(node.clone());
                self.tail = Some(node);
                self.total = 1;
            }
        }
    }
    fn pop(&mut self) -> Option<Rc<RefCell<LinkedlistNode>>> {
        match &self.head {
            Some(head_node) => {
                let head_node = head_node.clone();
                self.head = head_node.borrow().next_node();
                self.total = self.total - 1;
                match &self.head {
                    Some(head_node) => {
                        head_node.borrow_mut().prev = None;
                    }
                    None => {
                        self.tail = None;
                    }
                }
                Some(head_node)
            }
            None => {
                None
            }
        }
    }
}

fn slide_window_max(nums:&[i32], k:usize) -> Vec<i32> {
    let mut window_max:Vec<i32> = Vec::new();
    if nums.len() < k {
        return window_max
    }
    let mut left_index_prev:usize = 0;
    let mut window = MonotonicQueue::new(k);
    for index in 0..k {
        window.push(LinkedlistNode::new(nums[index]), None);
    }
    window_max.push(window.get_front());
    for right_index in k..nums.len() {
        left_index_prev = right_index - k;
        window.push(LinkedlistNode::new(nums[right_index]), Some(nums[left_index_prev]));
        window_max.push(window.get_front());
    }
    window_max
}

fn main() {
    let nums:[i32;8] = [1, 3, -1, -3, 5, 3, 6, 7];
    let size:usize = 3;
    let window_max = slide_window_max(&nums, size);
    println!("{:?} window size {} get {:?}", &nums, size, window_max);
}

