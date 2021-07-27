use std::rc::Rc;
use std::mem::swap;
use std::cmp::max;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;

fn post_order_tree_fib(n: u32) -> u32 {
    if n == 1 || n == 2 {
        1
    } else {
        let left = post_order_tree_fib(n - 1);
        let right = post_order_tree_fib(n - 2);
        left + right
    }
}

fn memory_fib(mem:&mut HashMap<u32, u32>, n: u32) -> u32 {
    if n == 1 || n == 2 {
        1
    } else if mem.contains_key(&n) {
        let state = mem.get(&n);
        match state {
            Some(rc_state) => {
                *rc_state
            }
            None => {
                0
            }
        }
    } else {
        let left = post_order_tree_fib(n - 1);
        let right = post_order_tree_fib(n - 2);
        let state = left + right;
        mem.insert(n, state);
        state
    }
}

fn dp_fib(n: u32) -> u32 {
    if n == 1 || n == 2 {
        1
    } else {
        // let mut dp_table = Vec:: // Vec::with_capacity(n as usize);
        let mut dp_table = vec![0; n as usize];
        dp_table[1] = 1;
        dp_table[2] = 1;
        for k in 3..(n as usize) {
            dp_table[k] = dp_table[k-1] + dp_table[k-2];
        }
        dp_table[(n-1) as usize] + dp_table[(n-2) as usize]
    }
}

fn main() {
    let n:u32 = 10;
    let mut mem = HashMap::with_capacity(n as usize);
    println!("{}", post_order_tree_fib(n));
    println!("{}", memory_fib(&mut mem, n));
    println!("{}", dp_fib(n));
}
