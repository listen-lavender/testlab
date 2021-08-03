use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;

fn remove_cover_region(regions: &mut [[u32;2]]) -> usize {
    if regions.len() < 2 {
        return regions.len()
    }
    regions.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1]).reverse()));
    let mut left = regions[0][0];
    let mut right = regions[0][1];
    let mut total: usize = regions.len();
    for k in 1..regions.len() {
        if regions[k][0] <= right {
            if regions[k][1] <= right {
                total = total - 1;
            } else {
                right = regions[k][1];
            }
        } else {
            left = regions[k][0];
            right = regions[k][1];
        }
    }
    total
}

fn union_region(regions: &mut [[u32;2]]) -> &[[u32;2]] {
    if regions.len() < 2 {
        return &regions[0..0]
    }
    regions.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1]).reverse()));
    let mut left = regions[0][0];
    let mut right = regions[0][1];
    let mut ind:usize = 0;
    for k in 1..regions.len() {
        // println!("{} {}; {}, {}", regions[k][0], regions[k][1], left, right);
        if regions[k][0] <= right {
            if regions[k][1] <= right {
                continue
            } else {
                right = regions[k][1];
            }
        } else {
            regions[ind] = [left, right];
            ind = ind + 1;
            left = regions[k][0];
            right = regions[k][1];
        }
    }
    regions[ind] = [left, right];
    &regions[0..ind+1]
}

fn intersect_region(up_line: &mut [[u32;2]], down_line: &mut [[u32;2]]) -> Vec<[u32;2]> {
    let mut r:Vec<[u32;2]> = Vec::new();
    if up_line.len() == 0 || down_line.len() == 0 {
        return r
    }
    up_line.sort_by(|a, b| a[0].cmp(&b[0]));
    down_line.sort_by(|a, b| a[0].cmp(&b[0]));

    // let mut left = up_line[0][0];
    // let mut right = up_line[0][1];
    let mut i:usize = 0;
    let mut j:usize = 0;
    loop {
        if i >= up_line.len() || j >= down_line.len() {
            break;
        }
        let (up_left, up_right) = (up_line[i][0], up_line[i][1]);
        let (down_left, down_right) = (down_line[j][0], down_line[j][1]);
        if up_right >= down_left && down_right >= up_left {
            r.push([cmp::max(up_left, down_left), cmp::min(up_right, down_right)]);
        }
        if up_right > down_right {
            j = j + 1;
        } else {
            i = i + 1;
        }
    }
    r
}

fn test_remove_cover_region(regions: &mut [[u32;2]]){
    println!("=====start remove cover region {:?}", regions);
    println!("=====left total {}", remove_cover_region(regions));
    println!("=====end region {:?}", regions);
}

fn test_union_region(regions: &mut [[u32;2]]){
    println!("=====start union region {:?}", regions);
    println!("===== {:?}", union_region(regions));
    println!("=====end region");
}

fn test_intersect_region(up_line: &mut [[u32;2]], down_line: &mut [[u32;2]]){
    println!("=====start intersect region up {:?} down {:?}", up_line, down_line);
    println!("===== {:?}", intersect_region(up_line, down_line));
    println!("=====end region");
}

fn new_int<'a>() -> &'a isize {
    let a: &'a isize = &5; // write 5 on the function's local stack
    a // return a pointer to that area of memory
}

fn main() {
    test_remove_cover_region(&mut [[1, 4], [1, 5], [3, 6], [2, 8]]);
    test_union_region(&mut [[1, 3], [2, 6], [8, 10], [15, 18]]);
    test_union_region(&mut [[1, 4], [4, 5], [8, 10], [15, 18]]);
    test_intersect_region(&mut [[0, 2], [5, 10], [13, 23], [24, 25]], &mut [[1, 5], [8, 12], [15, 24], [25, 26]])
}