use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use rand::prelude::*;
use std::collections::HashSet;

// fn recurse<'a>(nums: &'a [i32], already_printed: &'a mut HashSet<&'a [i32]>) {
fn combine_set<'a, 'b>(nums: &'a [i32], already_printed: &'b mut HashSet<&'a [i32]>) {
    if !already_printed.contains(nums) {
        println!("{:#?} ;; {:#?}", already_printed, nums);
    }

    already_printed.insert(nums);

    if nums.len() >= 2 {
        combine_set(&nums[0..nums.len() - 1], already_printed);
        combine_set(&nums[1..nums.len()], already_printed);
    }
}

macro_rules! make_array {
    ($size: literal) => {{
        let mut rng = rand::thread_rng();
        let mut arr = [0_f64; $size];
        for item in arr.iter_mut() {
            *item = rng.gen::<f64>()*100.;
        }
        arr
    }}
}

struct Position {
    row: u32,
    col: u32,
}

struct Solution {
    // x: Position,
    // y: Position,
    pos: Vec<Position>,
}

struct SolutionSummary {
    solutions: Vec<Solution>,
    total: u32,
}

struct SolutionBrief {
    solutions: Vec<Vec<u32>>,
    total: u32,
}

fn abs_pair_equal(a: u32, b: u32, c: u32, d: u32) -> bool{
    if (a == c) || (b == d) || (a == b && c == d) {
        true
    } else {
        abs_sub(a, c) == abs_sub(b, d)
    }
}

fn abs_sub(a: u32, b: u32) -> u32{
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn matrix_2_queen(matrix: &[&[u32]]) -> SolutionSummary {
    let mut ss: SolutionSummary = SolutionSummary{
        solutions: Vec::new(),
        total: 0,
    };
    if matrix.len() == 0 {
        return ss
    }
    if matrix.len() != matrix[0].len() {
        return ss
    }
    let total_row = matrix.len();
    // let total_row = total_row as u32;
    let total_col = matrix[0].len();
    // let total_col = total_col as u32;
    let mut cell_row: usize = 1;
    let mut cell_col: usize = 1;
    // let mut s = Solution{
    //     x: Position{
    //         row: cell_row as u32,
    //         col: cell_col as u32,
    //     },
    //     y: Position{
    //         row: 0,
    //         col: 0,
    //     },
    // };
    let mut s = Solution{
        pos: Vec::new(),
    };
    s.pos.push(Position{
        row: cell_row as u32,
        col: cell_col as u32,
    });
    s.pos.push(Position{
        row: 0,
        col: 0,
    });
    loop {
        if cell_col > total_col {
            cell_row = cell_row + 1;
            cell_col = 1;
            s.pos[0].row = cell_row as u32;
            s.pos[0].col = cell_col as u32;
        }
        if cell_row > total_row {
            break
        }

        // for (i, row) in matrix[cell_row..total_row].iter().enumerate() {
        for i in cell_row..total_row {
            let i = i + 1;
            let i = i as u32;
            for j in 0..total_col {
                let j = j + 1;
                let j = j as u32;
                if i == s.pos[0].row || j == s.pos[0].col {
                    continue
                } else if abs_pair_equal(s.pos[0].row, s.pos[0].col, i, j) {
                    continue
                } else {
                    s.pos[1].row = i;
                    s.pos[1].col = j;
                    ss.solutions.push(s);
                    ss.total = ss.total + 1;
                    s = Solution{
                        pos: Vec::new(),
                    };
                    s.pos.push(Position{
                        row: cell_row as u32,
                        col: cell_col as u32,
                    });
                    s.pos.push(Position{
                        row: 0,
                        col: 0,
                    });
                }
            }
        }
        cell_col = cell_col + 1;
        s.pos[0].col = cell_col as u32;
    };
    ss
}

fn arr_2_queen(matrix: &[u32], dimension: usize) -> SolutionSummary {
    let mut ss: SolutionSummary = SolutionSummary{
        solutions: Vec::new(),
        total: 0,
    };
    if matrix.len() != dimension * dimension {
        return ss
    }
    let total_row = dimension;
    // let total_row = total_row as u32;
    let total_col = dimension;
    // let total_col = total_col as u32;
    let mut cell_row: usize = 1;
    let mut cell_col: usize = 1;
    // let mut s = Solution{
    //     x: Position{
    //         row: cell_row as u32,
    //         col: cell_col as u32,
    //     },
    //     y: Position{
    //         row: 0,
    //         col: 0,
    //     },
    // };
    let mut s = Solution{
        pos: Vec::new(),
    };
    s.pos.push(Position{
        row: cell_row as u32,
        col: cell_col as u32,
    });
    s.pos.push(Position{
        row: 0,
        col: 0,
    });
    loop {
        if cell_col > total_col {
            cell_row = cell_row + 1;
            cell_col = 1;
            s.pos[0].row = cell_row as u32;
            s.pos[0].col = cell_col as u32;
        }
        if cell_row > total_row {
            break
        }

        for i in cell_row..total_row {
            let i = i + 1;
            let i = i as u32;
            for j in 0..total_col {
                let j = j + 1;
                let j = j as u32;
                if i == s.pos[0].row || j == s.pos[0].col {
                    continue
                } else if abs_pair_equal(s.pos[0].row, s.pos[0].col, i, j) {
                    continue
                } else {
                    s.pos[1].row = i;
                    s.pos[1].col = j;
                    ss.solutions.push(s);
                    ss.total = ss.total + 1;
                    s = Solution{
                        pos: Vec::new(),
                    };
                    s.pos.push(Position{
                        row: cell_row as u32,
                        col: cell_col as u32,
                    });
                    s.pos.push(Position{
                        row: 0,
                        col: 0,
                    });
                }
            }
        }
        cell_col = cell_col + 1;
        s.pos[0].col = cell_col as u32;
    };
    ss
}

fn place_row<'a>(arr: &'a mut [u32], row: u32) {
    if row == (arr.len() as u32) {
        // for (i, j) in arr.iter().enumerate() {
        //     println!("solution: {:?} {:?}", i, j);
        // }
        println!("solution: {:?} {}", arr, test_n_queen(arr));
        // println!("========");
    } else {
        for j in 0..arr.len() {
            let mut can_place = true;
            for i in 0..row as usize {
                let prev_row = i as u32;
                let prev_col = arr[i];
                can_place = !abs_pair_equal(row, j as u32, prev_row, prev_col);
                // println!("curr {}, {} = prev {}, {} :: {}", row, j as u32, prev_row, prev_col, can_place);
                if !can_place {
                    // println!("false: {}", row);
                    break
                }
            }
            if can_place {
                // println!("true: {} {}", row, j);
                arr[row as usize] = j as u32;
                place_row(arr, row + 1);
            }
        }
    }
}

