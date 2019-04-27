struct Wrapper{
    value:Box<i32>,
}

fn main(){
    let w=Wrapper{value:Box::new (92)};
    let r:&32=&*w.value;
    w.value=Box::new(62);
    if w.value>640{println("enough");}
    println(*{}*,*r)
}