use std::rc::Rc;
use std::mem::swap;
use std::mem::size_of;
use std::cmp::max;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;

#[derive(Clone)]
struct Keyval {
    val: String,
    key: u32,
}

#[derive(Clone)]
struct MultiBTreeNode {
    parent: Option<Rc<RefCell<MultiBTreeNode>>>,
    children: Vec<Option<Rc<RefCell<MultiBTreeNode>>>>,
    kvs: Vec<Keyval>,
	is_leaf: bool,
    is_root: bool,
    m: usize,
}

enum KeyvalOrNode {
    Keyval(Keyval),
    Node(Option<Rc<RefCell<MultiBTreeNode>>>),
}

struct ChildOrKeyval {
    kvn: KeyvalOrNode,
    index: isize,
}

struct MultiBTreeAroundNode {
    left_neighbor: Option<Rc<RefCell<MultiBTreeNode>>>,
    left_index: isize,
    right_neighbor: Option<Rc<RefCell<MultiBTreeNode>>>,
    right_index: isize,
}

fn judge(key1:u32, key2:u32) -> bool {
    return key1 < key2
}

impl MultiBTreeNode {
    fn new(m: usize) -> Rc<RefCell<MultiBTreeNode>> {
        let mut is_root:bool = true;
        let mut is_leaf:bool = true;
        let children:Vec<Option<Rc<RefCell<MultiBTreeNode>>>> = Vec::new();
        Rc::new(RefCell::new(MultiBTreeNode{
            parent: None,
            children: children,
            is_root: is_root,
            is_leaf: is_leaf,
            kvs: Vec::new(),
            m: m,
        }))
    }
    fn set_parent(&mut self, parent_node:Option<Rc<RefCell<MultiBTreeNode>>>) {
        match parent_node {
            Some(raw_parent_node) => {
                // raw_parent_node.borrow_mut().is_root = true;
                raw_parent_node.borrow_mut().is_leaf = false;
                self.parent = Some(raw_parent_node.clone());
                self.is_root = false;
            }
            None => {}
        }
    }
    fn parent_node(&self) -> Option<Rc<RefCell<MultiBTreeNode>>> {
        match &self.parent {
            Some(parent_node) => {
                Some(parent_node.clone())
            }
            None => {None}
        }
    }
    fn is_not_full(&self) -> bool {
        self.children.len() < self.m && self.kvs.len() < (self.m - 1)
    }
    fn is_not_empty(&self) -> bool {
        if !self.is_root {
            let half = (self.m - 1) / 2 + 1;
            self.children.len() > half && self.kvs.len() > half - 1
        } else {
            self.children.len() > 2 && self.kvs.len() > 1
        }
    }
    fn be_notleaf(&mut self) {
        self.is_leaf = false
    }
    fn be_notroot(&mut self) {
        self.is_root = false
    }
    fn get_kv(&self, index:usize) -> Keyval {
        Keyval{
            key:self.kvs[index].key,
            val:self.kvs[index].val.clone(),
        }
    }
    fn get_last_kv(&self) -> Keyval {
        if self.kvs.len() > 0 {
            Keyval{
                key:self.kvs[self.kvs.len()-1].key,
                val:self.kvs[self.kvs.len()-1].val.clone(),
            }
        } else {
            Keyval{
                key:0,
                val:"".to_string(),
            }
        }
    }
    fn get_first_kv(&self) -> Keyval {
        if self.kvs.len() > 0 {
            Keyval{
                key:self.kvs[0].key,
                val:self.kvs[0].val.clone(),
            }
        } else {
            Keyval{
                key:0,
                val:"".to_string(),
            }
        }
    }
    fn set_kv(&mut self, index:usize, val: String) {
        self.kvs[index].val = val;
    }
    fn is_kv_index_ok(&self, index:usize) -> bool {
        index < self.kvs.len()
    }
    fn get_child(&self, index:usize) -> Option<Rc<RefCell<MultiBTreeNode>>> {
        if index < self.children.len() {
            match &self.children[index] {
                Some(child_node) => {
                    Some(child_node.clone())
                }
                None => {None}
            }
        } else {
            None
        }
    }
    fn get_first_child(&self) -> Option<Rc<RefCell<MultiBTreeNode>>> {
        if self.children.len() > 0 {
            match &self.children[0] {
                Some(child_node) => {
                    Some(child_node.clone())
                }
                None => {None}
            }
        } else {
            None
        }
    }
    fn get_last_child(&self) -> Option<Rc<RefCell<MultiBTreeNode>>> {
        if self.children.len() > 0 {
            match &self.children[self.children.len() - 1] {
                Some(child_node) => {
                    Some(child_node.clone())
                }
                None => {None}
            }
        } else {
            None
        }
    }
    fn is_child_index_ok(&self, index:usize) -> bool {
        index < self.children.len()
    }
    fn add_keyval(&mut self, kv: Keyval) -> usize {
        let mut insert_index:usize = 0;
        for (ind, ele) in self.kvs.iter().enumerate() {
            insert_index = ind;
            if judge(kv.key, ele.key) {
                break
            }
        }
        self.kvs.insert(insert_index, kv);
        self.children.insert(insert_index, None);
        insert_index
    }
    fn search_key_index(&self, k:u32) -> usize {
        let mut left:usize = 0;
        let mut right:usize = self.kvs.len() - 1;
        while left < right {
            let mid:usize = left + (right - left)/2;
            if k < self.kvs[mid].key {
                right = mid;
            } else if k > self.kvs[mid].key {
                left = mid + 1;
            } else {
                left = mid;
                right = mid;
            }
        }
        left
    }
    fn get_child_or_keyval(&self, k: u32) -> ChildOrKeyval {
        if self.kvs.len() == 0 {
            return ChildOrKeyval{
                kvn:KeyvalOrNode::Node(None),
                index:-1,
            }
        }
        let match_index = self.search_key_index(k);
        if k == self.kvs[match_index].key {
            ChildOrKeyval{
                kvn:KeyvalOrNode::Keyval(self.get_kv(match_index as usize)),
                index:match_index as isize,
            }
        } else if k < self.kvs[match_index].key {
            let next_index = match_index;
            ChildOrKeyval{
                kvn:KeyvalOrNode::Node(self.get_child(next_index)),
                index:next_index as isize,
            }
        } else {
            let next_index = match_index + 1;
            ChildOrKeyval{
                kvn:KeyvalOrNode::Node(self.get_child(next_index)),
                index:next_index as isize,
            }
        }
        // let mut match_index:isize = -1;
        // let mut next_index:isize = -1;
        // for (ind, ele) in self.kvs.iter().enumerate() {
        //     if k == ele.key {
        //         match_index = ind as isize;
        //         break
        //     } else if judge(k, ele.key) {
        //         next_index = ind as isize;
        //         break
        //     }
        // }
        // if match_index > - 1 {
        //     ChildOrKeyval{
        //         kvn:KeyvalOrNode::Keyval(self.get_kv(match_index as usize)),
        //         index:match_index,
        //     }
        // } else if next_index > -1 {
        //     ChildOrKeyval{
        //         kvn:KeyvalOrNode::Node(self.get_child(next_index as usize)),
        //         index:next_index,
        //     }
        // } else {
        //     next_index = self.kvs.len() as isize;
        //     ChildOrKeyval{
        //         kvn:KeyvalOrNode::Node(self.get_child(next_index as usize)),
        //         index:next_index,
        //     }
        // }
    }
    fn remove_keyval(&mut self, match_index:usize, k: u32) {
        // let match_index = self.search_key_index(k);
        if k == self.kvs[match_index].key {
            self.kvs.remove(match_index);
            self.children.remove(match_index);
        }
    }
}

