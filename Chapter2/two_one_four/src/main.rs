use std::ops::{Add};

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i+j
}

fn main() {
    let d = 10;
    let e = 20;
    let res = add_with_lifetimes(&d, &e);
    
    println!("{} + {} = {}", d, e, res);

    let (a, b) = (1.2, 3.4);
    let (x, y) = (10, 20);

    let c = add(a,b);
    let z = add(x,y);

    println!("{} + {} = {}", a, b, c);
    println!("{} + {} = {}", x, y, z);
}


