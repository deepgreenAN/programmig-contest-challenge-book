use crate::utils::lower_bound;

/// 汎用的なVecの拡張
pub trait VecExt<T> {
    /// P(x) = trueとなる最小のxのインデックスを返す
    fn lower_bound<F: FnMut(&T) -> bool>(&self, predicate: F) -> Option<usize>;
}

impl<T> VecExt<T> for Vec<T> {
    fn lower_bound<F: FnMut(&T) -> bool>(&self, predicate: F) -> Option<usize> {
        lower_bound(self, predicate)
    }
}

#[cfg(test)]
mod test {
    use super::VecExt;

    #[test]
    fn test_lower_bound() {
        let a = vec![3, 5, 8, 10, 14, 17, 21, 39];

        assert_eq!(a.lower_bound(|x| { *x >= 9 }), Some(3));
        assert_eq!(a.lower_bound(|x| { *x >= 10 }), Some(3));
        assert_eq!(a.lower_bound(|x| { *x >= 0 }), Some(0));
        assert_eq!(a.lower_bound(|x| { *x >= 50 }), None);
    }
}
