use std::mem::swap;
use std::cmp::max;
use std::thread;
use std::str::Chars;
use std::collections::HashMap;
use std::collections::HashSet;

fn lms<F>(nums: &[u32], state: &mut [u32], receive:&mut F) where F: FnMut(u32){
    for i in 0..nums.len() {
        state[i] = 1;
        for j in 0..i {
            if nums[j] < nums[i] {
                state[i] = max(state[j] + 1, state[i]);
            }
        }
    }
}

fn binary_search_lms(s: &str, i:usize, t: &str, j:usize) {
}

fn main() {
    let mut subsequence:Vec<u32> = Vec::new();
    let mut collect = |x: u32| subsequence.push(x);

    let nums:[u32;8] = [10, 9, 2, 5, 3, 7, 101, 18];
    let mut state:Vec<u32> = vec![0;nums.len()];
    lms(&nums, &mut state, &mut collect);
    let mut m:u32 = 0;
    for k in &state {
        m = max(m, *k);
    }
    println!("{:?} {:?} monofonic substr {}", &nums, &state, m);
}
