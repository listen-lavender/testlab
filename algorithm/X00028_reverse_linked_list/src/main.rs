use std::rc::Rc;
use std::mem::swap;
use std::cmp::max;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;
use std::thread;

struct LinkedlistNode {
    value: u32,
    next: Option<Rc<RefCell<LinkedlistNode>>>,
}

impl LinkedlistNode {
    fn new(value: u32) -> Rc<RefCell<LinkedlistNode>> {
        Rc::new(RefCell::new(LinkedlistNode {
            value,
            next: None,
        }))
    }
    fn next_node(&self) -> Option<Rc<RefCell<LinkedlistNode>>> {
        match &self.next {
            Some(next_node) => {
                Some(next_node.clone())
            }
            None => {None}
        }
    }
}

fn clone_linkedlist_node(node: &Option<Rc<RefCell<LinkedlistNode>>>) -> Option<Rc<RefCell<LinkedlistNode>>> {
    match node {
        Some(rc_node) => {
            Some(rc_node.clone())
        }
        None => {
            None
        }
    }
}

fn traverse_linkedlist(node: &Option<Rc<RefCell<LinkedlistNode>>>) {
    match node {
        Some(rc_node) => {
            let mut next = Some(rc_node.clone());
            while let Some(rc_node) = next {
                println!("{}", rc_node.borrow().value);
                // thread::sleep_ms(1000);
                next = rc_node.borrow().next_node();
            }
        }
        None => {
        }
    }
}

fn iter_reverse_linkedlist(node: Option<Rc<RefCell<LinkedlistNode>>>, start:i32, end:i32) -> Option<Rc<RefCell<LinkedlistNode>>> {
    match node {
        Some(rc_node) => {
            let mut prev:Option<Rc<RefCell<LinkedlistNode>>> = None;
            let mut next = Some(rc_node.clone());
            let mut head = rc_node.clone();

            let mut reverse_first = rc_node.clone();
            let mut reverse_last = rc_node.clone();
            let mut before_last:Option<Rc<RefCell<LinkedlistNode>>> = None;
            let mut after_first:Option<Rc<RefCell<LinkedlistNode>>> = None;

            let mut ind:i32 = 0;
            while let Some(rc_node) = next {
                // println!("{}<<<<", ind);
                // thread::sleep_ms(1000);
                if start == 0 && ind < end {
                    head = rc_node.clone();
                }
                next = rc_node.borrow().next_node();
                
                if ind == start - 1 {
                    before_last = Some(rc_node.clone());
                }

                if ind == start {
                    reverse_first = rc_node.clone();
                }
                if ind >= start && ind < end {
                    rc_node.borrow_mut().next = clone_linkedlist_node(&prev);
                }
                if ind == end - 1 {
                    reverse_last = rc_node.clone();
                }

                if ind == end {
                    after_first = Some(rc_node.clone());
                }

                prev = Some(rc_node.clone());
                ind = ind + 1;
            }

            match before_last {
                Some(before_last) => {
                    before_last.borrow_mut().next = Some(reverse_last.clone());
                }
                None => {}
            }

            match after_first {
                Some(after_first) => {
                    reverse_first.borrow_mut().next = Some(after_first.clone());
                }
                None => {
                    reverse_first.borrow_mut().next = None;
                }
            }
            Some(head.clone())
        }
        None => {
            None
        }
    }
}

fn recursive_reverse_linkedlist(node: Option<Rc<RefCell<LinkedlistNode>>>) -> Option<Rc<RefCell<LinkedlistNode>>> {
    match node {
        Some(rc_node) => {
            let next = rc_node.borrow().next_node();
            match next {
                Some(rc_next) => {
                    let reverse_last = recursive_reverse_linkedlist(Some(rc_next.clone()));
                    rc_next.borrow_mut().next = Some(rc_node.clone());
                    rc_node.borrow_mut().next = None;
                    reverse_last
                }
                None => {
                    Some(rc_node.clone())
                }
            }
        }
        None => {
            None
        }
    }
}

fn recursive_n_reverse_linkedlist<F>(node: Option<Rc<RefCell<LinkedlistNode>>>, n:usize, link:&mut F) -> Option<Rc<RefCell<LinkedlistNode>>> where F: FnMut(Option<Rc<RefCell<LinkedlistNode>>>, Option<Rc<RefCell<LinkedlistNode>>>) {
    match node {
        Some(rc_node) => {
            let next = rc_node.borrow().next_node();
            match next {
                Some(rc_next) => {
                    if n == 1 {
                        let successor = Some(rc_next.clone());
                        link(None, successor);
                        return Some(rc_node.clone());
                    }
                    let reverse_last = recursive_n_reverse_linkedlist(Some(rc_next.clone()), n-1, link);
                    rc_next.borrow_mut().next = Some(rc_node.clone());
                    link(Some(rc_node.clone()), None); // rc_node.borrow_mut().next = None;
                    reverse_last
                }
                None => {
                    link(Some(rc_node.clone()), None);
                    Some(rc_node.clone())
                }
            }
        }
        None => {
            None
        }
    }
}