fn place_row_col<'a>(arr: &'a mut [u32], row: u32, col: u32) {
    // println!("{:#?}, {}, {}", arr, row, col);
    if col == (arr.len() as u32) {
    } else if row == (arr.len() as u32) {
        println!("solution: {:?}", arr);
        // for i in 0..arr.len() {
        //     arr[i] = 0;
        // }
    } else if row == 0 {
        arr[row as usize] = col;
        if col < arr.len() as u32 {
            place_row_col(arr, row+1, 0);
        }
        // for j in 0..arr.len() {
        //     place(arr, row+1, j as u32);
        // }
    } else {
        let mut can_place = true;
        for i in 0..row as usize {
            println!("{} <<<", i);
            let prev_row = i as u32;
            let prev_col = arr[i];
            can_place = !abs_pair_equal(row, col, prev_row, prev_col);
            println!("curr {}, {} = prev {}, {} :: {}", row, col, prev_row, prev_col, can_place);
            if !can_place {
                break
            }
        }
        if can_place {
            arr[row as usize] = col;
            println!("222 {} {}<<<", row+1, 0);
            place_row_col(arr, row+1, 0);
        } else {
            if col >= arr.len() as u32 {
                if row > 0 {
                    println!("000 {} {}<<<", row-1, arr[(row-1)as usize] + 1);
                    place_row_col(arr, row-1, arr[(row-1)as usize] + 1);
                } else {
                    println!("3333");
                }
            } else {
                println!("111 {} {}<<<", row, col+1);
                place_row_col(arr, row, col+1);
            }
        }
    }
}

// fn combine_set<'a, 'b>(nums: &'a [i32], already_printed: &'b mut HashSet<&'a [i32]>) {
fn n_queen(arr: &mut [u32]) {
    place_row(arr, 0);
    // place_row_col(arr, 0, 0);
}

fn test_n_queen(arr: &[u32]) -> bool {
    // let mut row_set: HashSet<u32> = HashSet::new();
    let mut col_set: HashSet<u32> = HashSet::new();
    let mut add_set: HashSet<u32> = HashSet::new();
    let mut sub_set: HashSet<u32> = HashSet::new();
    let mut is_ok = true;
    for (i, j) in arr.iter().enumerate() {
        let row = i as u32;
        let col = *j;
        if col_set.contains(&col) {
            is_ok = false;
            // println!("col: {:?} {}", col_set, col);
            break;
        } else {
            col_set.insert(col);
        }
        let add = row + col;
        if add_set.contains(&add) {
            is_ok = false;
            // println!("add: {:?} {}", add_set, add);
            break;
        } else {
            add_set.insert(add);
        }
        let sub = abs_sub(row, col);
        if sub_set.contains(&sub) {
            is_ok = false;
            // println!("sub: {:?} {}", sub_set, sub);
            break;
        } else {
            sub_set.insert(sub);
        }
    }
    is_ok
}

// fn ps(slice:&[&[u32]]) {
//     for p in slice {
//         let t = *p;
//         for s in t {
//             println!("ps {:?}", s);
//         }
//     }
// }

