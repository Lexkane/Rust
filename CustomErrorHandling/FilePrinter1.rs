use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> i32 {
    let mut file = File::open(file_path).unwrap(); // handling error 1
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();   // handling error 2
    let n: i32 = contents.trim().parse().unwrap(); //handling error 3 
    2 * n
}

fn main() {
    let doubled = file_double("foobar");
    println!("{}", doubled);
}