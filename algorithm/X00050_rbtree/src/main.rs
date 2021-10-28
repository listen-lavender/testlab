use std::rc::Rc;
use std::mem::swap;
use std::mem::size_of;
use std::cmp::max;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;

enum MultiType {
    M2, M3
}

#[derive(Clone, Copy)]
enum Color {
    Red, Black
}

struct Multi23TreeNode {
    kind: MultiType,
    val1: u32,
    val2: u32,
    left: Option<Rc<RefCell<Multi23TreeNode>>>,
    middle: Option<Rc<RefCell<Multi23TreeNode>>>,
    right: Option<Rc<RefCell<Multi23TreeNode>>>,
}

impl Multi23TreeNode {
    fn new(vals:&[u32]) -> Option<Rc<RefCell<Multi23TreeNode>>> {
        if vals.len() == 0 {
            None
        } else if vals.len() == 1 {
            Some(Multi23TreeNode::new2node(vals[0]))
        } else {
            Some(Multi23TreeNode::new2node(vals[0]))
        }
    }
    fn new2node(val:u32) -> Rc<RefCell<Multi23TreeNode>> {
        Rc::new(RefCell::new(Multi23TreeNode {
            kind: MultiType::M2,
            val1: val,
            val2: 0,
            left: None,
            middle: None,
            right: None,
        }))
    }
    fn new3node(val1:u32, val2:u32) -> Rc<RefCell<Multi23TreeNode>> {
        Rc::new(RefCell::new(Multi23TreeNode {
            kind: MultiType::M3,
            val1: val1,
            val2: val2,
            left: None,
            middle: None,
            right: None,
        }))
    }

    fn left_node(&mut self) -> Option<Rc<RefCell<Multi23TreeNode>>> {
        let left_node = self.left.take();
        if left_node.is_some() {
            let left_node = left_node.unwrap();
            self.left = Some(left_node.clone());
            Some(left_node)
        } else {
            None
        }
    }

    fn middle_node(&mut self) -> Option<Rc<RefCell<Multi23TreeNode>>> {
        let middle_node = self.middle.take();
        if middle_node.is_some() {
            let middle_node = middle_node.unwrap();
            self.middle = Some(middle_node.clone());
            Some(middle_node)
        } else {
            None
        }
    }

    fn right_node(&mut self) -> Option<Rc<RefCell<Multi23TreeNode>>> {
        let right_node = self.right.take();
        if right_node.is_some() {
            let right_node = right_node.unwrap();
            self.right = Some(right_node.clone());
            Some(right_node)
        } else {
            None
        }
    }
}

struct BinaryRBTreeNode {
    color: Color,
    val: u32,
    parent: Option<Rc<RefCell<BinaryRBTreeNode>>>,
    left: Option<Rc<RefCell<BinaryRBTreeNode>>>,
    right: Option<Rc<RefCell<BinaryRBTreeNode>>>,
}

impl BinaryRBTreeNode {
    fn new(val: u32, parent: Option<Rc<RefCell<BinaryRBTreeNode>>>) -> Rc<RefCell<BinaryRBTreeNode>> {
        let mut color:Color = Color::Red;
        if parent.is_none() {
            color = Color::Black;
        }
        Rc::new(RefCell::new(BinaryRBTreeNode{
            color: color,
            val: val,
            parent: parent,
            left: None,
            right: None,
        }))
    }

    fn is_root(&self) -> bool {
        match self.parent {
            Some(_) => {
                false
            }
            None => {
                true
            }
        }
    }

    fn IS_ROOT(node: &Option<Rc<RefCell<BinaryRBTreeNode>>>) -> bool {
        match node {
            Some(raw_node) => {
                raw_node.borrow().is_root()
            }
            None => {
                false
            }
        }
    }

    fn IS_BLACK(node: &Option<Rc<RefCell<BinaryRBTreeNode>>>) -> bool {
        match node {
            Some(raw_node) => {
                if let Color::Black = raw_node.borrow().color {
                    true
                } else {
                    false
                }
            }
            None => {
                true
            }
        }
    }

    fn IS_RED(node: &Option<Rc<RefCell<BinaryRBTreeNode>>>) -> bool {
        match node {
            Some(raw_node) => {
                if let Color::Red = raw_node.borrow().color {
                    true
                } else {
                    false
                }
            }
            None => {
                false
            }
        }
    }

