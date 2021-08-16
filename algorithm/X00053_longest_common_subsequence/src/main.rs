use std::mem::swap;
use std::cmp::max;
use std::thread;
use std::str::Chars;
use std::collections::HashMap;
use std::collections::HashSet;

fn lcs(s: &str, i:usize, t: &str, j:usize) -> String {
    if i >= s.len() {
        return "".to_string()
    }
    if j >= t.len() {
        return "".to_string()
    }
    let ss:Vec<char> = s.to_string().chars().collect();
    let tt:Vec<char> = t.to_string().chars().collect();
    let a:char = ss[i];
    let b:char = tt[j];
    if a == b {
        let mut cs = lcs(s, i+1, t, j+1);
        cs.push(a);
        cs
    } else {
        let ics = lcs(s, i, t, j+1);
        let jcs = lcs(s, i+1, t, j);
        // ignore: let ijcs = lcs(s, i+1, t, j+1);
        if ics.len() > jcs.len() {
            ics
        } else {
            jcs
        }
    }
}

fn memory_lcs(s: &str, i:usize, t: &str, j:usize) -> String {
    "".to_string()
}

fn dpt_lcs(s: &str, i:usize, t: &str, j:usize) -> String {
    "".to_string()
}

fn main() {
    let s = "abcde";
    let t = "acfe";
    let r = lcs(s, 0, t, 0);
    println!("{} {} common substr {}", s, t, r.chars().rev().collect::<String>());

    let s = "abcde";
    let t = "acfee";
    let r = lcs(s, 0, t, 0);
    println!("{} {} common substr {}", s, t, r.chars().rev().collect::<String>());
}
