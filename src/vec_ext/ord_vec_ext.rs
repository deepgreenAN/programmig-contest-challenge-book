use crate::utils::binary_search;

/// T:Ordに関するVecの拡張
pub trait OrdVecExt<T: Ord> {
    /// 自前で実装した二分探索
    fn binary_search_(&self, key: &T) -> Option<usize>;
}

impl<T: Ord> OrdVecExt<T> for Vec<T> {
    fn binary_search_(&self, key: &T) -> Option<usize> {
        binary_search(self, key)
    }
}

#[cfg(test)]
mod test {
    use super::OrdVecExt;

    #[test]
    fn test_binary_search() {
        {
            let array = vec![1, 3, 4, 6, 10];
            assert_eq!(array.binary_search_(&4), Some(2), "奇数個の場合");
        }

        {
            let array = vec![2, 3, 4, 12, 15, 21, 32, 33, 48, 50];
            assert_eq!(array.binary_search_(&48), Some(8), "偶数個の場合");
        }
    }
}
