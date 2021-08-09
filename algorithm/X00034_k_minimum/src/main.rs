use std::rc::Rc;
use std::mem::swap;
use std::mem::size_of;
use std::cmp::max;
use std::cell::RefCell;
use std::cell::Cell;
use std::collections::HashMap;

struct TreeNode {
    value: u32,
    rank: u32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: u32, rank: u32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            rank: rank,
            left: None,
            right: None,
        }))
    }

    fn left_node(&mut self) -> Option<Rc<RefCell<TreeNode>>> {
        let left_node = self.left.take();
        if left_node.is_some() {
            let left_node = left_node.unwrap();
            self.left = Some(left_node.clone());
            Some(left_node)
        } else {
            None
        }
    }

    fn right_node(&mut self) -> Option<Rc<RefCell<TreeNode>>> {
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

fn k_minimum(node: &Option<Rc<RefCell<TreeNode>>>, rank: u32, k: u32) -> u32 {
    match node {
        Some(raw_node) => {
            let mut rank = rank;
            if rank < k {
                rank = k_minimum(&raw_node.borrow().left, rank, k);
            }

            rank = rank + 1;
            if rank == k {
                let value = raw_node.borrow().value;
                println!("{} {}", rank, value);
            } else {
                let value = raw_node.borrow().value;
                println!("pass {} {}", rank, value);
            }

            if rank < k {
                rank = k_minimum(&raw_node.borrow().right, rank, k);
            }
            rank
        }
        None => {
            rank
        }
    }
}

fn fast_k_minimum(node: &Option<Rc<RefCell<TreeNode>>>, rank: u32) {
    match node {
        Some(raw_node) => {
            if raw_node.borrow().rank < rank - 1 {
                let value = raw_node.borrow().value;
                println!("pass {} {} {}", raw_node.borrow().rank, rank - 1, value);
                fast_k_minimum(&raw_node.borrow().left, rank);
            } else if raw_node.borrow().rank > rank - 1 {
                let value = raw_node.borrow().value;
                println!("pass {} {} {}", raw_node.borrow().rank, rank - 1, value);
                fast_k_minimum(&raw_node.borrow().right, rank);
            } else {
                let value = raw_node.borrow().value;
                println!("{} {}", rank, value);
            }
        }
        None => {
        }
    }
}

fn init_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let root = TreeNode::new(4, 3);
    
    {
        let left_node = TreeNode::new(2, 1);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(6, 5);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let mut node = root.borrow_mut();
        node.left = Some(left_node);
        node.right = Some(right_node);
    }

    {
        let left_node = TreeNode::new(1, 0);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(3, 2);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let node = root.borrow();
        let mut node = node.left.as_ref().unwrap().borrow_mut();
        node.left = Some(left_node);
        node.right = Some(right_node);
    }

    {
        let left_node = TreeNode::new(5, 4);
        left_node.borrow_mut().left = None;
        left_node.borrow_mut().right = None;

        let right_node = TreeNode::new(7, 6);
        right_node.borrow_mut().left = None;
        right_node.borrow_mut().right = None;

        let node = root.borrow();
        let mut node = node.right.as_ref().unwrap().borrow_mut();
        node.left = Some(left_node);
        node.right = Some(right_node);
    }

    return Some(root)
}

fn test_k_minimum(mark: &str, tree: Option<Rc<RefCell<TreeNode>>>, k: u32){
    println!("=====start {} tree", mark);
    k_minimum(&tree, 0, k);
    println!("=====end {} tree", mark);
}

fn test_fast_k_minimum(mark: &str, tree: Option<Rc<RefCell<TreeNode>>>, k: u32){
    println!("=====start {} tree", mark);
    fast_k_minimum(&tree, k);
    println!("=====end {} tree", mark);
}

trait SomeTrait {}

// A reference to a trait object is a fat pointer: (data_ptr, vtable_ptr)
trait Test {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
    fn mul(&self) -> i32;
}

// This will represent our home brewn fat pointer to a trait object
#[repr(C)]
struct FatPointer<'a> {
    /// A reference is a pointer to an instantiated `Data` instance
    data: &'a mut Data,
    /// Since we need to pass in literal values like length and alignment it's
    /// easiest for us to convert pointers to usize-integers instead of the other way around.
    vtable: *const usize,
}

// This is the data in our trait object. It's just two numbers we want to operate on.
struct Data {
    a: i32,
    b: i32,
}

// ====== function definitions ======
fn add(s: &Data) -> i32 {
    s.a + s.b
}
fn sub(s: &Data) -> i32 {
    s.a - s.b
}
fn mul(s: &Data) -> i32 {
    s.a * s.b
}

fn test(my_num:Box<i32>) {
    unsafe {
        //将安全的Box<int>类型转换成不安全的指针类型
        let my_num2: *const i32 = std::mem::transmute(my_num);
        println!("num = {}", *my_num2);
    }   
}    

fn main() {
    test_k_minimum("k minimum", init_tree(), 1);

    test_k_minimum("k minimum", init_tree(), 3);

    test_k_minimum("k minimum", init_tree(), 7);

    test_fast_k_minimum("fast k minimum", init_tree(), 1);

    test_fast_k_minimum("fast k minimum", init_tree(), 3);

    test_fast_k_minimum("fast k minimum", init_tree(), 7);

    let my_num: Box<i32> = Box::new(10);
    test(my_num);
    
    println!("======== The size of different pointers in Rust: ========");
    println!("&dyn Trait:-----{}", size_of::<&dyn SomeTrait>());
    println!("&[&dyn Trait]:--{}", size_of::<&[&dyn SomeTrait]>());
    println!("Box<Trait>:-----{}", size_of::<Box<SomeTrait>>());
    println!("&i32:-----------{}", size_of::<&i32>());
    println!("&[i32]:---------{}", size_of::<&[i32]>());
    println!("Box<i32>:-------{}", size_of::<Box<i32>>());
    println!("&Box<i32>:------{}", size_of::<&Box<i32>>());
    println!("[&dyn Trait;4]:-{}", size_of::<[&dyn SomeTrait; 4]>());
    println!("[i32;4]:--------{}", size_of::<[i32; 4]>());

    let mut data = Data {a: 3, b: 2};
    // vtable is like special purpose array of pointer-length types with a fixed
    // format where the three first values has a special meaning like the
    // length of the array is encoded in the array itself as the second value.
    let vtable = vec![
        0,            // pointer to `Drop` (which we're not implementing here)
        6,            // lenght of vtable
        8,            // alignment

        // we need to make sure we add these in the same order as defined in the Trait.
        add as usize, // function pointer - try changing the order of `add`
        sub as usize, // function pointer - and `sub` to see what happens
        mul as usize, // function pointer
    ];

    let fat_pointer = FatPointer { data: &mut data, vtable: vtable.as_ptr()};
    let test = unsafe { std::mem::transmute::<FatPointer, &dyn Test>(fat_pointer) };

    // And voalá, it's now a trait object we can call methods on
    println!("Add: 3 + 2 = {}", test.add());
    println!("Sub: 3 - 2 = {}", test.sub());
    println!("Mul: 3 * 2 = {}", test.mul());

}
