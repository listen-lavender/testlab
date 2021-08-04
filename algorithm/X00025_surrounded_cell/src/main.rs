use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

static DIRECTIONS: &[[i32;2]] = &[[0, 1], [0, -1], [1, 0], [-1, 0]];

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

fn replace_surround_cell_by_dfs(cells:&mut[&mut[char]]){
    let n = cells.len();
    if n < 3 {
        return
    }
    for i in 0..n {
        dfs_surround_cell(Rc::new(RefCell::new(cells)), i, 0);
        dfs_surround_cell(Rc::new(RefCell::new(cells)), i, n-1);
    }
    for j in 1..n-1 {
        dfs_surround_cell(Rc::new(RefCell::new(cells)), 0, j);
        dfs_surround_cell(Rc::new(RefCell::new(cells)), n-1, j);
    }
    for k in 0..n*n - 1 {
        let i = k/n;
        let j = k%n;
        if cells[i][j] == 'O' {
            cells[i][j] = 'X';
        } else if cells[i][j] == '*' {
            cells[i][j] = 'O';
        }
    }

}

fn dfs_surround_cell(cells:Rc<RefCell<&mut [&mut [char]]>>, i:usize, j:usize){
    let n = cells.borrow().len() as i32;
    if cells.borrow()[i][j] == '*' || cells.borrow()[i][j] == 'X' {
        return
    }
    if cells.borrow()[i][j] == 'O' {
        cells.borrow_mut()[i][j] = '*';
    }
    for d in DIRECTIONS {
        let next_i = i as i32 + d[0];
        let next_j = j as i32 + d[1];
        if next_i >= 0 && next_j >= 0 && next_i < n && next_j < n{
            dfs_surround_cell(cells.clone(), next_i as usize, next_j as usize);
        }
    }
}

fn union_find_surround_cell(cells:&[&[char]]){

}

fn test_replace_surround_cell_by_dfs(cells:&mut[&mut[char]]){
    println!("=====start replace surround cell by dfs");
    replace_surround_cell_by_dfs(cells);
    for c in cells {
        println!("{:?}", c);
    }
    println!("=====end");
}

fn main() {
    test_replace_surround_cell_by_dfs(&mut[&mut['X', 'X', 'X', 'X'], &mut['X', 'O', 'O', 'X'], &mut['X', 'X', 'O', 'X'], &mut['X', 'O', 'X', 'X']]);
}