fn recursive_m_n_reverse_linkedlist<F>(node: Option<Rc<RefCell<LinkedlistNode>>>, m:usize, n:usize, link:&mut F) -> Option<Rc<RefCell<LinkedlistNode>>> where F: FnMut(Option<Rc<RefCell<LinkedlistNode>>>, Option<Rc<RefCell<LinkedlistNode>>>) {
    if m == 1 {
        return recursive_n_reverse_linkedlist(node, n, link)
    }
    match node {
        Some(rc_node) => {
            let next = rc_node.borrow().next_node();
            match next {
                Some(rc_next) => {
                    let reverse_last = recursive_m_n_reverse_linkedlist(Some(rc_next.clone()), m-1, n-1, link);
                    rc_node.borrow_mut().next = reverse_last;
                    Some(rc_node.clone())
                }
                None => {
                    Some(rc_node.clone())
                }
            }
        }
        None => {
            None
        }
    }
}

fn init_linkedlist(vals:&[u32]) -> Option<Rc<RefCell<LinkedlistNode>>> {
    if vals.len() == 0 {
        return None
    }
    let head = LinkedlistNode::new(vals[0]);
    let mut curr = head.clone();
    let mut next:Rc<RefCell<LinkedlistNode>>;
    for k in 1..vals.len() {
        next = LinkedlistNode::new(vals[k]);
        curr.borrow_mut().next = Some(next.clone());
        curr = next;
    }
    Some(head.clone())
}

fn test_iter_reverse_linkedlist(mark: &str, vals:&[u32], start:i32, end:i32) {
    println!("=====start {} [{}, {}) tree", mark, start, end);
    let head = init_linkedlist(vals);
    traverse_linkedlist(&head);
    println!("-----");
    let head = iter_reverse_linkedlist(head, start, end);
    traverse_linkedlist(&head);
    println!("=====end {} tree", mark);
}

fn test_recursive_reverse_linkedlist(mark: &str, vals:&[u32]) {
    println!("=====start {} [{}, {}) tree", mark, 0, 0);
    let head = init_linkedlist(vals);
    traverse_linkedlist(&head);
    println!("-----");
    let head = recursive_reverse_linkedlist(head);
    traverse_linkedlist(&head);
    println!("=====end {} tree", mark);
}

fn test_recursive_n_reverse_linkedlist(mark: &str, vals:&[u32], n:usize) {
    println!("=====start {} [{}, {}) tree", mark, 0, n);
    let head = init_linkedlist(vals);
    traverse_linkedlist(&head);
    println!("-----");
    let mut g_successor:Option<Rc<RefCell<LinkedlistNode>>> = None;
    let mut link = |node:Option<Rc<RefCell<LinkedlistNode>>>, successor:Option<Rc<RefCell<LinkedlistNode>>>| {
        if successor.is_some() {
            g_successor = successor;
            return
        }
        match node {
            Some(rc_node) => {
                rc_node.borrow_mut().next = clone_linkedlist_node(&g_successor);
            }
            None => {}
        }
    };
    let head = recursive_n_reverse_linkedlist(head, n, &mut link);
    traverse_linkedlist(&head);
    println!("=====end {} tree", mark);
}

fn test_recursive_m_n_reverse_linkedlist(mark: &str, vals:&[u32], m:usize, n:usize) {
    println!("=====start {} [{}, {}) tree", mark, m-1, n);
    let head = init_linkedlist(vals);
    traverse_linkedlist(&head);
    println!("-----");
    let mut g_successor:Option<Rc<RefCell<LinkedlistNode>>> = None;
    let mut link = |node:Option<Rc<RefCell<LinkedlistNode>>>, successor:Option<Rc<RefCell<LinkedlistNode>>>| {
        if successor.is_some() {
            g_successor = successor;
            return
        }
        match node {
            Some(rc_node) => {
                rc_node.borrow_mut().next = clone_linkedlist_node(&g_successor);
            }
            None => {}
        }
    };
    let head = recursive_m_n_reverse_linkedlist(head, m, n, &mut link);
    traverse_linkedlist(&head);
    println!("=====end {} tree", mark);
}

fn main() {
    test_iter_reverse_linkedlist("iter reverse linkedlist", &[1, 2, 3, 4, 5, 6, 7], 0, 7);
    test_iter_reverse_linkedlist("iter reverse linkedlist", &[1, 2, 3, 4, 5, 6, 7], 0, 5);
    test_iter_reverse_linkedlist("iter reverse linkedlist", &[1, 2, 3, 4, 5, 6, 7], 4, 7);
    test_iter_reverse_linkedlist("iter reverse linkedlist", &[1, 2, 3, 4, 5, 6, 7], 2, 5);
    test_recursive_reverse_linkedlist("recursive reverse linkedlist", &[1, 2, 3, 4, 5, 6, 7]);
    test_recursive_n_reverse_linkedlist("recursive n reverse linkedlist", &[1, 2, 3, 4, 5, 6, 7], 7);
    test_recursive_n_reverse_linkedlist("recursive n reverse linkedlist", &[1, 2, 3, 4, 5, 6, 7], 4);
    test_recursive_m_n_reverse_linkedlist("recursive m_n reverse linkedlist", &[1, 2, 3, 4, 5, 6, 7], 3, 5);
    test_recursive_m_n_reverse_linkedlist("recursive m_n reverse linkedlist", &[1, 2, 3, 4, 5, 6, 7], 1, 7);
}
