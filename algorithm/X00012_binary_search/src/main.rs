use std::mem::swap;
use std::cmp::max;
use std::thread;

fn binary_search(nums:&[u32], target: u32) -> i32 {
    let mut left:usize = 0;
    let mut right = nums.len() - 1;
    let mut ind:i32 = -1;
    loop {
        if left > right {
            ind = -1;
            break;
        }
        // thread::sleep_ms(1000);
        let mid = left + (right - left)/2;
        // println!("{} {} {}, {} {}", left, mid, right, nums[mid], target);
        if nums[mid] < target {
            left = mid + 1;
            // println!("right {} {} {}", left, mid, right);
        } else if nums[mid] > target {
            right = mid - 1;
            // println!("left {} {} {}", left, mid, right);
        } else {
            ind = mid as i32;
            break;
        }
    }
    ind
}

fn recursive_binary_search(nums:&[u32], left:usize, right:usize, target: u32) -> i32 {
    if left > right {
        -1
    } else {
        let mid = left + (right - left)/2;
        // thread::sleep_ms(1000);
        // println!("{:?}, {} {} {}, {} {}", nums, left, mid, right, nums[mid], target);
        if nums[mid] < target {
            recursive_binary_search(nums, mid+1, right, target)
        } else if nums[mid] > target {
            recursive_binary_search(nums, left, mid-1, target)
        } else {
            mid as i32
        }
    }
}

fn main() {
    let nums = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 5;
    println!("binary_search {:?} {}: {}", nums, target, binary_search(nums, target));
    let target = 1;
    println!("binary_search {:?} {}: {}", nums, target, binary_search(nums, target));
    let target = 10;
    println!("binary_search {:?} {}: {}", nums, target, binary_search(nums, target));
    let target = 5;
    println!("recursive_binary_search {:?} {}: {}", nums, target, recursive_binary_search(nums, 0, nums.len(), target));
    let target = 1;
    println!("recursive_binary_search {:?} {}: {}", nums, target, recursive_binary_search(nums, 0, nums.len(), target));
    let target = 10;
    println!("recursive_binary_search {:?} {}: {}", nums, target, recursive_binary_search(nums, 0, nums.len(), target));
}
