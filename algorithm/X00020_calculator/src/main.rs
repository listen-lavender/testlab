use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

fn calculate(express: &[char]) -> i32 {
    // println!("start {}", to_string(express));
    let mut stack:Vec<i32> = Vec::new();
    let mut sub_process = false;
    let mut sub_start = 0;
    let mut sub_end = 0;
    let mut sub_count:u32 = 0;
    let mut curr_n:i32 = 0;
    let mut prefix_flag:char = express[0];
    if prefix_flag != '-' {
        prefix_flag = '+';
    }
    for (ind, c) in express.iter().enumerate() {
        if *c == '(' {
            if !sub_process {
                sub_process = true;
                sub_start = ind + 1;
            }
            sub_count = sub_count + 1;
            continue
        } else if *c == ')' {
            sub_count = sub_count - 1;
            if sub_count == 0 {
                sub_process = false;
                sub_end = ind;
                curr_n = calculate(&express[sub_start..sub_end]);
                // println!("-- {}", curr_n);
            }
        }
        if sub_count == 0 {
            if is_digit(*c) {
                // println!(">>> {}", *c);
                curr_n = curr_n * 10 + (*c as i32 - '0' as i32);
            } else if *c == ' ' {
            } else {
                if prefix_flag == '+' {
                    stack.push(curr_n);
                    curr_n = 0;
                } else if prefix_flag == '-' {
                    stack.push(0 - curr_n);
                    curr_n = 0;
                } else if prefix_flag == '*' {
                    let prev_n = stack.pop();
                    match prev_n {
                        Some(raw_prev_n) => {
                            // println!("{}", raw_prev_n * curr_n);
                            stack.push(raw_prev_n * curr_n);
                        }
                        None => {}
                    }
                    curr_n = 0;
                } else if prefix_flag == '/' {
                    let prev_n = stack.pop();
                    match prev_n {
                        Some(raw_prev_n) => {
                            stack.push(raw_prev_n / curr_n);
                        }
                        None => {}
                    }
                    curr_n = 0;
                } else {

                }
                if *c == '+' || *c == '-' || *c == '*' || *c == '/' {
                    prefix_flag = *c;
                    // println!("++ {}", prefix_flag);
                } else {
                    prefix_flag = ' ';
                }
            }
        }
    }
    if curr_n > 0 {
        if prefix_flag == '+' {
            stack.push(curr_n);
        } else if prefix_flag == '-' {
            stack.push(0 - curr_n);
        } else if prefix_flag == '*' {
            let prev_n = stack.pop();
            match prev_n {
                Some(raw_prev_n) => {
                    stack.push(raw_prev_n * curr_n);
                }
                None => {}
            }
        } else if prefix_flag == '/' {
            let prev_n = stack.pop();
            match prev_n {
                Some(raw_prev_n) => {
                    stack.push(raw_prev_n / curr_n);
                }
                None => {}
            }
        }
    }
    let mut result:i32 = 0;
    for k in stack {
        // println!("{} ??", k);
        result = result + k;
    }
    result
}

fn to_string(cs:&[char]) -> String {
    let mut s = String::with_capacity(cs.len());
    for c in cs {
        s.push(*c);
    }
    s
}

fn is_digit(c: char) -> bool {
    let c = c as u32;
    c > 47 && c < 58
}

fn test_calculate(express: &[char]){
    println!("=====start equations");
    println!("{}={}", to_string(express), calculate(express));
    println!("=====end");
}

fn main() {
    test_calculate(&['1', '-', '1', '2', '+', '3']);
    test_calculate(&['2', '-', '3', '*', '4', '+', '5']);
    test_calculate(&['1', '4', '-', '5', '/', '2']);
    test_calculate(&['1', '+', '(', '2', '+', '3', '*', '(', '1', '4', '-', '5', '/', '2', ')', ')', '-', '1', '6']);
    // println!("{}", '0' as i32);
    // println!("{}", '1' as i32);
    // println!("{}", '8' as i32);
    // println!("{}", '9' as i32);
    println!("{}", -5/2);
}