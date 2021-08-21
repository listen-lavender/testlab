use std::mem::swap;
use std::cmp::max;
use std::thread;
use std::str::Chars;
use std::collections::HashMap;
use std::collections::HashSet;

struct KMP {
    pattern_dp:Vec<Vec<usize>>,
    cb:usize,
    ct:usize,
    terminal_state:usize,
    pattern_length:usize,
}

impl KMP {
    fn new(pattern:&str) -> KMP {
        let terminal_state = pattern.len();
        let cb = to_upper_case('a') as usize;
        let ct = to_upper_case('z') as usize;

        let dp = vec![0;(to_upper_case('z') as usize - to_upper_case('a') as usize) + 1];
        let mut pattern_dp = vec![dp;pattern.len()+1];
        let cs = pattern.chars().collect::<Vec<char>>();

        let init_state:usize = 0;
        let init_action = (to_upper_case(cs[0]) as usize) - cb;
        let mut same_action_last_state:usize = 0;
        for maybe_action in cb .. ct + 1 {
            let maybe_action = maybe_action - cb;
            pattern_dp[init_state][maybe_action] = 0;
        }
        pattern_dp[init_state][init_action] = 1;

        for (current_state, current_action) in cs.iter().enumerate() {
            if current_state == init_state {
                continue;
            }
            let current_action = to_upper_case(*current_action) as usize - cb;
            let next_state = current_state + 1;
            for maybe_action in cb .. ct + 1 {
                let maybe_action = maybe_action - cb;
                if current_action == maybe_action {
                    pattern_dp[current_state][maybe_action] = next_state;
                } else {
                    pattern_dp[current_state][maybe_action] = 0;
                }
                if pattern_dp[current_state][maybe_action] == 0 {
                    pattern_dp[current_state][maybe_action] = pattern_dp[same_action_last_state][maybe_action]
                }
            }
            same_action_last_state = pattern_dp[same_action_last_state][current_action]
        }
        KMP{
            pattern_dp:pattern_dp,
            cb:cb,
            ct:ct,
            terminal_state:terminal_state,
            pattern_length:terminal_state,
        }
    }
    fn search(&self, txt: &str) -> i32 {
        if txt.len() < self.pattern_length {
            return -1
        }
        let cs = txt.chars().collect::<Vec<char>>();
        let mut current_state:usize = 0;
        let mut start_index:i32 = -1;
        for (index, current_action) in cs.iter().enumerate() {
            let current_action = to_upper_case(*current_action) as usize - self.cb;
            let next_state = self.pattern_dp[current_state][current_action];
            if next_state == self.terminal_state {
                start_index = (index + 1 - self.pattern_length) as i32;
                break
            }
            current_state = next_state;
        }
        start_index
    }
}

fn to_upper_case(c: char) -> char {
    c.to_uppercase().collect::<Vec<char>>()[0]
}

fn main() {
    println!("{} {} {}", to_upper_case('a') as u32, to_upper_case('b') as u32, to_upper_case('z') as u32);

    let s = "kkabdabce";
    let t = "abdabc";
    let kmp = KMP::new(t);
    let start_index = kmp.search(s);
    println!("in {} find {}: {}", s, t, start_index);

    let s = "kkfffeegee";
    let t = "abdabc";
    let kmp = KMP::new(t);
    let start_index = kmp.search(s);
    println!("in {} find {}: {}", s, t, start_index);

    let s = "abdabc";
    let t = "abdabc";
    let kmp = KMP::new(t);
    let start_index = kmp.search(s);
    println!("in {} find {}: {}", s, t, start_index);

    let s = "abdab";
    let t = "abdabc";
    let kmp = KMP::new(t);
    let start_index = kmp.search(s);
    println!("in {} find {}: {}", s, t, start_index);

    let s = "sabaabact";
    let t = "abaabac";
    let kmp = KMP::new(t);
    let start_index = kmp.search(s);
    println!("in {} find {}: {}", s, t, start_index);

    let s = "abaabac";
    let t = "abaabac";
    let kmp = KMP::new(t);
    let start_index = kmp.search(s);
    println!("in {} find {}: {}", s, t, start_index);

    let s = "kkkkkkkk";
    let t = "abaabac";
    let kmp = KMP::new(t);
    let start_index = kmp.search(s);
    println!("in {} find {}: {}", s, t, start_index);
}
