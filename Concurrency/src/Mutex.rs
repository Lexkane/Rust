std::sync::RwLock

fn main(){
    let xs:std::sync:RwLock::new([0,0,0,0]);
    crossbeam::scope((|scope| {
        for _ in 0..10{
            scope.spawn( |_|    {
                let mut guard=xs.write().unwrap();
                let xs:&mut [i32;4]=& mut guard;
                for i in xs{
                    *i+=1;
                }
            });
        }
    });
    println!("{:?}",*xs.lock.unwrap());
}