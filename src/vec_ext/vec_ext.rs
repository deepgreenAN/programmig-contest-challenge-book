/// 汎用的なVecの拡張
pub trait VecExt<T> {
    /// P(x) = trueとなる最小のxのインデックスを返す
    fn lower_bound<F: FnMut(&T) -> bool>(&self, predicate: F) -> Option<usize>;
}

impl<T> VecExt<T> for Vec<T> {
    fn lower_bound<F: FnMut(&T) -> bool>(&self, mut predicate: F) -> Option<usize> {
        // 左端が条件を満たす場合
        if predicate(self.first()?) {
            return Some(0_usize);
        }
        // 右端が条件を満たさない場合
        if !predicate(self.last()?) {
            return None;
        }

        let mut left = 0_usize;
        let mut right = self.len().saturating_sub(1);

        while right > left {
            let mid = left + (right - left) / 2_usize;

            if predicate(self.get(mid)?) {
                // 区間の中心が条件を満たす場合
                right = mid; // 探索範囲の右端をmidにする
            } else {
                // 区間の中心が条件を満たさない場合
                left = mid + 1; // 探索範囲の左端をmid+1にする(どちらもmidだと無限ループになる)
            }
        }

        Some(right)
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
