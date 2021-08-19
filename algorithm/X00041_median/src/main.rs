use std::rc::Rc;
use std::mem::swap;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;
// use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::thread;

trait BinaryHeap {
    fn size(&self) -> usize;
    fn uplimit(&self) -> usize;
    fn push(&mut self, value: u32);
    fn pop(&mut self) -> u32;
    fn top(&self) -> u32;
    fn swim(&mut self, index: usize);
    fn sink(&mut self, index: usize);
    fn left_index(&self, index: usize) -> usize;
    fn right_index(&self, index: usize) -> usize;
    fn parent_index(&self, index: usize) -> usize;
    fn exchange(&mut self, i: usize, j: usize);
    fn value(&self, k: usize) -> u32;
    fn elements(&self) -> Vec<u32>;
}

struct MaxBinaryHeap {
    eles: Vec<u32>,
    total: usize,
}

impl MaxBinaryHeap {
    fn new(capacity: usize) -> MaxBinaryHeap {
        MaxBinaryHeap {
            eles:Vec::with_capacity(capacity),
            total:0,
        }
    }
}

impl BinaryHeap for MaxBinaryHeap {
    fn elements(&self) -> Vec<u32> {
        self.eles.clone()
    }
    fn size(&self) -> usize {
        self.total
    }
    fn uplimit(&self) -> usize {
        self.total + 1
    }
    fn push(&mut self, value: u32) {
        self.total = self.total + 1;
        if self.total <= self.eles.len() {
            self.eles[self.total - 1] = value;
        } else {
            self.eles.push(value);
        }
        self.swim(self.total);
    }
    fn pop(&mut self) -> u32 {
        if self.total == 0 {
            return 0;
        }
        self.exchange(1, self.total);
        let top = self.value(self.total);
        self.eles[self.total - 1] = 0;
        self.total = self.total - 1;
        self.sink(1);
        top
    }
    fn top(&self) -> u32 {
        let top = self.value(1);
        top
    }
    fn swim(&mut self, index: usize) {
        if index > 0 && index < self.uplimit() {
            let mut index = index;
            let mut parent_index:usize;
            loop {
                parent_index = self.parent_index(index);
                if parent_index == 0 {
                    break
                }
                if self.value(index) > self.value(parent_index) {
                    self.exchange(index, parent_index);
                    index = parent_index;
                } else {
                    break
                }
            }
        }
    }
    fn sink(&mut self, index: usize) {
        if index > 0 && index < self.uplimit() {
            let mut index = index;
            let mut left_index:usize;
            let mut right_index:usize;
            loop {
                left_index = self.left_index(index);
                right_index = self.right_index(index);

                if self.value(left_index) > self.value(right_index) {
                    if self.value(index) < self.value(left_index) { // 不可能小于0
                        self.exchange(index, left_index);
                        index = left_index;
                    } else {
                        break
                    }
                } else {
                    if self.value(index) < self.value(right_index) { // 不可能小于0
                        self.exchange(index, right_index);
                        index = right_index;
                    } else {
                        break
                    }
                }
            }
        }
    }
    fn left_index(&self, index: usize) -> usize {
        if index > 0 {
            index * 2
        } else {
            0
        }
    }
    fn right_index(&self, index: usize) -> usize {
        if index > 0 {
            index * 2 + 1
        } else {
            0
        }
    }
    fn parent_index(&self, index: usize) -> usize {
        if index > 0 {
            index / 2
        } else {
            0
        }
    }

    fn exchange(&mut self, i: usize, j: usize) {
        if i > 0 && j > 0 && i < self.uplimit() && j < self.uplimit() && i != j {
            let temp = self.eles[i-1];
            self.eles[i-1] = self.eles[j-1];
            self.eles[j-1] = temp;
        }
    }
    fn value(&self, k: usize) -> u32 {
        if k > 0 && k < self.uplimit() {
            self.eles[k-1]
        } else {
            0
        }
    }
}

struct MinBinaryHeap {
    eles: Vec<u32>,
    total: usize,
}

impl MinBinaryHeap {
    fn new(capacity: usize) -> MinBinaryHeap {
        MinBinaryHeap {
            eles:Vec::with_capacity(capacity),
            total:0,
        }
    }
}

