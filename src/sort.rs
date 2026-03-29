mod bubble_sort;

pub use self::bubble_sort::bubble_sort;

#[cfg(test)]
use std::cmp;

#[cfg(test)]
pub fn have_same<T>(a: &[T], b: &[T]) -> bool
where
    T: cmp::PartialOrd + cmp::Eq + std::hash::Hash,
{
    use std::collections::HashSet;
    match a.len() == b.len() {
        true => {
            // O(n^2)
            // b.iter().all(|item| a.contains(item))

            // O(n)
            let set_a: HashSet<&T> = a.iter().collect();
            let set_b: HashSet<&T> = b.iter().collect();
            set_a == set_b
        }
        false => false,
    }
}

#[cfg(test)]
pub fn is_asc<T>(arr: &[T]) -> bool
where
    T: cmp::PartialOrd,
{
    arr.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
pub fn is_desc<T>(arr: &[T]) -> bool
where
    T: cmp::PartialOrd,
{
    arr.windows(2).all(|w| w[0] >= w[1])
}
