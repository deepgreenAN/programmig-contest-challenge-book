fn main() {
    use challenge_book::utils::lower_bound;

    {
        let a = vec![3, 5, 8, 10, 14, 17, 21, 39];
        assert_eq!(lower_bound(&a, |x| { *x >= 9 }), Some(3));
    }
}
