extern crate lazy_static;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static!{
    static ref MAP: HashMap<u32,&'static str> ={
        let mut m=HashMap::new();
        m.insert(0,"foo")
        m.insert(1,"bar")
        m
    };
}
fn main() {
    println!("The entry for'0' is {}", MAP.get(&0).unwrap)
    println!("The entry for'1' is {}", MAP.get(&1).unwrap)
}
