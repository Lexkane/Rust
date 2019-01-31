extern crate primal;

fn main() {
    let primes: Vec<_> = primal::Sieve::new(20_000_000)
        .primes_from(0)
        .take_while(|&p| p < 20_000_000)
        .collect();
    println!("Found {} primes.", primes.len());
}