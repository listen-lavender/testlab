use std::mem::swap;
use std::cmp::max;
use std::thread;
use std::str::Chars;
use std::collections::HashMap;
use std::collections::HashSet;

fn min_cover_substr(s: &str, t: &str) -> String {
    let s:Vec<char> = s.to_string().chars().collect();
    let t:Vec<char> = t.to_string().chars().collect();

    let mut need:HashMap<char, u32> = HashMap::with_capacity(t.len());
    for c in &t {
        if need.contains_key(c) {
            need.insert(*c, need[c]+1);
        } else {
            need.insert(*c, 1);
        }
    }

    let mut left:usize = 0;
    let mut right:usize = 0;
    let mut valid_count:usize = 0;
    let mut window:HashMap<char, u32> = HashMap::new();
    let mut index:usize = 0;
    let mut length:usize = s.len() + 1;
    while right < s.len() {
        let c = s[right];
        if need.contains_key(&c) {
            if window.contains_key(&c) {
                window.insert(c, window[&c] + 1);
            } else {
                window.insert(c, 1);
            }
            if window[&c] == need[&c] {
                valid_count = valid_count + 1;
            }
        }
        right = right + 1;

        while valid_count == need.len() {
            if right - left < length {
                length = right - left;
                index = left;
            }
            let c = s[left];
            if window.contains_key(&c) {
                if window[&c] == need[&c] {
                    valid_count = valid_count - 1;
                }
                window.insert(c, window[&c] - 1);
            }
            left = left + 1;
        }
    }

    if length > s.len() || length < t.len() {
        "".to_string()
    } else {
        s[index..index+length].iter().cloned().collect::<String>()
    }
}

fn cover_substr(s: &str, t: &str) -> Vec<String> {
    let s:Vec<char> = s.to_string().chars().collect();
    let t:Vec<char> = t.to_string().chars().collect();

    let mut need:HashMap<char, u32> = HashMap::with_capacity(t.len());
    for c in &t {
        if need.contains_key(c) {
            need.insert(*c, need[c]+1);
        } else {
            need.insert(*c, 1);
        }
    }

    let mut left:usize = 0;
    let mut right:usize = 0;
    let mut valid_count:usize = 0;
    let mut window:HashMap<char, u32> = HashMap::new();
    let mut index:usize = 0;
    let mut length:usize = s.len() + 1;
    let mut existed:HashSet<String> = HashSet::new();
    let mut permutation:Vec<String> = Vec::new();
    while right < s.len() {
        let c = s[right];
        if need.contains_key(&c) {
            if window.contains_key(&c) {
                window.insert(c, window[&c] + 1);
            } else {
                window.insert(c, 1);
            }
            if window[&c] == need[&c] {
                valid_count = valid_count + 1;
            }
        }
        right = right + 1;

        while right - left >= need.len() {
            if valid_count == need.len() {
                length = right - left;
                index = left;
                if length == need.len() {
                    let p = s[index..index+length].iter().cloned().collect::<String>();
                    if !existed.contains(&p) {
                        permutation.push(p.clone());
                        existed.insert(p);
                    }
                }
            }
            let c = s[left];
            if window.contains_key(&c) {
                if window[&c] == need[&c] {
                    valid_count = valid_count - 1;
                }
                window.insert(c, window[&c] - 1);
            }
            left = left + 1;
        }
    }

    permutation
}

fn return_str(s: &mut String) -> &str {

    for _ in 0..10 {
        s.push_str("ACTG");
    }

    &s[..]
}

fn max_diffstr(s: &str) -> String {
    let s:Vec<char> = s.to_string().chars().collect();

    let mut left:usize = 0;
    let mut right:usize = 0;
    let mut window:HashMap<char, usize> = HashMap::new();
    let mut index:usize = 0;
    let mut length:usize = 0;
    while right < s.len() {
        let c = s[right];

        if !window.contains_key(&c) {
            window.insert(c, right);
        } else if window[&c] == s.len() {
            window.insert(c, right);
        } else {
            if right - left > length {
                index = left;
                length = right - left;
            }
            while left < right {
                println!("{} {}", left, window[&c]);
                let cc = s[left];
                window.insert(cc, s.len());
                left = left + 1;
            }
            window.insert(c, right);
        }
        right = right + 1;
    }

    if length == 0 || index + length > s.len(){
        "".to_string()
    } else {
        s[index..index+length].iter().cloned().collect::<String>()
    }
}

fn example(s: String, t: &str) -> &str {
    ""
}

fn dangle<'a>() -> &'a str {
    "hello world"
}

fn main() {
    // let mut s = "ADOBECODEBANC".to_string();
    // let mut t = "ABC".to_string();
    // let a:Chars = s.chars();
    // let b:Vec<char> = a.clone().collect();
    // println!("{}", a.count() == b.len());
    // for k in t.chars() {
    //     println!("{}", k);
    // }
    // let mut s = String::new();
    // let s = return_str(&mut s);
    // assert_eq!("ACTGACTGACTGACTGACTGACTGACTGACTGACTGACTG", s);
    let s = "ADOBECODEBANC";
    let t = "ABC";
    let r = min_cover_substr(s, t);
    println!("{} {} cover substr {}", s, t, &r[..]);
    let s = "a";
    let t = "a";
    let r = min_cover_substr(s, t);
    println!("{} {} cover substr {}", s, t, &r[..]);
    let s = "a";
    let t = "aa";
    let r = min_cover_substr(s, t);
    println!("{} {} cover substr {}", s, t, &r[..]);
    let s = "eidbababooo";
    let t = "ab";
    let r = cover_substr(s, t);
    for c in r {
        println!("{} {} permutation string {}", s, t, c);
    }
    let s = "eidbacbacabacabcaooo";
    let t = "abc";
    let r = cover_substr(s, t);
    for c in r {
        println!("{} {} permutation string {}", s, t, c);
    }

    let s = "abcabcbb";
    let r =  max_diffstr(s);
    println!("{} max diffstr {}", s, &r[..]);
    let s = "bbbbb";
    let r =  max_diffstr(s);
    println!("{} max diffstr {}", s, &r[..]);
    let s = "pwwwkew";
    let r =  max_diffstr(s);
    println!("{} max diffstr {}", s, &r[..]);
}
