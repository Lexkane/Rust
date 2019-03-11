extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
use std::io
use std::prelude::*;

fn stringReverse(s :String) -> <String>{
return s.chars().rev().collect::<String>;
}




fn main(){

let stdin=io.stdin();
for line in stdin::lock().lines(){
    println!("{}",line.unwrap());
}



let locked=stdin.lock()
let v:Vec<String> =locked.lines().filter_map(|line| line.ok()).collect();


let word:& str="lowks";
let drow:String=word
    .graphemes(true)
    .rev()
    .flat_map(|g| g.chars())
    .collect();


    println!("drow={}",drow)





}