impl BinaryHeap for MinBinaryHeap {
    fn elements(&self) -> Vec<u32> {
        self.eles.clone()
    }
    fn size(&self) -> usize {
        self.total
    }
    fn uplimit(&self) -> usize {
        self.total + 1
    }
    fn push(&mut self, value: u32) {
        self.total = self.total + 1;
        if self.total <= self.eles.len() {
            self.eles[self.total - 1] = value;
        } else {
            self.eles.push(value);
        }
        self.swim(self.total);
    }
    fn pop(&mut self) -> u32 {
        if self.total == 0 {
            return 0;
        }
        self.exchange(1, self.total);
        let top = self.value(self.total);
        self.eles[self.total - 1] = 0;
        self.total = self.total - 1;
        self.sink(1);
        top
    }
    fn top(&self) -> u32 {
        let top = self.value(1);
        top
    }
    fn swim(&mut self, index: usize) {
        if index > 0 && index < self.uplimit() {
            let mut index = index;
            let mut parent_index:usize;
            loop {
                parent_index = self.parent_index(index);
                if self.value(index) < self.value(parent_index) {
                    self.exchange(index, parent_index);
                    index = parent_index;
                } else {
                    break
                }
            }
        }
    }
    fn sink(&mut self, index: usize) {
        if index > 0 && index < self.uplimit() {
            let mut index = index;
            let mut left_index:usize;
            let mut right_index:usize;
            loop {
                left_index = self.left_index(index);
                right_index = self.right_index(index);
                let left_value = self.value(left_index);
                let right_value = self.value(right_index);

                if left_value < right_value {
                    if left_value > 0 {
                        if self.value(index) > left_value {
                            self.exchange(index, left_index);
                            index = left_index;
                        } else {
                            break
                        }
                    } else {
                        if self.value(index) > right_value {
                            self.exchange(index, right_index);
                            index = right_index;
                        } else {
                            break
                        }
                    }
                } else {
                    if right_value > 0 {
                        if self.value(index) > right_value {
                            self.exchange(index, right_index);
                            index = right_index;
                        } else {
                            break
                        }
                    } else {
                        if self.value(index) > left_value {
                            self.exchange(index, left_index);
                            index = left_index;
                        } else {
                            break
                        }
                    }
                }
            }
        }
    }
    fn left_index(&self, index: usize) -> usize {
        if index > 0 {
            index * 2
        } else {
            0
        }
    }
    fn right_index(&self, index: usize) -> usize {
        if index > 0 {
            index * 2 + 1
        } else {
            0
        }
    }
    fn parent_index(&self, index: usize) -> usize {
        if index > 0 {
            index / 2
        } else {
            0
        }
    }

    fn exchange(&mut self, i: usize, j: usize) {
        if i > 0 && j > 0 && i < self.uplimit() && j < self.uplimit() && i != j {
            let temp = self.eles[i-1];
            self.eles[i-1] = self.eles[j-1];
            self.eles[j-1] = temp;
        }
    }
    fn value(&self, k: usize) -> u32 {
        if k > 0 && k < self.uplimit() {
            self.eles[k-1]
        } else {
            0
        }
    }
}

struct StreamMedian{
    large: MinBinaryHeap,
    small: MaxBinaryHeap,
    all: Vec<u32>,
}

impl StreamMedian {
    fn new() -> StreamMedian {
        StreamMedian {
            large:MinBinaryHeap::new(0),
            small:MaxBinaryHeap::new(0),
            all:Vec::new(),
        }
    }
    fn find_median(&self) -> f64 {
        if (self.large.size() < self.small.size()) {
            self.small.top() as f64
        } else if (self.large.size() > self.small.size()) {
            self.large.top() as f64
        } else {
            (self.large.top() + self.small.top()) as f64 / 2.0
        }
    }
    fn add(&mut self, value: u32) {
        self.all.push(value);
        if (self.large.size() <= self.small.size()) {
            self.small.push(value);
            let top = self.small.pop();
            self.large.push(top);
        } else {
            self.large.push(value);
            let top = self.large.pop();
            self.small.push(top);
        }
    }
}

fn test_binary_heap(mark: &str, bh:&mut BinaryHeap){
    println!("=====start {}", mark);
    bh.push(3);
    bh.push(2);
    bh.push(5);
    bh.push(4);
    bh.push(1);
    bh.push(7);
    bh.push(6);
    println!("list {:?}", bh.elements());
    loop {
        let top = bh.pop();
        if top == 0 {
            break;
        }
        print!("{} ", top);
    }
    println!("");
    bh.push(3);
    bh.push(2);
    bh.push(5);
    bh.push(4);
    bh.push(1);
    println!("list {:?}", bh.elements());
    loop {
        let top = bh.pop();
        if top == 0 {
            break;
        }
        print!("{} ", top);
    }
    println!("=====end {}", mark);
}

fn test_stream_median(mark: &str, sm:&mut StreamMedian){
    println!("=====start {}", mark);
    sm.add(3);
    println!("list {:?} middle {}", sm.all, sm.find_median());
    sm.add(2);
    println!("list {:?} middle {}", sm.all, sm.find_median());
    sm.add(5);
    println!("list {:?} middle {}", sm.all, sm.find_median());
    sm.add(4);
    println!("list {:?} middle {}", sm.all, sm.find_median());
    sm.add(1);
    println!("list {:?} middle {}", sm.all, sm.find_median());
    sm.add(7);
    println!("list {:?} middle {}", sm.all, sm.find_median());
    sm.add(6);
    println!("=====end {}", mark);
}

fn main() {
    let mut mbh = MaxBinaryHeap::new(4);
    test_binary_heap("max binary heap", &mut mbh);
    let mut mbh = MinBinaryHeap::new(4);
    test_binary_heap("min binary heap", &mut mbh);
    let mut sm = StreamMedian::new();
    test_stream_median("stream median", &mut sm);
}
