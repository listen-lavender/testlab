use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

struct Node {
    key: String,
    value: u32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: String, value: u32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

struct LRUCache {
    capacity: u32,
    storage: HashMap<String, Rc<RefCell<Node>>>,
    index: Option<Rc<RefCell<Node>>>,
}

impl LRUCache {
    fn new(capacity: u32) -> LRUCache {
        LRUCache {
            capacity,
            storage: HashMap::new(),
            index: None,
        }
    }

    fn get(&mut self, key: String) -> u32 {
        if self.storage.contains_key(&key) {
            let node:&Rc<RefCell<Node>> = self.storage.get(&key).unwrap();
            let prev_node = node.borrow();
            let prev_node = prev_node.prev.as_ref();
            let next_node = node.borrow();
            let next_node = next_node.next.as_ref();
            match prev_node {
                Some(_) => {
                    match next_node {
                        Some(_) => {
                            prev_node.unwrap().borrow_mut().next = Some(next_node.unwrap().clone());
                            next_node.unwrap().borrow_mut().prev = Some(prev_node.unwrap().clone());
                        }
                        None => {
                            prev_node.unwrap().borrow_mut().next = None;
                        }
                    }
                    
                    let index_node = self.index.as_ref();
                    match index_node {
                        Some(_)=>{
                            node.borrow_mut().next = Some(index_node.unwrap().clone());
                            index_node.unwrap().borrow_mut().prev = Some(node.clone());
                            self.index = Some(node.clone());
                        }
                        None=>{
                            self.index = Some(node.clone());
                        }
                    }
                }
                None => {}
            }

            node.borrow().value
        } else {
            0
        }
    }

    fn put(&mut self, key: String, value: u32) {
        if self.storage.contains_key(&key) {
            let node:&Rc<RefCell<Node>> = self.storage.get(&key).unwrap();
            let prev_node = node.borrow();
            let prev_node = prev_node.prev.as_ref();
            let next_node = node.borrow();
            let next_node = next_node.next.as_ref();
            match prev_node {
                Some(_) => {
                    match next_node {
                        Some(_) => {
                            prev_node.unwrap().borrow_mut().next = Some(next_node.unwrap().clone());
                            next_node.unwrap().borrow_mut().prev = Some(prev_node.unwrap().clone());
                        }
                        None => {
                            prev_node.unwrap().borrow_mut().next = None;
                        }
                    }
                    
                    let index_node = self.index.as_ref();
                    match index_node {
                        Some(_)=>{
                            node.borrow_mut().next = Some(index_node.unwrap().clone());
                            index_node.unwrap().borrow_mut().prev = Some(node.clone());
                            self.index = Some(node.clone());
                        }
                        None=>{
                            self.index = Some(node.clone());
                        }
                    }
                }
                None => {}
            }
        } else {
            let node = Node::new(key.clone(), value);

            let index_node = self.index.as_ref();
            match index_node {
                Some(_)=>{
                    node.borrow_mut().next = Some(index_node.unwrap().clone());
                    index_node.unwrap().borrow_mut().prev = Some(node.clone());
                    self.index = Some(node.clone());
                }
                None=>{
                    self.index = Some(node.clone());
                }
            }

            self.storage.insert(key.clone(), node);
            self.capacity = self.capacity + 1;
            self.clear();
        }
    }

    fn clear(&self) {
        // let mut count = 0;
        // let mut current_node = self.index.as_ref();
        // while let Some(node) = current_node {
        //   count = count + 1;
        //   let ref_node = node.borrow();
        //   current_node = ref_node.next.as_ref().map(|node| &*node)
        // }

        // let mut total = 0;
        // let mut delete_node: Option<Rc<RefCell<Node>>> = None;
        // let mut node = &Some(self.index.as_ref().unwrap().clone());
        // while let Some(raw_node) = node {
        //     total = total + 1;
        //     let ref_raw_node = raw_node.borrow();
        //     let node = &ref_raw_node.next.clone();

        //     if total > self.capacity {
        //         match delete_node {
        //             Some(_) => {}
        //             None => {
        //                 delete_node = node.clone();
        //             }
        //         }
        //         self.storage.remove(&node.as_ref().unwrap().borrow().key);
        //     }
        // }

        // let prev_node = delete_node.unwrap().borrow().prev;
        // let next_node = delete_node.unwrap().borrow().next;
        // match prev_node {
        //     Some(raw_node) => {
        //         prev_node.unwrap().borrow_mut().next = next_node;
        //     }
        //     None => {}
        // }
        // match next_node {
        //     Some(raw_node) => {
        //         next_node.unwrap().borrow_mut().prev = prev_node;
        //     }
        //     None => {}
        // }
    }

    fn display(&self) {
        // let mut node = self.index;
        // let mut total = 0;
        // let mut delete_node: Option<Rc<RefCell<Node>>> = None;
        // while node.is_some() {
        //     total = total + 1;
        //     node = node.unwrap().borrow().next;
        //     if total > self.capacity {
        //         match delete_node {
        //             Some(raw_node) => {}
        //             None => {
        //                 delete_node = node;
        //             }
        //         }
        //         self.storage.remove(&node.unwrap().borrow().key);
        //     }
        // }
        // let prev_node = delete_node.unwrap().borrow().prev;
        // let next_node = delete_node.unwrap().borrow().next;
        // match prev_node {
        //     Some(raw_node) => {
        //         prev_node.unwrap().borrow_mut().next = next_node;
        //     }
        //     None => {}
        // }
        // match next_node {
        //     Some(raw_node) => {
        //         next_node.unwrap().borrow_mut().prev = prev_node;
        //     }
        //     None => {}
        // }
    }
}

fn main() {
    let mut lru_c = LRUCache::new(10);
    lru_c.put(String::from("hello"), 1);
    lru_c.put(String::from("world"), 2);
    lru_c.put(String::from("hao"), 3);
    lru_c.put(String::from("kuan"), 4);
    println!("{}", lru_c.get(String::from("hello")));
    println!("{}", lru_c.get(String::from("world")));
    println!("{}", lru_c.get(String::from("hao")));
    println!("{}", lru_c.get(String::from("kuan")));
    println!("Hello, world!");
}
