// Contains both parts of 6.6 and 6.7
fn main(){
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr)};

    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);
}