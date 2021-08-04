use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;

fn zero_one_bag(weights: &[usize], values: &[usize], n: usize, w: usize) -> usize {
    let N = n + 1;
    let W = w + 1;
    let mut dp:Vec<Vec<usize>> = vec![vec![0;W];N];
    for i in 1..N {
        for j in 1..W {
            if j < weights[i-1] {
                dp[i][j] = dp[i-1][j];
            } else {
                dp[i][j] = cmp::max(dp[i-1][j], dp[i-1][j-weights[i-1]] + values[i-1]);
            }
        }
    }
    dp[n][w]
}

fn test_zero_one_bag(weights: &[usize], values: &[usize], n: usize, w: usize){
    println!("=====start zero one bag {:?} {:?} {} {}", weights, values, n, w);
    println!("{}", zero_one_bag(weights, values, n, w));
    println!("=====end array ");
}

fn main() {
    test_zero_one_bag(&[2, 1, 3], &[4, 2, 3], 3, 4);
}
