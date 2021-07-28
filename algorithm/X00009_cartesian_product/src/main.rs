use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;


struct Cartesian<'a> {
    solutions: Vec<&'a str>,
    total: u32,
}

fn sku_by_attr<'a>(matrix: &[&[&'a str]], row: usize, cartesian:&mut Vec<&'a str>, acc:&mut Vec<String>){
    if row >= matrix.len() {
        acc.push(cartesian.concat());
    } else {
        for col in matrix[row] {
            cartesian.push(col);
            sku_by_attr(matrix, row+1, cartesian, acc);
            cartesian.pop();
        }
    }
}

fn all_permutation(arr: &[u32], row: usize, cartesian:&mut Vec<u32>, acc:&mut Vec<String>) {
    if row >= arr.len() {
        let mut c: Vec<String> = Vec::new();
        for n in cartesian {
            c.push(n.to_string());
        }
        acc.push(c.concat());
    } else {
        for col in arr {
            if cartesian.contains(&col) {
                continue
            }
            cartesian.push(*col);
            all_permutation(arr, row+1, cartesian, acc);
            cartesian.pop();
        }
    }
}

fn test_sku_by_attr(matrix: &[&[&str]]){
    println!("=====start n dimension {} skus", matrix.len());
    let mut cartesian = Vec::new();
    let mut acc = Vec::new();
    sku_by_attr(matrix, 0, &mut cartesian, &mut acc);
    println!("=====");
    for cartesian in acc {
        println!("{}", cartesian);
    }
    println!("=====end ");
}

fn test_all_permutation(arr: &[u32]){
    println!("=====start n dimension {} permutation", arr.len());
    let mut cartesian = Vec::new();
    let mut acc = Vec::new();
    all_permutation(arr, 0, &mut cartesian, &mut acc);
    println!("=====");
    for cartesian in acc {
        println!("{}", cartesian);
    }
    println!("=====end ");
}

fn main() {
    test_sku_by_attr(&[&["红色", "黄色", "蓝色"], &["35码", "40码"], &["复古", "前卫", "经典", "西式"]]);
    test_all_permutation(&[1, 2, 3]);
}
