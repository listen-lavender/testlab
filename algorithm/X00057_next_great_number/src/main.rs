use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::min;

fn next_great_number(nums:&[u32]) -> Vec<i32> {
    let mut behind_heights:Vec<u32> = Vec::new();
    let mut next_higher:Vec<i32> = vec![-1;nums.len()];
    for index in (0..nums.len()).rev() {
        while behind_heights.len() > 0 && behind_heights[behind_heights.len()-1] < nums[index] {
            behind_heights.pop();
        }
        if behind_heights.len() == 0 {
            next_higher[index] = -1;
        } else {
            next_higher[index] = behind_heights[behind_heights.len()-1] as i32;
        }
        behind_heights.push(nums[index]);
    }
    next_higher
}

fn main() {
    let nums:[u32;5] = [2,1,2,4,3];
    let next_higher = next_great_number(&nums);
    println!("{:?} {:?}", nums, next_higher);
}

