use std::fmt::Display;

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
                arr.swap(i, i+1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::{is_sorted, is_same};

    #[test]
    fn empty() {
        let mut vec: Vec<usize> = vec![];
        let cloned = vec.clone();
        bubble_sort(&mut vec);
        assert!(is_sorted(&vec));
        assert!(is_same(&vec, &cloned));
    }

    #[test]
    fn ascending() {
        let answer: Vec<usize> = vec![1,2,3,4,5];
        let mut vec: Vec<usize> = vec![3,1,5,2,4];
        bubble_sort(&mut vec);
        assert!(is_sorted(&vec));
        assert!(is_same(&vec, &answer));
    }
}

