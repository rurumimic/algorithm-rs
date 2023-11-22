use std::fmt::Display;

#[allow(dead_code)]
fn show<T: Display>(arr: &[T]) {
    arr.iter().for_each(|a| print!("{} ", a));
    println!();
}

pub fn bubble_sort<T: Ord + Display>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    let mut sorted = false;
    let mut n = arr.len();

    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

#[allow(dead_code)]
#[allow(unused_assignments)]
pub fn bubble_sort_2<T: Ord + Display>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    let mut sorted = false;
    let n = arr.len();

    for i in 0..n - 1 {
        sorted = true;
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::{have_same, is_asc};

    fn test_func<F>(sort_func: F)
    where
        F: Fn(&mut [usize]),
    {
        let cases = vec![
            (vec![], vec![]),
            (vec![1], vec![1]),
            (vec![3, 2, 1], vec![1, 2, 3]),
            (vec![1, 2, 3, 4], vec![1, 2, 3, 4]),
            (vec![3, 1, 5, 2, 4], vec![1, 2, 3, 4, 5]),
        ];

        for (mut input, expected) in cases {
            let cloned = input.clone();
            sort_func(&mut input);
            show(&input);
            assert!(is_asc(&input) && have_same(&input, &cloned));
            assert_eq!(input, expected);
        }
    }

    #[test]
    fn test_1() {
        test_func(|arr| bubble_sort(arr));
    }

    #[test]
    fn test_2() {
        test_func(|arr| bubble_sort_2(arr));
    }
}