    fn parent_node(&self) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        match &self.parent {
            Some(parent_node) => {
                Some(parent_node.clone())
            }
            None => {None}
        }
    }

    fn brother_node(&self) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        match self.parent_node() {
            Some(parent_node) => {
                let brother_node = parent_node.borrow().left_node();
                if brother_node.is_none() {
                    return brother_node
                }
                match brother_node {
                    Some(raw_node) => {
                        if raw_node.borrow().val != self.val {
                            return Some(raw_node.clone())
                        }
                        parent_node.borrow().right_node()
                    }
                    None => {
                        None
                    }
                }
            }
            None => {
                None
            }
        }
    }

    fn is_left_child_of_parent(&self) -> bool {
        let parent = &self.parent;
        match parent {
            Some(raw_node) => {
                let left_node = raw_node.borrow().left_node();
                match left_node {
                    Some(raw_node) => {
                        self.val == raw_node.borrow().val
                    }
                    None => {
                        false
                    }
                }
            }
            None => {
                false
            }
        }
    }

    fn is_right_child_of_parent(&self) -> bool {
        let parent = &self.parent;
        match parent {
            Some(raw_node) => {
                let right_node = raw_node.borrow().right_node();
                match right_node {
                    Some(raw_node) => {
                        self.val == raw_node.borrow().val
                    }
                    None => {
                        false
                    }
                }
            }
            None => {
                false
            }
        }
    }

    fn is_left_none(&self) -> bool {
        self.left.is_none()
    }

    fn is_right_none(&self) -> bool {
        self.right.is_none()
    }

    fn left_node(&self) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        match &self.left {
            Some(left_node) => {
                Some(left_node.clone())
            }
            None => {None}
        }
    }

    fn right_node(&self) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        match &self.right {
            Some(right_node) => {
                Some(right_node.clone())
            }
            None => {None}
        }
        // let right_node = self.right.take();
        // if right_node.is_some() {
        //     let right_node = right_node.unwrap();
        //     self.right = Some(right_node.clone());
        //     Some(right_node)
        // } else {
        //     None
        // }
    }

    fn child_predecessor_node(&self) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        let mut node = self.left_node();
        let mut predecessor_node:Option<Rc<RefCell<BinaryRBTreeNode>>> = None;
        while let Some(raw_node) = node {
            predecessor_node = Some(raw_node.clone());
            node = raw_node.borrow().right_node();
        }
        predecessor_node
    }

    fn child_successor_node(&self) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        let mut node = self.right_node();
        let mut successor_node:Option<Rc<RefCell<BinaryRBTreeNode>>> = None;
        while let Some(raw_node) = node {
            successor_node = Some(raw_node.clone());
            node = raw_node.borrow().left_node();
        }
        successor_node
    }
    
    fn discolor(&mut self) {
        if let Color::Red = self.color {
            self.color = Color::Black
        } else {
            self.color = Color::Red
        }
    }

    fn InsertByDiscolorRRRotate(raw_parent_node:Rc<RefCell<BinaryRBTreeNode>>, grandparent_node:&Option<Rc<RefCell<BinaryRBTreeNode>>>) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        match grandparent_node {
            Some(raw_grandparent_node) => {
                // 1. 变色
                raw_parent_node.borrow_mut().discolor();
                raw_grandparent_node.borrow_mut().discolor();

                // 2. 父节点右旋
                let brother_node = raw_parent_node.borrow().right_node();
                raw_parent_node.borrow_mut().right = Some(raw_grandparent_node.clone());
                raw_grandparent_node.borrow_mut().left = brother_node; // brother_node == None 必然
                if BinaryRBTreeNode::IS_ROOT(grandparent_node) {
                    raw_grandparent_node.borrow_mut().parent = Some(raw_parent_node.clone());
                    raw_parent_node.borrow_mut().parent = None;
                    return Some(raw_parent_node.clone())
                } else {
                    let ancestor_node = raw_grandparent_node.borrow().parent_node();
                    match &ancestor_node {
                        Some(raw_ancestor_node) => {
                            if raw_grandparent_node.borrow().is_left_child_of_parent() {
                                raw_ancestor_node.borrow_mut().left = Some(raw_parent_node.clone());
                            } else {
                                raw_ancestor_node.borrow_mut().right = Some(raw_parent_node.clone());
                            }
                        }
                        None => {}
                    }
                    raw_grandparent_node.borrow_mut().parent = Some(raw_parent_node.clone());
                    raw_parent_node.borrow_mut().parent = ancestor_node;
                }
            }
            None => {}
        }
        None
    }

    fn InserByDiscolorLLRotate(raw_parent_node:Rc<RefCell<BinaryRBTreeNode>>, grandparent_node:&Option<Rc<RefCell<BinaryRBTreeNode>>>) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        match grandparent_node {
            Some(raw_grandparent_node) => {
                // 1. 变色
                raw_parent_node.borrow_mut().discolor();
                raw_grandparent_node.borrow_mut().discolor();

                // 2. 父节点左旋
                let brother_node = raw_parent_node.borrow().left_node();
                raw_parent_node.borrow_mut().left = Some(raw_grandparent_node.clone());
                raw_grandparent_node.borrow_mut().right = brother_node; // brother_node == None 必然
                if BinaryRBTreeNode::IS_ROOT(grandparent_node) {
                    raw_grandparent_node.borrow_mut().parent = Some(raw_parent_node.clone());
                    raw_parent_node.borrow_mut().parent = None;
                    return Some(raw_parent_node.clone())
                } else {
                    let ancestor_node = raw_grandparent_node.borrow().parent_node();
                    match &ancestor_node {
                        Some(raw_ancestor_node) => {
                            if raw_grandparent_node.borrow().is_left_child_of_parent() {
                                raw_ancestor_node.borrow_mut().left = Some(raw_parent_node.clone());
                            } else {
                                raw_ancestor_node.borrow_mut().right = Some(raw_parent_node.clone());
                            }
                        }
                        None => {}
                    }
                    raw_grandparent_node.borrow_mut().parent = Some(raw_parent_node.clone());
                    raw_parent_node.borrow_mut().parent = ancestor_node;
                }
            }
            None => {}
        }
        None
    }

    fn BalanceByDiscolorRRRotate(raw_parent_node:Rc<RefCell<BinaryRBTreeNode>>, raw_brother_node:Rc<RefCell<BinaryRBTreeNode>>, raw_nephew_node:Rc<RefCell<BinaryRBTreeNode>>,root_node:Option<Rc<RefCell<BinaryRBTreeNode>>>) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        let mut root_node = root_node;
        let color = raw_parent_node.borrow().color;
        raw_parent_node.borrow_mut().color = raw_brother_node.borrow().color;
        raw_brother_node.borrow_mut().color = color;

        // raw_node.borrow_mut().parent = None;
        // raw_parent_node.borrow_mut().right = None; // 删除raw_node
        raw_parent_node.borrow_mut().left = raw_brother_node.borrow().right_node();
        raw_brother_node.borrow_mut().right = Some(raw_parent_node.clone());

        let grandparent_node = raw_parent_node.borrow().parent_node();
        match &grandparent_node {
            Some(raw_grandparent_node) => {
                if raw_parent_node.borrow().is_left_child_of_parent() {
                    raw_grandparent_node.borrow_mut().left = Some(raw_brother_node.clone());
                } else {
                    raw_grandparent_node.borrow_mut().right = Some(raw_brother_node.clone());
                }
            }
            None => {
                root_node = Some(raw_brother_node.clone());
            }
        }
        raw_parent_node.borrow_mut().parent = Some(raw_brother_node.clone());
        raw_brother_node.borrow_mut().parent = grandparent_node;
        raw_nephew_node.borrow_mut().discolor();
        root_node
    }

    fn BalanceByDiscolorRLRotate(raw_parent_node:Rc<RefCell<BinaryRBTreeNode>>, raw_brother_node:Rc<RefCell<BinaryRBTreeNode>>, raw_nephew_node:Rc<RefCell<BinaryRBTreeNode>>,root_node:Option<Rc<RefCell<BinaryRBTreeNode>>>) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        let mut root_node = root_node;
        BinaryRBTreeNode::SWAP(raw_nephew_node.clone(), raw_brother_node.clone());
        let node = raw_brother_node.borrow().right_node();
        raw_brother_node.borrow_mut().right = Some(raw_nephew_node.clone());
        raw_brother_node.borrow_mut().left = raw_nephew_node.borrow().left_node();
        raw_nephew_node.borrow_mut().left = raw_nephew_node.borrow().right_node();
        raw_nephew_node.borrow_mut().right = node;
        root_node = BinaryRBTreeNode::BalanceByDiscolorLLRotate(raw_parent_node, raw_brother_node, raw_nephew_node.clone(), root_node);
        root_node
    }

    fn BalanceByDiscolorLLRotate(raw_parent_node:Rc<RefCell<BinaryRBTreeNode>>, raw_brother_node:Rc<RefCell<BinaryRBTreeNode>>,raw_nephew_node:Rc<RefCell<BinaryRBTreeNode>>,root_node:Option<Rc<RefCell<BinaryRBTreeNode>>>) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        let mut root_node = root_node;
        let color = raw_parent_node.borrow().color;
        raw_parent_node.borrow_mut().color = raw_brother_node.borrow().color;
        raw_brother_node.borrow_mut().color = color;

        // raw_node.borrow_mut().parent = None;
        // raw_parent_node.borrow_mut().left = None; // 删除raw_node
        raw_parent_node.borrow_mut().right = raw_brother_node.borrow().left_node();
        raw_brother_node.borrow_mut().left = Some(raw_parent_node.clone());

        let grandparent_node = raw_parent_node.borrow().parent_node();
        match &grandparent_node {
            Some(raw_grandparent_node) => {
                if raw_parent_node.borrow().is_left_child_of_parent() {
                    raw_grandparent_node.borrow_mut().left = Some(raw_brother_node.clone());
                } else {
                    raw_grandparent_node.borrow_mut().right = Some(raw_brother_node.clone());
                }
            }
            None => {
                root_node = Some(raw_brother_node.clone());
            }
        }
        raw_parent_node.borrow_mut().parent = Some(raw_brother_node.clone());
        raw_brother_node.borrow_mut().parent = grandparent_node;
        raw_nephew_node.borrow_mut().discolor();
        root_node
    }

    fn BalanceByDiscolorLRRotate(raw_parent_node:Rc<RefCell<BinaryRBTreeNode>>, raw_brother_node:Rc<RefCell<BinaryRBTreeNode>>, raw_nephew_node:Rc<RefCell<BinaryRBTreeNode>>,root_node:Option<Rc<RefCell<BinaryRBTreeNode>>>) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        let mut root_node = root_node;
        BinaryRBTreeNode::SWAP(raw_nephew_node.clone(), raw_brother_node.clone());
        let node = raw_brother_node.borrow().left_node();
        raw_brother_node.borrow_mut().left = Some(raw_nephew_node.clone());
        raw_brother_node.borrow_mut().right = raw_nephew_node.borrow().right_node();
        raw_nephew_node.borrow_mut().right = raw_nephew_node.borrow().left_node();
        raw_nephew_node.borrow_mut().left = node;
        root_node = BinaryRBTreeNode::BalanceByDiscolorRRRotate(raw_parent_node, raw_brother_node, raw_nephew_node.clone(), root_node);
        root_node
    }

    fn FIND_PARENT_NODE_AND_MAYINSERT(node: &Option<Rc<RefCell<BinaryRBTreeNode>>>, val: u32) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        match node {
            Some(raw_node) => {
                if raw_node.borrow().val > val {
                    let left_node = raw_node.borrow().left_node();
                    if left_node.is_none() {
                        let node = BinaryRBTreeNode::new(val, Some(raw_node.clone()));
                        raw_node.borrow_mut().left = Some(node.clone());
                        return Some(node.clone())
                    }
                    BinaryRBTreeNode::FIND_PARENT_NODE_AND_MAYINSERT(&left_node, val)
                } else if raw_node.borrow().val < val  {
                    let right_node = raw_node.borrow().right_node();
                    if right_node.is_none() {
                        let node = BinaryRBTreeNode::new(val, Some(raw_node.clone()));
                        raw_node.borrow_mut().right = Some(node.clone());
                        return Some(node.clone())
                    }
                    BinaryRBTreeNode::FIND_PARENT_NODE_AND_MAYINSERT(&right_node, val)
                } else {
                    Some(raw_node.clone())
                }
            }
            None => {
                let node = BinaryRBTreeNode::new(val, None);
                Some(node)
            }
        }
    }

    fn INSERT(val: u32, root:&Option<Rc<RefCell<BinaryRBTreeNode>>>) -> Option<Rc<RefCell<BinaryRBTreeNode>>> { // 返回值是为了root节点的可能变更
        // 插入到叶子节点
        let node = BinaryRBTreeNode::FIND_PARENT_NODE_AND_MAYINSERT(root, val);
        if BinaryRBTreeNode::IS_ROOT(&node) {
            return node
        }
        match node {
            Some(raw_node) => {
                let parent_node = raw_node.borrow().parent_node();
                if BinaryRBTreeNode::IS_ROOT(&parent_node) {
                    return parent_node
                }
                if BinaryRBTreeNode::IS_BLACK(&parent_node) {
                    return None
                }
                match parent_node {
                    Some(raw_parent_node) => {
                        let uncle_node = raw_parent_node.borrow().brother_node();
                        let grandparent_node = raw_parent_node.borrow().parent_node();
                        // 1. LL型，node相对于parent_node不会有brother_node
                        //    a) 变色（父节点红->黑，祖节点黑->红）
                        //    b) 祖节点围绕父节点右旋，逆转父子关系
                        if raw_parent_node.borrow().is_left_child_of_parent() && raw_node.borrow().is_left_child_of_parent() && BinaryRBTreeNode::IS_BLACK(&uncle_node) {
                            //    a) 变色（父节点红->黑，祖节点黑->红）
                            //    b) 祖节点围绕父节点右旋，逆转父子关系
                            return BinaryRBTreeNode::InsertByDiscolorRRRotate(raw_parent_node, &grandparent_node)
                        }

                        // 2. LR型，node相对于parent_node不会有brother_node
                        //    a) 父节点围绕子节点左旋，逆转父子关系，注意祖父节点和新父节点实际状态断开，保持了语义状态
                        //    b) 变色（父节点红->黑，祖节点黑->红）
                        //    c) 祖节点围绕父节点右旋，逆转父子关系
                        if raw_parent_node.borrow().is_left_child_of_parent() && raw_node.borrow().is_right_child_of_parent() && BinaryRBTreeNode::IS_BLACK(&uncle_node) {
                            //    a) 父节点围绕子节点左旋，逆转父子关系，注意祖父节点和新父节点实际状态断开，保持了语义状态
                            raw_node.borrow_mut().parent = raw_parent_node.borrow().parent_node();
                            raw_node.borrow_mut().left = Some(raw_parent_node.clone());
                            raw_parent_node.borrow_mut().parent = Some(raw_node.clone());
                            raw_parent_node.borrow_mut().right = None;
                            //    b) 变色（父节点红->黑，祖节点黑->红）
                            //    c) 祖节点围绕父节点右旋，逆转父子关系
                            return BinaryRBTreeNode::InsertByDiscolorRRRotate(raw_node, &grandparent_node)
                        }

                        // 3. RR型，node相对于parent_node不会有brother_node
                        //    a) 变色（父节点红->黑，祖节点黑->红）
                        //    b) 祖节点围绕父节点左旋，逆转父子关系
                        if raw_parent_node.borrow().is_right_child_of_parent() && raw_node.borrow().is_right_child_of_parent() && BinaryRBTreeNode::IS_BLACK(&uncle_node) {
                            //    a) 变色（父节点红->黑，祖节点黑->红）
                            //    b) 祖节点围绕父节点左旋，逆转父子关系
                            return BinaryRBTreeNode::InserByDiscolorLLRotate(raw_parent_node, &grandparent_node)
                        }

                        // 4. RL型，node相对于parent_node不会有brother_node
                        //    a) 父节点围绕子节点右旋，逆转父子关系，注意祖父节点和新父节点实际状态断开，保持了语义状态
                        //    b) 变色（父节点红->黑，祖节点黑->红）
                        //    c) 祖节点围绕父节点左旋，逆转父子关系
                        if raw_parent_node.borrow().is_right_child_of_parent() && raw_node.borrow().is_left_child_of_parent() && BinaryRBTreeNode::IS_BLACK(&uncle_node) {
                            //    a) 父节点围绕子节点右旋，逆转父子关系，注意祖父节点和新父节点实际状态断开，保持了语义状态
                            raw_node.borrow_mut().parent = raw_parent_node.borrow().parent_node();
                            raw_node.borrow_mut().right = Some(raw_parent_node.clone());
                            raw_parent_node.borrow_mut().parent = Some(raw_node.clone());
                            raw_parent_node.borrow_mut().left = None;
                            //    b) 变色（父节点红->黑，祖节点黑->红）
                            //    c) 祖节点围绕父节点左旋，逆转父子关系
                            return BinaryRBTreeNode::InserByDiscolorLLRotate(raw_node, &grandparent_node)
                        }
                    }
                    None => {    
                    }
                }
                None
            }
            None => {
                None
            }
        }
    }

    fn SWAP(raw_from_node:Rc<RefCell<BinaryRBTreeNode>>, raw_to_node:Rc<RefCell<BinaryRBTreeNode>>) {
        let val = raw_from_node.borrow().val;
        raw_from_node.borrow_mut().val = raw_to_node.borrow().val;
        raw_to_node.borrow_mut().val = val;
    }

    fn ReBalance(parent_node:Option<Rc<RefCell<BinaryRBTreeNode>>>, brother_node:Option<Rc<RefCell<BinaryRBTreeNode>>>, root_node:Option<Rc<RefCell<BinaryRBTreeNode>>>) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        let mut root_node = root_node;
        match parent_node {
            Some(raw_parent_node) => {
                if BinaryRBTreeNode::IS_BLACK(&brother_node) {
                    match brother_node {
                        Some(raw_brother_node) => {
                            let left_nephew_node = raw_brother_node.borrow().left_node();
                            let right_nephew_node = raw_brother_node.borrow().right_node();
                            if BinaryRBTreeNode::IS_RED(&left_nephew_node) && raw_brother_node.borrow().is_left_child_of_parent() {
                               match &left_nephew_node {
                                   Some(raw_nephew_node)=>{
                                        // raw_node.borrow_mut().parent = None;
                                        // raw_parent_node.borrow_mut().right = None; // 删除raw_node
                                        root_node = BinaryRBTreeNode::BalanceByDiscolorRRRotate(raw_parent_node, raw_brother_node, raw_nephew_node.clone(), root_node);
                                   }
                                   None => {}
                               }
                            } else if BinaryRBTreeNode::IS_RED(&right_nephew_node) && raw_brother_node.borrow().is_right_child_of_parent() {
                                match &right_nephew_node {
                                    Some(raw_nephew_node)=>{
                                        // raw_node.borrow_mut().parent = None;
                                        // raw_parent_node.borrow_mut().left = None; // 删除raw_node
                                        root_node = BinaryRBTreeNode::BalanceByDiscolorLLRotate(raw_parent_node, raw_brother_node, raw_nephew_node.clone(), root_node);
                                    }
                                    None => {}
                                }
                            } else if BinaryRBTreeNode::IS_RED(&left_nephew_node) && raw_brother_node.borrow().is_right_child_of_parent() {
                                match &left_nephew_node {
                                    Some(raw_nephew_node)=>{
                                        // raw_node.borrow_mut().parent = None;
                                        // raw_parent_node.borrow_mut().left = None; // 删除raw_node
                                        root_node = BinaryRBTreeNode::BalanceByDiscolorRLRotate(raw_parent_node, raw_brother_node, raw_nephew_node.clone(), root_node);
                                    }
                                    None => {}
                                }
                            } else if BinaryRBTreeNode::IS_RED(&right_nephew_node) && raw_brother_node.borrow().is_left_child_of_parent() {
                                match &right_nephew_node {
                                    Some(raw_nephew_node)=>{
                                        // raw_node.borrow_mut().parent = None;
                                        // raw_parent_node.borrow_mut().right = None; // 删除raw_node
                                        root_node = BinaryRBTreeNode::BalanceByDiscolorLRRotate(raw_parent_node, raw_brother_node, raw_nephew_node.clone(), root_node);
                                    }
                                    None => {}
                                }
                            } else {
                                if BinaryRBTreeNode::IS_RED(&Some(raw_parent_node.clone())) {
                                    raw_brother_node.borrow_mut().discolor();
                                    raw_parent_node.borrow_mut().discolor();
                                } else {
                                    let parent_node = raw_parent_node.borrow().parent_node();
                                    let brother_node = raw_parent_node.borrow().brother_node();
                                    root_node = BinaryRBTreeNode::ReBalance(parent_node, brother_node, root_node);
                                }
                            }
                        }
                        None => {}
                    }
                } else {
                    match brother_node {
                        Some(raw_brother_node) => {
                            if raw_brother_node.borrow().is_right_child_of_parent() {
                                let nephew_node = raw_brother_node.borrow().left_node();
                                raw_parent_node.borrow_mut().right = raw_brother_node.borrow().left_node();
                                raw_brother_node.borrow_mut().left = Some(raw_parent_node.clone());
                                raw_brother_node.borrow_mut().parent = raw_parent_node.borrow().parent_node();
                                raw_parent_node.borrow_mut().parent = Some(raw_brother_node.clone());
                                root_node = BinaryRBTreeNode::ReBalance(Some(raw_brother_node.clone()), nephew_node, root_node);
                            } else {
                                let nephew_node = raw_brother_node.borrow().right_node();
                                raw_parent_node.borrow_mut().left = raw_brother_node.borrow().right_node();
                                raw_brother_node.borrow_mut().right = Some(raw_parent_node.clone());
                                raw_brother_node.borrow_mut().parent = raw_parent_node.borrow().parent_node();
                                raw_parent_node.borrow_mut().parent = Some(raw_brother_node.clone());
                                root_node = BinaryRBTreeNode::ReBalance(Some(raw_brother_node.clone()), nephew_node, root_node);
                            }
                        }
                        None => {} // impossible
                    }
                }
            }
            None => {}
        }
        root_node
    }

    fn DELETE_NOCHILD_NODE(raw_node:Rc<RefCell<BinaryRBTreeNode>>, root_node:Option<Rc<RefCell<BinaryRBTreeNode>>>) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        if BinaryRBTreeNode::IS_ROOT(&Some(raw_node.clone())) {
            return None
        }
        let mut root_node = root_node;
        let parent_node = raw_node.borrow().parent_node();
        let brother_node = raw_node.borrow().brother_node();
        
        match &parent_node {
            Some(raw_parent_node) => {
                if raw_node.borrow_mut().is_left_child_of_parent() {
                    raw_parent_node.borrow_mut().left = None;
                } else {
                    raw_parent_node.borrow_mut().right = None;
                }
                raw_node.borrow_mut().parent = None;
            }
            None => {}
        }

        if BinaryRBTreeNode::IS_RED(&Some(raw_node.clone())) { // 1
            // match parent_node {
            //     Some(raw_parent_node) => {
            //         if raw_node.borrow_mut().is_left_child_of_parent() {
            //             raw_parent_node.borrow_mut().left = None;
            //         } else {
            //             raw_parent_node.borrow_mut().right = None;
            //         }
            //         raw_node.borrow_mut().parent = None;
            //     }
            //     None => {}
            // }
        } else { // 2
            root_node = BinaryRBTreeNode::ReBalance(parent_node, brother_node, root_node);
        }
        root_node
    }

    fn DELETE_ONECHILD_NODE(raw_node:Rc<RefCell<BinaryRBTreeNode>>, root_node:Option<Rc<RefCell<BinaryRBTreeNode>>>) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        let mut root_node = root_node;
        if BinaryRBTreeNode::IS_RED(&Some(raw_node.clone())) { // 3
            // impossible
        } else { // 4
            let parent_node = raw_node.borrow().parent_node();
            let mut child_node = raw_node.borrow().left_node();
            if child_node.is_none() {
                child_node = raw_node.borrow().right_node();
            }
            match parent_node {
                Some(raw_parent_node) => {
                    match child_node {
                        Some(raw_child_node) => {
                            raw_child_node.borrow_mut().discolor();
                            if raw_node.borrow_mut().is_left_child_of_parent() {
                                raw_parent_node.borrow_mut().left = Some(raw_child_node.clone());
                            } else {
                                raw_parent_node.borrow_mut().right = Some(raw_child_node.clone());
                            }
                            raw_child_node.borrow_mut().parent = Some(raw_parent_node.clone());
                        }
                        None => {}
                    }
                }
                None => {
                    match child_node {
                        Some(raw_child_node) => {
                            raw_child_node.borrow_mut().discolor();
                            raw_child_node.borrow_mut().parent = None;
                            root_node = Some(raw_child_node.clone());
                        }
                        None => {}
                    }
                }
            }
        }
        root_node
    }

    fn DELETE(val: u32, root:&Option<Rc<RefCell<BinaryRBTreeNode>>>) -> Option<Rc<RefCell<BinaryRBTreeNode>>> { // 返回值是为了root节点的可能变更
        if root.is_none() {
            return None
        }
        let mut node = BinaryRBTreeNode::FIND_NODE(root, val);
        let mut root_node:Option<Rc<RefCell<BinaryRBTreeNode>>> = None;
        match root {
            Some(raw_node) => {
                root_node = Some(raw_node.clone())
            }
            None => {}
        }
        match &node {
            Some(raw_node) => {
                if raw_node.borrow().is_left_none() && raw_node.borrow().is_right_none() { // 1, 2
                    root_node = BinaryRBTreeNode::DELETE_NOCHILD_NODE(raw_node.clone(), root_node);
                } else if raw_node.borrow().is_left_none() || raw_node.borrow().is_right_none() { // 3, 4
                    root_node = BinaryRBTreeNode::DELETE_ONECHILD_NODE(raw_node.clone(), root_node);
                } else { // 5, 6
                    let successor_node = raw_node.borrow().child_successor_node();
                    match successor_node {
                        Some(raw_successor_node) => {
                            BinaryRBTreeNode::SWAP(raw_node.clone(), raw_successor_node.clone());
                            if raw_successor_node.borrow().is_left_none() || raw_successor_node.borrow().is_right_none() { // 3, 4
                                root_node = BinaryRBTreeNode::DELETE_ONECHILD_NODE(raw_successor_node.clone(), root_node);
                            } else { // 1, 2
                                root_node = BinaryRBTreeNode::DELETE_NOCHILD_NODE(raw_successor_node.clone(), root_node);
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }
        root_node
    }

    fn UPDATE(root:&Option<Rc<RefCell<BinaryRBTreeNode>>>, old_val: u32, new_val: u32) -> bool {
        if root.is_none() {
            return false
        }
        let node = BinaryRBTreeNode::FIND_NODE(root, old_val);
        match node {
            Some(raw_node) => {
                raw_node.borrow_mut().val = new_val;
                true
            }
            None => {
                false
            }
        }
    }

    fn FIND_NODE(node: &Option<Rc<RefCell<BinaryRBTreeNode>>>, val: u32) -> Option<Rc<RefCell<BinaryRBTreeNode>>> {
        match node {
            Some(raw_node) => {
                if raw_node.borrow().val > val {
                    let left_node = raw_node.borrow().left_node();
                    BinaryRBTreeNode::FIND_NODE(&left_node, val)
                } else if raw_node.borrow().val < val  {
                    let right_node = raw_node.borrow().right_node();
                    BinaryRBTreeNode::FIND_NODE(&right_node, val)
                } else {
                    Some(raw_node.clone())
                }
            }
            None => {
                None
            }
        }
    }
}
// fn insert(root:Rc<RefCell<Multi23TreeNode>>, val: u32) -> Rc<RefCell<Multi23TreeNode>> {
//     Multi23TreeNode::new2node(val)
// }

// fn valid_bst(root: &Option<Rc<RefCell<TreeNode>>>, min: &Option<Rc<RefCell<TreeNode>>>, max: &Option<Rc<RefCell<TreeNode>>>) -> bool {
//     match root {
//         Some(raw_root_node) => {
//             let mut valid = true;
//             valid = valid && valid_bst(&raw_root_node.borrow().left, &None, &Some(raw_root_node.clone()));
//             if valid {
//                 match min {
//                     Some(raw_min_node) => {
//                         valid = valid && raw_min_node.borrow().mlval <= raw_root_node.borrow().mlval;
//                     }
//                     None => {}
//                 }
//             }
//             if valid {
//                 match max {
//                     Some(raw_max_node) => {
//                         valid = valid && raw_max_node.borrow().mlval >= raw_root_node.borrow().mlval;
//                     }
//                     None => {}
//                 }
//             }
//             if valid {
//                 valid = valid && valid_bst(&raw_root_node.borrow().right, &Some(raw_root_node.clone()), &None);
//             }
//             valid
//         }
//         None => {
//             true
//         }
//     }
// }

// fn traverse(node: &Option<Rc<RefCell<TreeNode>>>) {
//     match node {
//         Some(raw_node) => {
//             traverse(&raw_node.borrow().left);
//             let mlval = raw_node.borrow().mlval;
//             let sum = raw_node.borrow().sum;
//             println!("{} {}", sum, mlval);
//             traverse(&raw_node.borrow().right);
//         }
//         None => {
//         }
//     }
// }

// fn init_valid_bst_tree() -> Option<Rc<RefCell<TreeNode>>> {
//     let root = TreeNode::new(4);
    
//     {
//         let left_node = TreeNode::new(2);
//         left_node.borrow_mut().left = None;
//         left_node.borrow_mut().right = None;

//         let right_node = TreeNode::new(6);
//         right_node.borrow_mut().left = None;
//         right_node.borrow_mut().right = None;

//         let mut node = root.borrow_mut();
//         node.left = Some(left_node);
//         node.right = Some(right_node);
//     }

//     {
//         let left_node = TreeNode::new(1);
//         left_node.borrow_mut().left = None;
//         left_node.borrow_mut().right = None;

//         let right_node = TreeNode::new(3);
//         right_node.borrow_mut().left = None;
//         right_node.borrow_mut().right = None;

//         let node = root.borrow();
//         let mut node = node.left.as_ref().unwrap().borrow_mut();
//         node.left = Some(left_node);
//         node.right = Some(right_node);
//     }

//     {
//         let left_node = TreeNode::new(5);
//         left_node.borrow_mut().left = None;
//         left_node.borrow_mut().right = None;

//         let right_node = TreeNode::new(7);
//         right_node.borrow_mut().left = None;
//         right_node.borrow_mut().right = None;

//         let node = root.borrow();
//         let mut node = node.right.as_ref().unwrap().borrow_mut();
//         node.left = Some(left_node);
//         node.right = Some(right_node);
//     }

//     return Some(root)
// }

// fn init_invalid_bst_tree() -> Option<Rc<RefCell<TreeNode>>> {
//     let root = TreeNode::new(4);
    
//     {
//         let left_node = TreeNode::new(2);
//         left_node.borrow_mut().left = None;
//         left_node.borrow_mut().right = None;

//         let right_node = TreeNode::new(6);
//         right_node.borrow_mut().left = None;
//         right_node.borrow_mut().right = None;

//         let mut node = root.borrow_mut();
//         node.left = Some(left_node);
//         node.right = Some(right_node);
//     }

//     {
//         let left_node = TreeNode::new(1);
//         left_node.borrow_mut().left = None;
//         left_node.borrow_mut().right = None;

//         let right_node = TreeNode::new(3);
//         right_node.borrow_mut().left = None;
//         right_node.borrow_mut().right = None;

//         let node = root.borrow();
//         let mut node = node.left.as_ref().unwrap().borrow_mut();
//         node.left = Some(left_node);
//         node.right = Some(right_node);
//     }

//     {
//         let left_node = TreeNode::new(7);
//         left_node.borrow_mut().left = None;
//         left_node.borrow_mut().right = None;

//         let right_node = TreeNode::new(5);
//         right_node.borrow_mut().left = None;
//         right_node.borrow_mut().right = None;

//         let node = root.borrow();
//         let mut node = node.right.as_ref().unwrap().borrow_mut();
//         node.left = Some(left_node);
//         node.right = Some(right_node);
//     }

//     return Some(root)
// }

// fn test_bst(mark: &str, tree: Option<Rc<RefCell<TreeNode>>>){
//     println!("=====start {} tree", mark);
//     println!("{}", valid_bst(&tree, &None, &None));
//     println!("=====end {} tree", mark);
// }

fn main() {
    // test_bst("valid bst", init_valid_bst_tree());
    // test_bst("invalid bst", init_invalid_bst_tree());
}
