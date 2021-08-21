use std::mem::swap;
use std::cmp::max;
use std::cmp::min;
use std::thread;
use std::str::Chars;
use std::collections::HashMap;
use std::collections::HashSet;

fn find_max(heights: &[u32]) -> u32 {
    let mut m:u32 = 0;
    for n in heights {
        if m == 0 {
            m = *n;
        } else {
            m = max(m, *n);
        }
    }
    m
}

fn trap(heights: &[u32]) -> u32 {
    let mut total:u32 = 0;
    for (index, n) in heights.iter().enumerate() {
        let left_max_height = find_max(&heights[0..index+1]);
        let right_max_height = find_max(&heights[index..]);
        total = total + min(left_max_height, right_max_height) - n;
    }
    total
}

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Bucket {
    left_max_height:u32,
    right_max_height:u32,
}

fn trap_memo(heights: &[u32]) -> u32 {
    let mut buckets:HashMap<usize, Bucket> = HashMap::new();
    let mut rain:u32 = 0;
    for (index, n) in heights.iter().enumerate() {
        rain = max(rain, *n);
        let b = Bucket{
            left_max_height:rain,
            right_max_height:0,
        };
        buckets.insert(index, b);
    }
    let mut rain:u32 = 0;
    // println!("all {}", heights.len());
    for index in (0..12).rev() {
        // println!("{}, {}", index, rain);
        rain = max(rain, heights[index]);
        let b = buckets.get(&index);
        match b {
            Some(b) => {
                let b = Bucket{
                    left_max_height:b.left_max_height,
                    right_max_height:rain,
                };
                // println!("{}, {}", b.left_max_height, b.right_max_height);
                buckets.insert(index, b);
            }
            None => {
                // println!("{}, {}", 0, 0);
            }
        }
    }
    // println!("{:?}", buckets);
    let mut total:u32 = 0;
    for (index, n) in heights.iter().enumerate() {
        let b = buckets.get(&index);
        match b {
            Some(b) => {
                total = total + min(b.left_max_height, b.right_max_height) - n;
            }
            None => {}
        }
    }
    total
}

fn trap_best(heights: &[u32]) -> u32 {
    let mut left_index:usize = 0;
    let mut right_index:usize = heights.len() - 1;
    let mut left_max:u32 = 0;
    let mut right_max:u32 = 0;
    let mut total:u32 = 0;
    loop {
        if left_index > right_index {
            break
        }
        left_max = max(left_max, heights[left_index]);
        right_max = max(right_max, heights[right_index]);
        if left_max <= right_max {
            total = total + left_max - heights[left_index];
            left_index = left_index + 1;
        } else {
            total = total + right_max - heights[right_index];
            right_index = right_index - 1;
        }
    }
    total
}

fn main() {
    let heights:[u32;12] = [0,1,0,2,1,0,1,3,2,1,2,1];
    println!("{:?} max total rain {}", &heights, trap(&heights));
    println!("{:?} max total rain {}", &heights, trap_memo(&heights));
    println!("{:?} max total rain {}", &heights, trap_best(&heights));
}
