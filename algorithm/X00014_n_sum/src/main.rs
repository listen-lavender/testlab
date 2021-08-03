use std::mem::swap;
use std::cmp::max;
use std::thread;
use std::collections::HashMap;

struct SumTwoPair {
    x: u32,
    y: u32,
}

impl SumTwoPair {
    fn new(x: u32, y: u32) -> SumTwoPair {
        SumTwoPair{
            x:x,
            y:y,
        }
    }
}

struct SumThreePair {
    x: u32,
    y: u32,
    z: u32,
}

impl SumThreePair {
    fn new(x: u32, y: u32, z: u32) -> SumThreePair {
        SumThreePair{
            x:x,
            y:y,
            z:z,
        }
    }
}

fn fast_two_sum(nums:&[u32], target: u32) -> Vec<SumTwoPair> {
    let mut halfs:HashMap<u32, u32> = HashMap::with_capacity(nums.len());
    for n in nums {
        if halfs.contains_key(n) {
            halfs.insert(*n, halfs[n] + 1);
        } else {
            halfs.insert(*n, 1);
        }
    }
    let mut pairs:Vec<SumTwoPair> = Vec::new();
    for n in nums {
        if target < *n {
            continue
        }
        if halfs[n] == 0 {
            continue
        }
        let m = target - *n;
        if halfs.contains_key(&m) {
            if (*n == m && halfs[&m] > 1) || halfs[&m] > 0 {
                pairs.push(SumTwoPair{
                    x:*n,
                    y:m,
                });
                halfs.insert(m, halfs[&m] - 1);
            }
        }
        halfs.insert(*n, halfs[n] - 1);
    }
    pairs
}

fn two_sum(nums:&mut [u32], start:usize, target: u32) -> Vec<SumTwoPair> {
    let mut pairs:Vec<SumTwoPair> = Vec::new();
    let mut left:i32 = start as i32;
    let mut right:i32 = nums.len() as i32 - 1;

    if left >= right {
        return pairs;
    }

    nums.sort();
    
    while left < right {
        let n = nums[left as usize];
        let m = nums[right as usize];
        if n + m < target {
            left = left + 1;
        } else if n + m > target {
            right = right - 1;
        } else {
            pairs.push(SumTwoPair{
                x:n,
                y:m,
            });
            left = left + 1;
            right = right - 1;
        }
    }
    pairs
}

fn fast_three_sum(nums:&[u32], target: u32) -> Vec<SumThreePair> {
    let mut pairs:Vec<SumThreePair> = Vec::new();
    // let mut ghalfs:HashMap<u32, u32> = HashMap::with_capacity(nums.len());
    // for n in nums {
    //     if ghalfs.contains_key(n) {
    //         ghalfs.insert(*n, ghalfs[n] + 1);
    //     } else {
    //         ghalfs.insert(*n, 1);
    //     }
    // }

    // for l in nums {
    //     let mut halfs:HashMap<u32, u32> = HashMap::with_capacity(nums.len());
    //     for n in nums {
    //         if halfs.contains_key(n) {
    //             halfs.insert(*n, halfs[n] + 1);
    //         } else {
    //             halfs.insert(*n, 1);
    //         }
    //     }
    //     let mut pairs:Vec<SumTwoPair> = Vec::new();
    //     for n in nums {
    //         if target < *n {
    //             continue
    //         }
    //         let m = target - *n;
    //         if halfs.contains_key(&m) {
    //             if (*n == m && halfs[&m] > 1) || halfs[&m] > 0 {
    //                 pairs.push(SumTwoPair{
    //                     x:*n,
    //                     y:m,
    //                 });
    //                 halfs.insert(m, halfs[&m] - 1);
    //             }
    //         }
    //         halfs.insert(*n, halfs[n] - 1);
    //     }
    // }
    pairs
}

fn three_sum(nums:&mut [u32], start:usize, target: u32) -> Vec<SumThreePair> {
    let mut pairs:Vec<SumThreePair> = Vec::new();

    if start >= nums.len() - 2 {
        return pairs;
    }

    nums.sort();

    for (ind, l) in nums.iter().enumerate() {
        if ind < start {
            continue
        }

        if *l >= target {
            continue
        }

        let mut left:i32 = ind as i32 + 1;
        let mut right:i32 = nums.len() as i32 - 1;

        if left >= right {
            break;
        }
        
        while left < right {
            let n = nums[left as usize];
            let m = nums[right as usize];
            if n + m < (target - l) {
                left = left + 1;
            } else if n + m > (target - l) {
                right = right - 1;
            } else {
                pairs.push(SumThreePair{
                    x:*l,
                    y:m,
                    z:n,
                });
                left = left + 1;
                right = right - 1;
            }
        }
    }

    pairs
}

fn main() {
    let nums = &[12, 6, 5, 3, 1, 6, 3, 7, 10];
    let target = 9;
    let pairs = fast_two_sum(nums, target);
    for p in pairs {
        println!("fast_two_sum {:?} {}: {} {}", nums, target, p.x, p.y);
    }

    let mut nums = [12, 6, 5, 3, 1, 6, 3, 7, 10];
    let target = 9;
    let pairs = two_sum(&mut nums, 0, target);
    for p in pairs {
        println!("two_sum {:?} {}: {} {}", nums, target, p.x, p.y);
    }

    let nums = &[12, 5, 3, 1, 4, 4, 6, 7, 10];
    let target = 8;
    let pairs = fast_two_sum(nums, target);
    for p in pairs {
        println!("fast_two_sum {:?} {}: {} {}", nums, target, p.x, p.y);
    }

    let mut nums = [12, 5, 3, 1, 4, 4, 6, 7, 10];
    let target = 8;
    let pairs = two_sum(&mut nums, 0, target);
    for p in pairs {
        println!("two_sum {:?} {}: {} {}", nums, target, p.x, p.y);
    }

    let mut nums = [12, 5, 3, 1, 4, 4, 6, 7, 10];
    let target = 9;
    let pairs = three_sum(&mut nums, 0, target);
    for p in pairs {
        println!("three_sum {:?} {}: {} {} {}", nums, target, p.x, p.y, p.z);
    }

    let mut nums = [12, 5, 3, 2, 1, 4, 4, 6, 3, 7, 4, 5, 5, 10];
    let target = 10;
    let pairs = three_sum(&mut nums, 0, target);
    for p in pairs {
        println!("three_sum {:?} {}: {} {} {}", nums, target, p.x, p.y, p.z);
    }
}