// fn ab(a: &[&[u32]]) {
//     for i in 0..a.len() {
//         let z = &a[i];
//         for j in 0..z.len() {
//             println!("ab {:?}", z[j]);
//         }
//     }
// }

fn min(a:u32, b:u32) -> u32 {
    if a < b { a } else { b }
}

fn test_matrix_2_queen(matrix: &[&[u32]]){
    println!("=====start matrix 2 queen in {} X {} cells", matrix.len(), matrix.len());
    let solution_summary = matrix_2_queen(matrix);
    println!("{} solutions ", solution_summary.total);
    for s in solution_summary.solutions {
        for (k, p) in s.pos.iter().enumerate() {
            println!("{}: {}{}", k, p.row, p.col);
        }
        println!("----");
    }
    println!("=====end matrix ");
}

fn test_arr_2_queen(matrix: &[u32], dimension: usize){
    println!("=====start array 2 queen in {} X {} cells", matrix.len(), matrix.len());
    let solution_summary = arr_2_queen(matrix, dimension);
    println!("{} solutions ", solution_summary.total);
    for s in solution_summary.solutions {
        for (k, p) in s.pos.iter().enumerate() {
            println!("{}: {}{}", k, p.row, p.col);
        }
        println!("----");
    }
    println!("=====end array ");
}

fn main() {
    // let coins = vec![5, 2, 1];
    // println!("{:?}", coins);
    // let arr = make_array!(32);
    // println!("{:?}", arr);
    // let arr = make_array!(16);
    // println!("{:?}", arr);

    // let mut arr_2_3:[[i32; 3]; 2] = [[0; 3]; 2];
    // let mut n: i32 = 1;
    // for i in 0..2 {
    //     for j in 0..3 {
    //         arr_2_3[i][j] = n * 10;
    //         n = n + 1;
    //     }
    // }
    // for i in 0..2 {
    //     for j in 0..3 {
    //         println!("{}", arr_2_3[i][j])
    //     }
    // }

    // let mut arr_2_3:[[i32; 3]; 2] = [[0; 3]; 2];
    // for (i, row) in arr_2_3.iter().enumerate() {
    //     for (j, col) in row.iter().enumerate() {
    //         println!("{}, {}: {}", i, j, col)
    //     }
    // }
    // let mut n: i32 = 1;
    // for i in 0..2{
    //     for j in 0..3{
    //         arr_2_3[i][j] = n * 10;
    //         n = n + 1;
    //     }
    // }
    // for (i, row) in arr_2_3.iter().enumerate() {
    //     for (j, col) in row.iter().enumerate() {
    //         println!("{}, {}: {}", i, j, col)
    //     }
    // }

    // let arr = [1, 2, 3];
    // match arr {
    //     [_, _, _] => println!("has three"),
    //     [a, b, 3] => println!("ends with three, {}, {}", a, b),
    //     [1, a, b] => println!("starts with one, {}, {}", a, b),
    //     [a, b, c] => println!("starts with something else"),
    // }
    // let v = vec![1, 2, 3];
    // match v[..] {
    //     [1, _, _] => println!("starts with one"),
    //     [a, b] => println!("match two eles"),
    //     [a, b, c] => println!("match three eles"),
    //     _ => println!("match other eles"),
    // }
    // let arr = [1, 2, 3];
    // match arr {
    //     [1, _, _] => "starts with one",
    //     [a, b, c] => "starts with something else",
    // };
    // ​
    // // Dynamic size
    // let v = vec![1, 2, 3];
    // match v[..] {
    //     [a, b] => { /* this arm will not apply because the length doesn't match */ }
    //     [a, b, c] => { /* this arm will apply */ }
    //     _ => { /* this wildcard is required, since the length is not known statically */ }
    // };
    // ps(&[&[1, 2], &[1, 2, 3], &[1, 2, 4, 5]]);
    // ab(&[&[1, 2], &[1, 2, 3], &[1, 2, 4, 5]]);
    // test_matrix_2_queen(&[&[0, 0], &[0, 0]]);
    // test_matrix_2_queen(&[&[0, 0, 0], &[0, 0, 0], &[0, 0, 0]]);
    // test_matrix_2_queen(&[&[0, 0, 0, 0], &[0, 0, 0, 0], &[0, 0, 0, 0], &[0, 0, 0, 0]]);
    // test_arr_2_queen(&[0, 0, 0, 0], 2);
    // test_arr_2_queen(&[0, 0, 0, 0, 0, 0, 0, 0, 0], 3);
    // test_arr_2_queen(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 4);

    // let k = vec![1, 2, 3, 4, 5];
    // let k = vec![1, 2, 3];
    // println!("{:#?} <<<>>>", &k[0..]);
    // let mut already_printed: HashSet<&[i32]> = HashSet::new();
    // combine_set(&k[0..], &mut already_printed);
    n_queen(&mut [0, 0, 0, 0, 0, 0, 0]);
}
