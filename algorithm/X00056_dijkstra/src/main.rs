use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::min;

fn get_directdistance(flights:&[[i32;3]]) -> HashMap<[i32;2], i32> {
    let mut directdistance:HashMap<[i32;2], i32> = HashMap::new();
    for f in flights {
        let src_dst = [f[0], f[1]];
        directdistance.insert(src_dst, f[2]);
    }
    directdistance
}

fn dijkstra(n:usize, paths:&[[i32;3]], src:i32) -> Vec<i32> {
    let mut distance_from_src:Vec<i32> = vec![-1;n]; // Vec::with_capacity(n);
    let mut directdistance = get_directdistance(paths);

    let mut station_nodes:HashSet<i32> = HashSet::with_capacity(n);

    let mut min_station_distance:i32 = -1;
    let mut min_node:i32 = -1;
    for index in 0..n {
        let dst = index as i32;
        if dst == src {
            distance_from_src[dst as usize] = 0;
        } else {
            station_nodes.insert(dst);
            let src_dst = [src, dst];
            if directdistance.contains_key(&src_dst) {
                distance_from_src[dst as usize] = directdistance[&src_dst];
                if min_station_distance == -1 {
                    min_station_distance = distance_from_src[dst as usize];
                    min_node = dst;
                } else {
                    if distance_from_src[dst as usize] < min_station_distance {
                        min_station_distance = distance_from_src[dst as usize];
                        min_node = dst;
                    }
                }
            }
        }
    }

    loop {
        if station_nodes.len() == 0 || min_station_distance == -1{
            break
        }
        station_nodes.remove(&min_node);
        min_station_distance = -1;
        let mut next_min_node:i32 = -1;
        for dst in &station_nodes {
            let dst = *dst;
            let src_station = [src, min_node];
            let station_dst = [min_node, dst];
            // println!("==== {} {} {}", src, min_node, dst);
            let mut src_dst_path = distance_from_src[dst as usize];
            
            if directdistance.contains_key(&src_station) && directdistance.contains_key(&station_dst) {
                let src_station_path = directdistance[&src_station];
                let station_dst_path = directdistance[&station_dst];
                if src_dst_path == -1 || src_dst_path > src_station_path + station_dst_path {
                    src_dst_path = src_station_path + station_dst_path;
                    distance_from_src[dst as usize] = src_dst_path;
                    directdistance.insert([src, dst], src_dst_path);
                }
            }

            if src_dst_path != -1 {
                if min_station_distance == -1 {
                    min_station_distance = src_dst_path;
                    next_min_node = dst;
                } else {
                    if src_dst_path < min_station_distance {
                        min_station_distance = src_dst_path;
                        next_min_node = dst;
                    }
                }
            }
        }
        min_node = next_min_node;
    }

    distance_from_src
}

fn main() {
    let n:usize = 7;
    let paths:[[i32;3];24] = [
        [0,1,12],
        [1,0,12],
        [0,5,16],
        [5,0,16],
        [0,6,14],
        [6,0,14],
        [1,2,10],
        [2,1,10],
        [1,5,7],
        [5,1,7],
        [2,3,3],
        [3,2,3],
        [2,4,5],
        [4,2,5],
        [2,5,6],
        [5,2,6],
        [3,4,4],
        [4,3,4],
        [4,5,2],
        [5,4,2],
        [4,6,8],
        [6,4,8],
        [5,6,9],
        [6,5,9],
    ];
    
    let src:i32 = 3;
    let distances = dijkstra(n, &paths, src);
    for (dst, distance) in distances.iter().enumerate() {
        println!("{} to {} shortest {}", src, dst, distance);
    }
}

