use std::time;



fn main() {
    let st = time::Instant::now();
    let count = (2..200000).filter(|&p| is_prime(p)).count();
    let en = time::Instant::now();
  
    println!("{:?} {}", en - st, count);
    
    let st = time::Instant::now();
    let count = (2..200000).filter(|&p| is_prime2(p)).count();
    let en = time::Instant::now();
  
    println!("{:?} {}", en - st, count);

    let st = time::Instant::now();
    let count = (2..200000).filter(|&p| is_prime3(p)).count();
    let en = time::Instant::now();
  
    println!("{:?} {}", en - st, count);

        let st = time::Instant::now();
    let count = (2..200000).filter(|&p| is_prime(p)).count();
    let en = time::Instant::now();
  
    println!("{:?} {}", en - st, count);
    
    let st = time::Instant::now();
    let count = (2..200000).filter(|&p| is_prime2(p)).count();
    let en = time::Instant::now();
  
    println!("{:?} {}", en - st, count);

    let st = time::Instant::now();
    let count = (2..200000).filter(|&p| is_prime3(p)).count();
    let en = time::Instant::now();
  
    println!("{:?} {}", en - st, count);

}

fn is_prime(n: u64) -> bool {
    (2..).take_while(|x| x * x <= n).all(|i| n % i != 0)
}

fn is_prime2(n: u64) -> bool {
    let m = (n as f64).sqrt();
    (2..).take_while(|x| *x as f64 <= m).all(|i| n % i != 0)
}

fn is_prime3(n: u64) -> bool {
    let m = (n as f64).sqrt().floor() as u64;
    (2..).take_while(|x| *x <= m).all(|i| n % i != 0)
}