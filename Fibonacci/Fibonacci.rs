extern crate threadpool;
extern crate num_cpus;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

    fn fib_real(number: u64) -> u64 {
        match number < 2 {
            true => number,
            false => run(number - 1) + run(number - 2),
        }
    }

struct FibonacciPool {
    pool: threadpool::ThreadPool,
}

impl Fibonacci {
    fn new() -> Self {
        let pool = ThreadPool::new(num_cpus::get());
        Fibonacci { 
            pool }
    }

    fn run(&self, number: u64) -> u64 {
        let (tx, rx) = channel();
        let tx = tx.clone();
        self.pool.execute(move ||  {
            tx.send(self.fib_real(number - 1)).expect("Could not send data!");
        });
        let tx = tx.clone();
        self.pool.execute(move ||  {
            tx.send(self.fib_real(number - 2)).expect("Could not send data!");
        });
        rx.iter().take(2).fold(0, |a, b| a + b)
    }
}

fn main() {
    let fibonacci = FibonacciPool::new();
    println!("{}", fibonacci.run(50));
}