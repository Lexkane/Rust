use rayon::prelude::*;

fn main() {
    fn joining_children(a: usize, b: usize) -> usize {
        a + b
    }

    mod rayon_bench {
        use rayon::prelude::*;

        pub fn test(a: Vec<usize>) {
            let collection: Vec<_> = a
                .par_iter()
                .flat_map(|f| a.par_iter().map(move |s| s + f))
                .collect();
        }
    }

    let a = (0..1000).collect();
    rayon_bench::test(a);
}
