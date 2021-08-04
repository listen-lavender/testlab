use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

struct UFG {
    connected_component_total: u32,
    forest_node_parent: Vec<usize>,
    forest_node_size: Vec<u32>,
}

impl UFG {
    fn new(n: u32) -> UFG {
        let mut forest_node_parent = vec![0; n as usize];
        let mut forest_node_size = vec![1; n as usize];
        for k in 0..n as usize {
            forest_node_parent[k] = k;
        }
        UFG{
            connected_component_total: n,
            forest_node_parent:forest_node_parent,
            forest_node_size:forest_node_size,
        }
    }

    fn connect(&mut self, p: usize, q: usize) {
        let pa = self.find_ancestor(p);
        let qa = self.find_ancestor(q);
        if pa == qa {
            return
        }
        self.forest_node_parent[p] = q;
        // self.forest_node_size[p] = self.forest_node_size[p] - 1;
        // self.forest_node_size[q] = self.forest_node_size[q] + 1;
        self.forest_node_size[q] = self.forest_node_size[q] + self.forest_node_size[p];
        self.connected_component_total = self.connected_component_total - 1;
    }

    fn balance_connect(&mut self, p: usize, q: usize) {
        let pa = self.find_ancestor(p);
        let qa = self.find_ancestor(q);
        if pa == qa {
            return
        }
        if self.forest_node_size[pa] > self.forest_node_size[qa] {
            self.forest_node_parent[q] = p;
            // self.forest_node_size[q] = self.forest_node_size[q] - 1;
            // self.forest_node_size[p] = self.forest_node_size[p] + 1;
            self.forest_node_size[p] = self.forest_node_size[p] + self.forest_node_size[q];
        } else {
            self.forest_node_parent[p] = q;
            // self.forest_node_size[p] = self.forest_node_size[p] - 1;
            // self.forest_node_size[q] = self.forest_node_size[q] + 1;
            self.forest_node_size[q] = self.forest_node_size[q] + self.forest_node_size[p];
        }
        self.connected_component_total = self.connected_component_total - 1;
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        let pa = self.find_ancestor(p);
        let qa = self.find_ancestor(q);
        pa == qa
    }

    fn find_ancestor(&self, p: usize) -> usize {
        let mut q = p;
        while self.forest_node_parent[q] != q {
            q = self.forest_node_parent[q];
        }
        q
    }

    fn compact_find_ancestor(&mut self, p: usize) -> usize {
        let mut q = p;
        while self.forest_node_parent[q] != q {
            self.forest_node_parent[q] = self.forest_node_parent[self.forest_node_parent[q]];
            q = self.forest_node_parent[q];
        }
        q
    }

    fn count(&self) -> u32 {
        self.connected_component_total
    }
}

fn equations(express: &[[char;4]]) -> bool {
    let c:char = 'a';
    let start = c as usize;
    let c:char = 'z';
    let end = c as usize;
    let n = end - start + 1;

    let mut ufg = UFG::new(n as u32);
    for cs in express {
        if cs[1] == '=' {
            // println!("{} {}", cs[0], cs[3]);
            ufg.balance_connect(to_lower_case(cs[0]) as usize - start, to_lower_case(cs[3]) as usize - start);
        }
    }

    let mut equal = true;
    for cs in express {
        if cs[1] == '!' && ufg.connected(to_lower_case(cs[0]) as usize - start, to_lower_case(cs[3]) as usize - start) {
            // println!("{} {}", cs[0], cs[3]);
            equal = false;
            break
        }
    }

    equal
}

fn to_lower_case(c: char) -> char {
    c.to_lowercase().collect::<Vec<char>>()[0]
}

fn to_string(cs:&[char;4]) -> String {
    let mut s = String::with_capacity(cs.len());
    for c in cs {
        s.push(*c);
    }
    s
}

fn test_equations(express: &[[char;4]]){
    println!("=====start equations");
    for cs in express {
        print!("{} ", to_string(cs));
    }
    println!("");
    println!("====={}", equations(express));
    println!("=====end");
}

fn main() {
    // let c:char = 'C';
    // println!("{} {}", c, to_lower_case(c));
    // let c:char = 'a';
    // println!("{} {}", c, c as u32);
    // let c:char = 'z';
    // println!("{} {}", c, c as u32);
    test_equations(&[['a', '=', '=', 'b'], ['b', '!', '=', 'c'], ['c', '=', '=', 'a']]);
    test_equations(&[['a', '=', '=', 'b'], ['b', '=', '=', 'c'], ['c', '=', '=', 'a']]);
    test_equations(&[['c', '=', '=', 'c'], ['b', '=', '=', 'd'], ['x', '!', '=', 'z']]);
}