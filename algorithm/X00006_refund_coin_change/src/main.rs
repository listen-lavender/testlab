use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

fn min(a:u32, b:u32) -> u32 {
    if a < b { a } else { b }
}

fn minimum(arr: &Vec<u32>) -> u32 {
    if arr.len() == 0 {
        return 0
    }
    let mut m:u32 = arr[0];
    for (ind, n) in arr.iter().enumerate() {
        if ind == 0 {
            continue
        }
        m = min(m, *n);
    }
    m
}

fn floor(m:u32, n:u32) -> u32 {
    let mut m = m;
    let mut n = n;
    if n > m {
        let t = m;
        m = n;
        n = t;
    }
    let r = m % n;
    if r == 0 {
        return m/n;
    }
    m/n + 1
}

fn plain_coin_change(coins: &Vec<u32>, amount: u32) -> u32 {
    if amount == 0 {
        return 0
    }

    let mut total = floor(amount, minimum(coins));
    for c in coins.iter() {
        if amount < *c {
            continue
        }
        let back_total = plain_coin_change(coins, amount - c);
        total = min(total, back_total + 1);
    }
    return total
}

fn main() {
    let coins = vec![5, 2, 1];
    let amount = 10;
    println!("coin {}: {}", amount, plain_coin_change(&coins, amount));
    let amount = 9;
    println!("coin {}: {}", amount, plain_coin_change(&coins, amount));
    let amount = 7;
    println!("coin {}: {}", amount, plain_coin_change(&coins, amount));
    let amount = 3;
    println!("coin {}: {}", amount, plain_coin_change(&coins, amount));
    let amount = 2;
    println!("coin {}: {}", amount, plain_coin_change(&coins, amount));
    let amount = 1;
    println!("coin {}: {}", amount, plain_coin_change(&coins, amount));
    let amount = 0;
    println!("coin {}: {}", amount, plain_coin_change(&coins, amount));
}
