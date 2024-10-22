// 接口
trait Calculator {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
    fn mul(&self) -> i32;
}

// 胖指针，相似于golang interface
#[repr(C)]
struct FatPointer<'a> {
    // 数据段
    data: &'a mut Data,
    // 接口段
    vtable: *const usize,
}

// 数据对象
struct Data {
    a: i32,
    b: i32,
}

// 数据对象的方法，相当于实现了golang interface，但是rust需要显示指定 implement
fn add(s: &Data) -> i32 {
    s.a + s.b
}
fn sub(s: &Data) -> i32 {
    s.a - s.b
}
fn mul(s: &Data) -> i32 {
    s.a * s.b
}

fn main() {
    let mut data = Data {a: 3, b: 2};
    // 数据对象的接口视图部分的内存布局，byte数组，必须和rust的标准一致
    let vtable = vec![
        0,            // rust标准，pointer to `Drop`
        6,            // rust标准，lenght of vtable
        8,            // rust标准，alignment

        // 方法内存布局的排列，必须和 Trait Calculator的定义顺序一致
        add as usize, // 数据对象的add对应trait Calculator的add
        sub as usize, // 数据对象的sub对应trait Calculator的sub
        mul as usize, // 数据对象的mul对应trait Calculator的mul
    ];

    // 定义一个胖指针，包括数据对象和匹配Trait Calculator的方法组
    let fat_pointer = FatPointer { data: &mut data, vtable: vtable.as_ptr()};
    // 将数据对象转换成接口对象
    let calculator = unsafe { std::mem::transmute::<FatPointer, &dyn Calculator>(fat_pointer) };

    // 通过接口视图限制，调用方法
    println!("Add: 3 + 2 = {}", calculator.add());
    println!("Sub: 3 - 2 = {}", calculator.sub());
    println!("Mul: 3 * 2 = {}", calculator.mul());
}
