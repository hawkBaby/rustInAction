fn main() {
    // More Notation
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a,b), add(c,d));
    println!("(a + b) + (c + d) = {}", e);

    // Notation
    let twenty = 20;
    let twenty_one: i32 = twenty + 1;
    let floats_okay = 21.0;
    let one_million = 1_000_000;

    println!("{}; {}; {}; {}",twenty,twenty_one,floats_okay,one_million);

    // trickery
    let three = 0b11; //Binary
    let thirty = 0o36; //Octal
    let threee_hundred = 0x12C; //Hexidecimal

    println!("base 10: {} {} {}", three,thirty,threee_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, threee_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, threee_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, threee_hundred);

    let needle = 42;
    let haystack = [1,1,2,5,14,42,132,429,1430,4862];
    
    for reference in haystack.iter() {
        let item = *reference;
        if item == needle {
            println!("{}",item);
        }
    }

}

fn add(i: i32, j: i32) -> i32 {
    i + j
}
