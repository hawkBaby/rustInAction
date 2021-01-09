// Designed to throw an error around race conditions
use std::thread;

fn main() {
    let mut data = 100;
    thread::spawn(|| {data = 500; });
    thread::spawn(|| {data = 1000; });

    println!("{}",data);
}