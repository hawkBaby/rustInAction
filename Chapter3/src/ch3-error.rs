static mut ERROR: i32 = 0;

use std::fs::File;
use std::fs::read;

fn main() {
    let mut f = File::new("something.txt");

    read(f, buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while reading the file ")
        }
    }

    close(f);
    unsafe {
        if ERROR != 0 {
            panic!("An error has closed while closing the file ");
        }
    }
}