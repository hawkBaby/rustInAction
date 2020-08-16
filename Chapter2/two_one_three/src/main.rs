fn main() {
//    let needle = 42;
    let haystack = [1,1,2,5,14,42,132,429,1430,4862];

    for reference in haystack.iter() {
        let item = *reference;

        let result = match item {
            42 | 132 | 1430 => "hit!",
            _ => "miss"
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}
