use std::mem;

fn main() {
    println!("==============take==============");
    let mut hello = vec![1, 2, 3];
    println!("hello ({:?}) address: {:p}, internal buffer address: {:p}", hello, &hello, &*hello);
    let old_hello = mem::take(&mut hello);
    println!("hello ({:?}) address: {:p}, internal buffer address: {:p}; old_hello ({:?}) address: {:p}, internal buffer address: {:p}", hello, &hello, &*hello, old_hello, &old_hello, &*old_hello);

    println!("==============replace==============");
    let mut hello = vec![1, 2, 3];
    println!("hello ({:?}) address: {:p}, internal buffer address: {:p}", hello, &hello, &*hello);
    let old_hello = mem::replace(&mut hello, vec![4, 5, 6]);
    println!("hello ({:?}) address: {:p}, internal buffer address: {:p}; old_hello ({:?}) address: {:p}, internal buffer address: {:p}", hello, &hello, &*hello, old_hello, &old_hello, &*old_hello);

    println!("==============swap==============");
    let mut hello1 = vec![1, 2, 3];
    let mut hello2 = vec![4, 5, 6];
    println!("hello1 ({:?}) address: {:p}, internal buffer address: {:p}; hello2 ({:?}) address: {:p}, internal buffer address: {:p}", hello1, &hello1, &*hello1, hello2, &hello2, &*hello2);
    mem::swap(&mut hello1, &mut hello2);
    println!("hello1 ({:?}) address: {:p}, internal buffer address: {:p}; hello2 ({:?}) address: {:p}, internal buffer address: {:p}", hello1, &hello1, &*hello1, hello2, &hello2, &*hello2);
}
