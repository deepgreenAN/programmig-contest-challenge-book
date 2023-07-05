use std::cmp::Ordering;

/// T:Ordに関するVecの拡張
pub trait OrdVecExt<T: Ord> {
    /// 自前で実装した二分探索
    fn binary_search_(&self, key: &T) -> Option<usize>;
}

impl<T: Ord> OrdVecExt<T> for Vec<T> {
    fn binary_search_(&self, key: &T) -> Option<usize> {
        let mut left = 0_usize;
        let mut right = self.len().saturating_sub(1);

        while right > left {
            let mid = left + (right - left) / 2_usize;

            match self.get(mid)?.cmp(key) {
                // 見つかった場合
                Ordering::Equal => return Some(mid),
                // Keyに比べてmidの値が小さい場合
                Ordering::Less => {
                    left = mid + 1; // 探索範囲の左端をmid+1にする(どちらもmidだと無限ループになる)
                }
                // Keyに比べてmidの値が大きい場合
                Ordering::Greater => {
                    right = mid; // 探索範囲の右端をmidにする
                }
            }
        }
        None // 見つからなかった場合
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
