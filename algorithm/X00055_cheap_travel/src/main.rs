use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::min;

fn get_directcost(flights:&[[usize;3]]) -> HashMap<usize, Vec<[usize;2]>> {
    let mut directcost:HashMap<usize, Vec<[usize;2]>> = HashMap::new();
    for f in flights {
        if !directcost.contains_key(&f[1]) {
            directcost.insert(f[1], Vec::new());
            // directcost[&f[1]] = Vec::new();
        }
        let path = directcost.get_mut(&f[1]);
        match path {
            Some(path) => {
                path.push([f[0], f[2]]);
            }
            None => {}
        }
    }
    directcost
}

fn find_cheapest_price(n:usize, flights:&[[usize;3]], src:usize, dst:usize, k:usize) -> i32 {
    let directcost = get_directcost(flights);
    // println!("{:?}", directcost);
    dp_path(&directcost, src, dst, k + 1)
}

fn dp_path(directcost:&HashMap<usize, Vec<[usize;2]>>, src:usize, dst:usize, k:usize) -> i32 {
    if src == dst {
        return 0;
    }
    if k == 0 {
        return -1;
    }
    let mut cost:i32 = -1;
    if directcost.contains_key(&dst) {
        let path = directcost.get(&dst);
        match path {
            Some(path) => {
                for dc in path {
                    let prev_cost = dp_path(directcost, src, dc[0], k - 1);
                    if prev_cost < 0 {
                        continue;
                    }
                    // println!("path {:?}", dc);
                    if cost == -1 {
                        cost = dc[1] as i32 + prev_cost;
                    } else {
                        cost = min(dc[1] as i32 + prev_cost, cost);
                    }
                }
            }
            None => {}
        }
    }
    cost
}

fn main() {
    let n:usize = 3;
    let flights:[[usize;3];3] = [[0,1,100],[1,2,100],[0,2,500]];
    let src:usize = 0;
    let dst:usize = 2;
    let k:usize = 0;
    println!("{} to {} in {} cost {}", src, dst, k, find_cheapest_price(n, &flights, src, dst, k));

    let n:usize = 3;
    let flights:[[usize;3];3] = [[0,1,100],[1,2,100],[0,2,500]];
    let src:usize = 0;
    let dst:usize = 2;
    let k:usize = 1;
    println!("{} to {} in {} cost {}", src, dst, k, find_cheapest_price(n, &flights, src, dst, k));

    let n:usize = 3;
    let flights:[[usize;3];3] = [[0,1,100],[1,2,100],[0,2,500]];
    let src:usize = 0;
    let dst:usize = 2;
    let k:usize = 2;
    println!("{} to {} in {} cost {}", src, dst, k, find_cheapest_price(n, &flights, src, dst, k));
}

