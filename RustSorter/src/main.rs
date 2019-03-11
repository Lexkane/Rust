#[allow(dead_code)]
fn main() {
    println!("Sorting algorithms!");

    let mut v0: Vec<i64> = vec![43, 12, 53, 121, 3, 12, 3];
    bubble_sort(v0.as_mut_slice());
    println!("{:?}", v0);
    let mut v1: Vec<i64> = vec![3, 2, 1, 6, 4, 9];
    selection_sort(v1.as_mut_slice());
    println!("{:?}", v1);

    let mut v2: Vec<u32> = vec![0, 1, 0, 0, 1, 0];
    count_sort_binary(v2.as_mut_slice());
    println!("{:?}", v2);

    let mut v3: Vec<i64> = vec![342, 223, 10, 61, 43, 9];
    insertion_sort(v3.as_mut_slice());
    println!("{:?}", v3);

    println!("{:?}", assert!(is_sorted(&v0)));
    println!("{:?}", assert!(is_sorted(&v1)));
    println!("{:?}", assert!(is_sorted(&v2)));
    println!("{:?}", assert!(is_sorted(&v3)));
}

fn is_sorted<I>(data: I) -> bool
    where
        I: IntoIterator,
        I::Item: Ord,
{
    let mut it = data.into_iter();
    match it.next() {
        None => true,
        Some(first) => it.scan(first, |state, next| {
            let cmp = *state <= next;
            *state = next;
            Some(cmp)
        }).all(|b| b),
    }
}


fn bubble_sort(list: &mut [i64]) {
    for i in 0..list.len() {
        for y in 0..list.len() {
            if list[i] < list[y] {
                list.swap(i, y);
            }
        }
    }
}

fn selection_sort(list: &mut [i64]) {
    for i in 0..list.len() {
        let mut small = i;
        for j in (i + 1)..list.len() {
            if list[j] < list[small] {
                small = j;
            }
        }
        list.swap(small, i);
    }
}


fn insertion_sort<T: Ord>(list: &mut [T]) {
    for i in 0..list.len() {
        if let Some((j, _)) = list.iter()
            .enumerate()
            .skip(i)
            .min_by_key(|x| x.1) {
            list.swap(i, j);
        }
    }
}


pub fn count_sort_binary(list: &mut [u32]) {
    let (zero_count, _one_count) = list.iter()
        .fold((0, 0),
              |(zero, one), &el| {
                  if el == 0 {
                      (zero + 1, one)
                  } else {
                      (zero, one + 1)
                  }
              });

    for i in 0..zero_count {
        list[i] = 0;
    }
    for i in zero_count..list.len() {
        list[i] = 1;
    }
}