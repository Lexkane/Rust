//fork/join 

fn qucik_sort(xs:& [mut i32]){
    if xs.len()<=1{return}
    let mid=partition(xs);
    let(lo,hi)=sx.split_at_mut(mid);
    rayon::join(|| quick_sort(lo), || quick_sort(hi))
}

fn partition(xs:& mut[i32])->usize{/* _ */}

fn main(){
    let mut xs=[1,3,0,6,2,4,92]
    quick_sort(&mut xs);
}