struct MultiBTree {
    root: Option<Rc<RefCell<MultiBTreeNode>>>,
    m: usize,
    height: usize,
}

impl MultiBTree {
    fn new(m: usize) -> MultiBTree {
        MultiBTree{
            root: None,
            m: m,
            height: 0,
        }
    }
    fn mid_index(&self) -> usize {
        (self.m - 1) / 2 + 1
    }
    fn around(&self, node:Rc<RefCell<MultiBTreeNode>>) -> MultiBTreeAroundNode {
        let mut aroundNode = MultiBTreeAroundNode{
            left_neighbor:None,
            left_index:-1,
            right_neighbor:None,
            right_index:-1,
        };
        let parent_node = node.borrow().parent_node();
        match parent_node {
            Some(raw_parent_node) => {
                let index = raw_parent_node.borrow().search_key_index(node.borrow().kvs[0].key);
                let index_kv = raw_parent_node.borrow().get_kv(index);

                let mut left_index = index as isize;
                let mut right_index = index as isize + 2;

                if node.borrow().kvs[0].key < index_kv.key {
                    left_index = index as isize - 1;
                    right_index = index as isize + 1;
                }

                if left_index > -1 {
                    aroundNode.left_neighbor = raw_parent_node.borrow().get_child(left_index as usize);
                    aroundNode.left_index = left_index;
                }

                if raw_parent_node.borrow().is_child_index_ok(right_index as usize) {
                    aroundNode.right_neighbor = raw_parent_node.borrow().get_child(right_index as usize);
                    aroundNode.right_index = right_index;
                }
            }
            None => {}
        }
        aroundNode
    }
    fn split(&mut self, node:Option<Rc<RefCell<MultiBTreeNode>>>) -> usize {
        let mut index:usize = 0;
        match node {
            Some(raw_node) => {
                let mid_index = self.mid_index();
                let raw_other_node = MultiBTreeNode::new(self.m);
                let parent_node = raw_node.borrow().parent_node();
                
                raw_other_node.borrow_mut().be_notroot();
                raw_other_node.borrow_mut().is_leaf = raw_node.borrow().is_leaf;
                raw_other_node.borrow_mut().kvs = raw_node.borrow_mut().kvs.split_off(mid_index + 1);
                raw_other_node.borrow_mut().children = raw_node.borrow_mut().children.split_off(mid_index+1);
                for child_node in raw_other_node.borrow().children.iter() {
                    match child_node {
                        Some(raw_child_node) => {
                            raw_child_node.borrow_mut().set_parent(Some(raw_other_node.clone()));
                        }
                        None => {}
                    }
                }

                raw_node.borrow_mut().kvs.resize(mid_index, Keyval{
                    key:0,
                    val:"".to_string(),
                });
                raw_node.borrow_mut().children.resize(mid_index, None);
                
                match parent_node {
                    Some(raw_parent_node) => {
                        raw_other_node.borrow_mut().set_parent(Some(raw_parent_node.clone()));
                        let kv = raw_node.borrow().get_kv(mid_index);
                        index = raw_parent_node.borrow_mut().add_keyval(kv);
                        raw_parent_node.borrow_mut().children.insert(index, Some(raw_node.clone()));
                        raw_parent_node.borrow_mut().children.insert(index + 1, Some(raw_other_node.clone()));
                    }
                    None => {}
                }
            }
            None => {}
        }
        index
    }
    fn implement_keyval(&mut self, node:Option<Rc<RefCell<MultiBTreeNode>>>, k:u32, v:String) -> Option<Rc<RefCell<MultiBTreeNode>>> {
        let mut next_node:Option<Rc<RefCell<MultiBTreeNode>>> = None;
        match node {
            Some(raw_node) => {
                let child_or_keyval = raw_node.borrow().get_child_or_keyval(k);
                match child_or_keyval.kvn {
                    KeyvalOrNode::Keyval(kv) => {
                        raw_node.borrow_mut().set_kv(child_or_keyval.index as usize, v.clone());
                    }
                    KeyvalOrNode::Node(around_node) => {
                        match around_node {
                            Some(raw_around_node) => {
                                if !raw_node.borrow().is_not_full() {
                                    let mut parent_node = raw_node.borrow().parent_node();
                                    if raw_node.borrow().is_root {
                                        let raw_parent_node = MultiBTreeNode::new(self.m);
                                        raw_node.borrow_mut().set_parent(Some(raw_parent_node.clone()));
                                        self.root = Some(raw_parent_node.clone());
                                        parent_node = Some(raw_parent_node.clone());
                                    }
                                    let split_index = self.split(Some(raw_node.clone()));
                                    match parent_node {
                                        Some(raw_parent_node) => {
                                            if raw_parent_node.borrow().kvs[split_index].key == k {
                                                raw_parent_node.borrow_mut().set_kv(split_index, v.clone());
                                            } else if k < raw_parent_node.borrow().kvs[split_index].key {;
                                                next_node = raw_parent_node.borrow().get_child(split_index);
                                            } else {
                                                next_node = raw_parent_node.borrow().get_child(split_index+1);
                                            }
                                        }
                                        None => {}
                                    }
                                } else if raw_node.borrow().is_leaf {
                                    raw_node.borrow_mut().add_keyval(Keyval{
                                        key:k,
                                        val:v,
                                    });
                                } else {
                                    next_node = Some(raw_around_node.clone());
                                }
                            }
                            None => {
                                raw_node.borrow_mut().add_keyval(Keyval{
                                    key:k,
                                    val:v,
                                });
                            }
                        }
                    }
                }
            }
            None => {}
        }
        next_node
    }
    fn insert_or_update(&mut self, k:u32, v:String) {
        let mut node:Option<Rc<RefCell<MultiBTreeNode>>> = None;
        match &self.root {
            Some(raw_root_node) => {
                node = Some(raw_root_node.clone());
            }
            None => {
                let raw_root_node = MultiBTreeNode::new(self.m);
                self.root = Some(raw_root_node.clone());
                node = Some(raw_root_node.clone());
            }
        }
        while let Some(raw_node) = node {
            node = self.implement_keyval(Some(raw_node.clone()), k, v.clone());
        }
    }
    fn merge(&mut self, left_node:Option<Rc<RefCell<MultiBTreeNode>>>, right_node:Option<Rc<RefCell<MultiBTreeNode>>>, mid_kv:Keyval) -> Option<Rc<RefCell<MultiBTreeNode>>> {
        match &left_node {
            Some(raw_left_node) => {
                raw_left_node.borrow_mut().add_keyval(mid_kv);
                match &right_node {
                    Some(raw_right_node) => {
                        // let mut kvs = raw_left_node.borrow_mut().kvs;
                        // raw_right_node.borrow_mut().kvs.extend(&mut kvs);
                        for kv in raw_left_node.borrow().kvs.iter() {
                            raw_right_node.borrow_mut().kvs.push(kv.clone());
                        }
                        for child_node in raw_left_node.borrow().children.iter() {
                            match child_node {
                                Some(raw_child_node) => {
                                    raw_child_node.borrow_mut().set_parent(Some(raw_right_node.clone()));
                                    raw_right_node.borrow_mut().children.push(Some(raw_child_node.clone()));
                                }
                                None => {
                                    raw_right_node.borrow_mut().children.push(None);
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
        right_node
    }
    fn delete(&mut self, k:u32) {
        let mut k = k;
        match &self.root {
            Some(raw_root_node) => {
                let mut node = Some(raw_root_node.clone());
                while let Some(raw_node) = node {
                    let mut child_or_keyval = raw_node.borrow().get_child_or_keyval(k);
                    match child_or_keyval.kvn {
                        KeyvalOrNode::Keyval(kv) => {
                            let pre_child = raw_node.borrow().get_child(child_or_keyval.index as usize);
                            let post_child = raw_node.borrow().get_child(child_or_keyval.index as usize + 1);
                            match pre_child {
                                Some(raw_pre_child) => {
                                    if raw_pre_child.borrow().is_not_empty() {
                                        let last_pre_kv = raw_pre_child.borrow().get_last_kv();
                                        k = last_pre_kv.key;
                                        raw_pre_child.borrow_mut().kvs[child_or_keyval.index as usize] = last_pre_kv;
                                        node = Some(raw_pre_child.clone());
                                    } else {
                                        match post_child {
                                            Some(raw_post_child) => {
                                                if raw_post_child.borrow().is_not_empty() {
                                                    let first_post_kv = raw_post_child.borrow().get_first_kv();
                                                    k = first_post_kv.key;
                                                    raw_post_child.borrow_mut().kvs[child_or_keyval.index as usize] = first_post_kv;
                                                    node = Some(raw_post_child.clone());
                                                } else {
                                                    let merge_node = self.merge(Some(raw_pre_child.clone()), Some(raw_post_child.clone()), raw_node.borrow().get_kv(child_or_keyval.index as usize));
                                                    raw_node.borrow_mut().remove_keyval(child_or_keyval.index as usize, k);
                                                    if !raw_node.borrow().is_not_empty() {
                                                        match &merge_node {
                                                            Some(raw_merge_node) => {
                                                                raw_merge_node.borrow_mut().set_parent(None);
                                                                self.root = Some(raw_merge_node.clone());
                                                                self.height = self.height - 1;
                                                            }
                                                            None => {}
                                                        }
                                                    }
                                                    node = merge_node;
                                                }
                                            }
                                            None => {
                                                let merge_node = self.merge(Some(raw_pre_child.clone()), None, raw_node.borrow().get_kv(child_or_keyval.index as usize));
                                                raw_node.borrow_mut().remove_keyval(child_or_keyval.index as usize, k);
                                                if !raw_node.borrow().is_not_empty() {
                                                    match &merge_node {
                                                        Some(raw_merge_node) => {
                                                            raw_merge_node.borrow_mut().set_parent(None);
                                                            self.root = Some(raw_merge_node.clone());
                                                            self.height = self.height - 1;
                                                        }
                                                        None => {}
                                                    }
                                                }
                                                node = merge_node;
                                            }
                                        }
                                    }
                                }
                                None => {
                                    match post_child {
                                        Some(raw_post_child) => {
                                            if raw_post_child.borrow().is_not_empty() {
                                                let first_post_kv = raw_post_child.borrow().get_first_kv();
                                                k = first_post_kv.key;
                                                raw_post_child.borrow_mut().kvs[child_or_keyval.index as usize] = first_post_kv;
                                                node = Some(raw_post_child.clone());
                                            } else {
                                                let merge_node = self.merge(None, Some(raw_post_child.clone()), raw_node.borrow().get_kv(child_or_keyval.index as usize));
                                                raw_node.borrow_mut().remove_keyval(child_or_keyval.index as usize, k);
                                                if !raw_node.borrow().is_not_empty() {
                                                    match &merge_node {
                                                        Some(raw_merge_node) => {
                                                            raw_merge_node.borrow_mut().set_parent(None);
                                                            self.root = Some(raw_merge_node.clone());
                                                            self.height = self.height - 1;
                                                        }
                                                        None => {}
                                                    }
                                                }
                                                node = merge_node;
                                            }
                                        }
                                        None => {
                                            let merge_node = self.merge(None, None, raw_node.borrow().get_kv(child_or_keyval.index as usize));
                                            raw_node.borrow_mut().remove_keyval(child_or_keyval.index as usize, k);
                                            if !raw_node.borrow().is_not_empty() {
                                                match &merge_node {
                                                    Some(raw_merge_node) => {
                                                        raw_merge_node.borrow_mut().set_parent(None);
                                                        self.root = Some(raw_merge_node.clone());
                                                        self.height = self.height - 1;
                                                    }
                                                    None => {}
                                                }
                                            }
                                            node = merge_node;
                                        }
                                    }
                                }
                            }
                        }
                        KeyvalOrNode::Node(maybe_node) => {
                            match maybe_node {
                                Some(raw_maybe_node) => {
                                    if raw_maybe_node.borrow().is_not_empty() {
                                        node = Some(raw_maybe_node.clone());
                                    } else {
                                        let around_node = self.around(raw_maybe_node.clone());
                                        let mut left_neighbor = around_node.left_neighbor;
                                        let left_index = around_node.left_index;
                                        let mut right_neighbor = around_node.right_neighbor;
                                        let right_index = around_node.right_index;
                                        if left_neighbor.is_none() {
                                            left_neighbor = Some(raw_node.clone());
                                            child_or_keyval.index = right_index;
                                        } else {
                                            right_neighbor = Some(raw_node.clone());
                                            child_or_keyval.index = left_index;
                                        }
                                        match left_neighbor {
                                            Some(raw_left_neighbor) => {
                                                if raw_left_neighbor.borrow().is_not_empty() {
                                                    match right_neighbor {
                                                        Some(raw_right_neighbor) => {
                                                            raw_right_neighbor.borrow_mut().kvs.insert(0, raw_node.borrow().get_kv(child_or_keyval.index as usize));
                                                            let left_neighbor_last_node = raw_left_neighbor.borrow_mut().get_last_child();
                                                            if !raw_left_neighbor.borrow().is_leaf {
                                                                match &left_neighbor_last_node {
                                                                    Some(raw_left_neighbor_last_node) => {
                                                                        raw_left_neighbor_last_node.borrow_mut().set_parent(Some(raw_right_neighbor.clone()));
                                                                    }
                                                                    None => {}
                                                                }
                                                            }
                                                            raw_right_neighbor.borrow_mut().children.insert(0, Some(raw_left_neighbor.clone()));
                                                            raw_node.borrow_mut().kvs[child_or_keyval.index as usize] = raw_left_neighbor.borrow().get_last_kv();
                                                            let _ = raw_left_neighbor.borrow_mut().kvs.split_off(raw_left_neighbor.borrow().kvs.len()-1);
                                                            let _ = raw_left_neighbor.borrow_mut().children.split_off(raw_left_neighbor.borrow().children.len()-1);
                                                            node = Some(raw_right_neighbor.clone());
                                                        }
                                                        None => {
                                                            node = None
                                                        }
                                                    }
                                                } else {
                                                    match right_neighbor {
                                                        Some(raw_right_neighbor) => {
                                                            if raw_right_neighbor.borrow().is_not_empty() {
                                                                raw_left_neighbor.borrow_mut().kvs.insert(0, raw_node.borrow().get_kv(child_or_keyval.index as usize));
                                                                let right_neighbor_first_node = raw_right_neighbor.borrow_mut().get_first_child();
                                                                if !raw_right_neighbor.borrow().is_leaf {
                                                                    match &right_neighbor_first_node {
                                                                        Some(raw_right_neighbor_last_node) => {
                                                                            raw_right_neighbor_last_node.borrow_mut().set_parent(Some(raw_left_neighbor.clone()));
                                                                        }
                                                                        None => {}
                                                                    }
                                                                }
                                                                raw_left_neighbor.borrow_mut().children.insert(0, Some(raw_right_neighbor.clone()));
                                                                raw_node.borrow_mut().kvs[child_or_keyval.index as usize] = raw_right_neighbor.borrow().get_first_kv();
                                                                let _ = raw_right_neighbor.borrow_mut().kvs.remove(0);
                                                                let _ = raw_right_neighbor.borrow_mut().children.remove(0);
                                                                node = Some(raw_left_neighbor.clone());
                                                            } else {
                                                                let merge_node = self.merge(Some(raw_left_neighbor.clone()), Some(raw_right_neighbor.clone()), raw_node.borrow().get_kv(child_or_keyval.index as usize));
                                                                raw_node.borrow_mut().remove_keyval(child_or_keyval.index as usize, k);
                                                                if !raw_node.borrow().is_not_empty() {
                                                                    match &merge_node {
                                                                        Some(raw_merge_node) => {
                                                                            raw_merge_node.borrow_mut().set_parent(None);
                                                                            self.root = Some(raw_merge_node.clone());
                                                                            self.height = self.height - 1;
                                                                        }
                                                                        None => {}
                                                                    }
                                                                }
                                                                node = merge_node;
                                                            }
                                                        }
                                                        None => {
                                                            node = None;
                                                        }
                                                    }
                                                }
                                            }
                                            None => {
                                                node = None;
                                            }
                                        }
                                        node = None;
                                    }
                                }
                                None => {
                                    node = None;
                                }
                            }
                        }
                    }
                }
            }
            None => {}
        }
    }
}

fn main() {
}
