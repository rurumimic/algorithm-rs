pub mod sort;

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn bubble_sort() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        sort::bubble_sort(&mut vec);
        assert!(sort::is_asc(&vec));
    }
}
