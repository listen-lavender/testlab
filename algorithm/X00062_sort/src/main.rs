use std::mem::swap;
use std::cmp::max;
use std::thread;

fn partion(nums:&mut [u32], left: usize, right: usize) -> usize {
    let pivot = nums[left];
    let mut left = left;
    let mut right = right;
    
    loop {
        loop {
            if right <= left {
                break
            }
            if nums[right] <= pivot { // <=
                let temp = nums[right];
                nums[right] = nums[left];
                nums[left] = temp;
                left = left + 1;
                break
            }
            right = right - 1;
        }
        loop {
            if left >= right {
                break
            }
            if nums[left] > pivot {
                let temp = nums[left];
                nums[left] = nums[right];
                nums[right] = temp;
                right = right - 1;
                break
            }
            left = left + 1;
        }
        if left >= right {
            break
        }
    }
    println!("{} {} =====", pivot, left);
    left
}

fn quick_sort(nums:&mut [u32], left: usize, right: usize) {
    if left < right {
        let mid = partion(nums, left, right);
        // partion(nums, left, mid-1);
        // partion(nums, mid+1, right);
        if mid > 0 {
            quick_sort(nums, left, mid-1);
        }
        if mid + 1 < nums.len() {
            quick_sort(nums, mid+1, right);
        }
    }
}

fn main() {
    let mut nums = &mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut left = 0;
    let mut right = nums.len()-1;
    println!("before quick sort {:?}", nums);
    quick_sort(nums, left, right);
    println!("after quick sort {:?}", nums);

    let mut nums = &mut [2, 5, 4, 1, 9, 6, 3, 8, 7, 6, 10];
    let mut left = 0;
    let mut right = nums.len()-1;
    println!("before quick sort {:?}", nums);
    quick_sort(nums, left, right);
    println!("after quick sort {:?}", nums);


    let mut nums = &mut [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut left = 0;
    let mut right = nums.len()-1;
    println!("before quick sort {:?}", nums);
    quick_sort(nums, left, right);
    println!("after quick sort {:?}", nums);
}
