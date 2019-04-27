#![allow(unused)]
extern crate rayon; // 1.0.3

fn main() {
    fn joining_children(a: usize, b: usize) -> usize {
        a + b
    }

    mod rayon_bench {
        use rayon::prelude::*;

        pub fn test(a: Vec<usize>) {
            a.par_iter()
                .map(|f| {
                    let s = a.par_iter().map(|s| s + f);
                    s.collect::<usize>()
                })
                .collect()
                .flatten();
        }
    }

    let a = (0..1000).collect();
    rayon_bench::test(a);
}
