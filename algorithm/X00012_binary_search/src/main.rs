use std::mem::swap;
use std::cmp::max;
use std::thread;

fn binary_search(nums:&[u32], target: u32) -> i32 {
    let mut left:usize = 0;
    let mut right = nums.len() - 1;
    let mut ind:i32 = -1;
    loop {
        if left > right {
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

fn left_bound(nums:&[u32], target:u32) -> i32 {
    if nums.len() == 0 {
        -1
    } else {
        // let mut left:usize = 0;
        // let mut right = nums.len();
        // let mut ind:i32 = -1;
        // loop {
        //     if left >= right {
        //         ind = left as i32;
        //         break;
        //     }
        //     // thread::sleep_ms(1000);
        //     let mid = left + (right - left)/2;
        //     if nums[mid] < target {
        //         left = mid + 1;
        //     } else if nums[mid] > target {
        //         right = mid;
        //     } else {
        //         right = mid;
        //     }
        // }
        // if left >= nums.len() || nums[left] != target {
        //     ind = -1;
        // }
        // ind
        let mut left:i32 = 0;
        let mut right = (nums.len() - 1) as i32;
        let mut ind:i32 = -1;
        loop {
            if left > right {
                ind = left;
                break;
            }
            // thread::sleep_ms(1000);
            let mid = (left + (right - left)/2) as usize;
            if nums[mid] < target {
                left = mid as i32 + 1;
            } else if nums[mid] > target {
                right = mid as i32 - 1;
            } else {
                right = mid as i32 - 1;
            }
        }
        if (left as usize) >= nums.len() || nums[left as usize] != target {
            ind = -1;
        }
        ind
    }
}

fn right_bound(nums:&[u32], target:u32) -> i32 {
    if nums.len() == 0 {
        -1
    } else {
        let mut left:i32 = 0;
        let mut right = (nums.len() - 1) as i32;
        let mut ind:i32 = -1;
        loop {
            if left > right {
                left = left - 1;
                ind = left as i32;
                break;
            }
            // thread::sleep_ms(1000);
            let mid = (left + (right - left)/2) as usize;
            if nums[mid] < target {
                left = mid as i32 + 1;
            } else if nums[mid] > target {
                right = mid as i32 - 1;
            } else {
                left = mid as i32 + 1;
            }
        }
        if left < 0 || nums[left as usize] != target {
            ind = -1;
        }
        ind
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
    let nums = &[1, 2, 3, 5, 5, 5, 5, 8, 9, 10];
    let target = 5;
    println!("bound {:?} {}: left {}, right {}", nums, target, left_bound(nums, target), right_bound(nums, target));
    let nums = &[1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
    let target = 6;
    println!("bound {:?} {}: left {} right {}", nums, target, left_bound(nums, target), right_bound(nums, target));
    let target = 5;
    println!("bound {:?} {}: left {} right {}", nums, target, left_bound(nums, target), right_bound(nums, target));
    let target = 7;
    println!("bound {:?} {}: left {} right {}", nums, target, left_bound(nums, target), right_bound(nums, target));
    let target = 11;
    println!("bound {:?} {}: left {} right {}", nums, target, left_bound(nums, target), right_bound(nums, target));
    let target = 1;
    println!("bound {:?} {}: left {} right {}", nums, target, left_bound(nums, target), right_bound(nums, target));
    let target = 12;
    println!("bound {:?} {}: left {} right {}", nums, target, left_bound(nums, target), right_bound(nums, target));
    let target = 0;
    println!("bound {:?} {}: left {} right {}", nums, target, left_bound(nums, target), right_bound(nums, target));
}
