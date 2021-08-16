use std::mem::swap;
use std::cmp::min;
use std::thread;
use std::str::Chars;
use std::collections::HashMap;
use std::collections::HashSet;

fn min_three(a:i32, b:i32, c:i32) -> i32 {
    min(min(a, b), c)
}

fn min_distance(s: &str, i:i32, t: &str, j:i32) -> i32 {
    if i == -1 {
        return j + 1
    }
    if j == -1 {
        return i + 1
    }
    let ss:Vec<char> = s.to_string().chars().collect();
    let tt:Vec<char> = t.to_string().chars().collect();
    let a:char = ss[i as usize];
    let b:char = tt[j as usize];
    if a == b {
        min_distance(s, i-1, t, j-1)
    } else {
        let a = min_distance(s, i, t, j-1) + 1; // 插入
        let b = min_distance(s, i-1, t, j) + 1; // 删除
        let c = min_distance(s, i-1, t, j-1) + 1; // 替换
        min_three(a, b, c)
    }
}

fn memory_min_distance(s: &str, i:i32, t: &str, j:i32) -> i32 {
    0
}

fn dpt_min_distance(s: &str, i:i32, t: &str, j:i32) -> i32 {
    0
}

fn main() {
    let s = "horse";
    let t = "ros";
    let r = min_distance(s, s.len() as i32 - 1, t, t.len() as i32 - 1);
    println!("{} => {} min edit distance {}", s, t, r);

    let s = "ros";
    let t = "horse";
    let r = min_distance(s, s.len() as i32 - 1, t, t.len() as i32 - 1);
    println!("{} => {} min edit distance {}", s, t, r);
}
