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
    length: u32,
    storage: HashMap<String, Rc<RefCell<Node>>>,
    index: Option<Rc<RefCell<Node>>>,
}

impl LRUCache {
    fn new(capacity: u32) -> LRUCache {
        LRUCache {
            capacity,
            length: 0,
            storage: HashMap::new(),
            index: None,
        }
    }

    fn fetch(&mut self, key: String) -> Option<Rc<RefCell<Node>>> {
        // if self.storage.contains_key(&key) {
        // } else {
        // }
        let node = self.storage.get(&key);
        match node {
            Some(rc_node) => {
                let mut update_index = false;
                {
                    let prev_node = rc_node.borrow();
                    let prev_node = prev_node.prev.as_ref();
                    let next_node = rc_node.borrow();
                    let next_node = next_node.next.as_ref();
                    match prev_node {
                        Some(rc_prev_node) => {
                            update_index = true;
                            match next_node {
                                Some(rc_next_node) => {
                                    let mut raw_prev_node = rc_prev_node.borrow_mut();
                                    raw_prev_node.next = Some(rc_next_node.clone());
                                    let mut raw_next_node = rc_next_node.borrow_mut();
                                    raw_next_node.prev = Some(rc_prev_node.clone());
                                }
                                None => {
                                    let mut raw_prev_node = rc_prev_node.borrow_mut();
                                    raw_prev_node.next = None;
                                }
                            }
                        }
                        None => {}
                    }
                }
    
                if update_index {
                    {
                        let mut raw_node = rc_node.borrow_mut();
                        raw_node.prev = None;
                        raw_node.next = None;
                    }
    
                    let index_node = self.index.as_ref();
                    match index_node {
                        Some(rc_index_node)=>{
                            {
                                let mut raw_index_node = rc_index_node.borrow_mut();
                                let mut raw_node = rc_node.borrow_mut();
                                raw_index_node.prev = Some(rc_node.clone());
                                raw_node.next = Some(rc_index_node.clone());
                            }
                            self.index = Some(rc_node.clone());
                        }
                        None=>{
                            self.index = Some(rc_node.clone());
                        }
                    }
                }
    
                Some(rc_node.clone())
            }
            None => {
                None
            }
        }
    }

    fn get(&mut self, key: String) -> u32 {
        let node = self.fetch(key);
        match node {
            Some(rc_node) => {
                rc_node.borrow().value
            }
            None => {
                0
            }
        }
    }

    fn put(&mut self, key: String, value: u32) {
        let node = self.fetch(key.clone());
        match node {
            Some(rc_node) => {
                let mut raw_node = rc_node.borrow_mut();
                raw_node.value = value;
            }
            None => {
                let rc_node = Node::new(key.clone(), value);

                let index_node = self.index.as_ref();
                match index_node {
                    Some(rc_index_node)=>{
                        {
                            let mut raw_index_node = rc_index_node.borrow_mut();
                            let mut raw_node = rc_node.borrow_mut();
                            raw_index_node.prev = Some(rc_node.clone());
                            raw_node.next = Some(rc_index_node.clone());
                        }
                        self.index = Some(rc_node.clone());
                    }
                    None=>{
                        self.index = Some(rc_node.clone());
                    }
                }

                self.storage.insert(key.clone(), rc_node);
                self.length = self.length + 1;
                if self.length > self.capacity + 3 {
                    self.clear();
                }
            }
        }
    }

    fn clear(&mut self) {
        let mut total = 0;
        let mut delete_node: Option<Rc<RefCell<Node>>> = None;
        let mut node = self.index.clone();
        while let Some(rc_node) = node {
            total = total + 1;

            if total > self.capacity {
                match delete_node {
                    Some(_) => {}
                    None => {
                        delete_node = Some(rc_node.clone());
                    }
                }
                self.storage.remove(&rc_node.borrow().key);
            }

            node = rc_node.borrow().next.clone();
        }

        match delete_node {
            Some(rc_delete_node) => {
                let prev_node = rc_delete_node.borrow().prev.clone();
                match prev_node {
                    Some(rc_prev_node) => {
                        let mut raw_prev_node = rc_prev_node.borrow_mut();
                        raw_prev_node.next = None;
                    }
                    None => {}
                }
            }
            None => {}
        }

        self.length = self.capacity;
    }

    fn display(&self) {
        println!("========= current state start");
        let mut total = 0;
        let mut node = self.index.clone();
        while let Some(rc_node) = node {
            total = total + 1;
            let raw_node = rc_node.borrow();
            println!("#{} {} {}", total, raw_node.key, raw_node.value);
            node = raw_node.next.clone();
        }
        println!("========= current state end");
    }
}

struct PathNode {
    value: u32,
    is_leaf: bool,
}

fn main() {
    let mut lru_c = LRUCache::new(3);
    lru_c.put(String::from("hello"), 1);
    lru_c.put(String::from("world"), 2);
    lru_c.put(String::from("hao"), 3);
    lru_c.display();
    lru_c.put(String::from("kuan"), 4);
    lru_c.put(String::from("red"), 42);
    lru_c.put(String::from("black"), 25);
    lru_c.display();
    lru_c.put(String::from("true"), 2);
    lru_c.display();
    lru_c.put(String::from("false"), 2);
    lru_c.display();
    // println!("{}", lru_c.get(String::from("hello")));
    // println!("{}", lru_c.get(String::from("world")));
    // println!("{}", lru_c.get(String::from("hao")));
    // println!("{}", lru_c.get(String::from("kuan")));
    // let mut a = PathNode{
    //     value: 1,
    //     is_leaf: false,
    // };
    // let b = &a;
    // // b.is_leaf = true;
    // let d = &a;
    // println!("{}", d.is_leaf);
    // println!("{}", b.is_leaf);
    // let mut c = &mut a;
    // let mut f = &mut a;
    // f.is_leaf = false;
    // println!("{}", c.is_leaf);
    // c.is_leaf = true;
    println!("Hello, world!");
}
