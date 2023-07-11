use std::cmp::Ordering;

/// 二分探索
pub fn binary_search<T: Ord>(array: &[T], key: &T) -> Option<usize> {
    let mut left = 0_usize;
    let mut right = array.len().saturating_sub(1);

    while right > left {
        let mid = left + (right - left) / 2_usize;

        match array.get(mid)?.cmp(key) {
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

pub fn lower_bound<F, T>(array: &[T], mut predicate: F) -> Option<usize>
where
    F: FnMut(&T) -> bool,
{
    // 左端が条件を満たす場合
    if predicate(array.first()?) {
        return Some(0_usize);
    }
    // 右端が条件を満たさない場合
    if !predicate(array.last()?) {
        return None;
    }

    let mut left = 0_usize;
    let mut right = array.len().saturating_sub(1);

    while right > left {
        let mid = left + (right - left) / 2_usize;

        if predicate(array.get(mid)?) {
            // 区間の中心が条件を満たす場合
            right = mid; // 探索範囲の右端をmidにする
        } else {
            // 区間の中心が条件を満たさない場合
            left = mid + 1; // 探索範囲の左端をmid+1にする(どちらもmidだと無限ループになる)
        }
    }

    Some(right)
}

// dpの更新の時に利用する．x > y の場合に x を y に更新する
pub fn ch_min<T: PartialOrd>(x: &mut T, y: T) {
    if *x > y {
        *x = y;
    }
}

// dpの更新の時に利用する． x < y の場合に x を y に更新する
pub fn ch_max<T: PartialOrd>(x: &mut T, y: T) {
    if *x < y {
        *x = y;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_binary_search() {
        {
            let array = vec![1, 3, 4, 6, 10];
            assert_eq!(super::binary_search(&array, &4), Some(2), "奇数個の場合");
        }

        {
            let array = vec![2, 3, 4, 12, 15, 21, 32, 33, 48, 50];
            assert_eq!(super::binary_search(&array, &48), Some(8), "偶数個の場合");
        }
    }

    #[test]
    fn test_lower_bound() {
        let a = vec![3, 5, 8, 10, 14, 17, 21, 39];

        assert_eq!(super::lower_bound(&a, |x| { *x >= 9 }), Some(3));
        assert_eq!(super::lower_bound(&a, |x| { *x >= 10 }), Some(3));
        assert_eq!(super::lower_bound(&a, |x| { *x >= 0 }), Some(0));
        assert_eq!(super::lower_bound(&a, |x| { *x >= 50 }), None);
    }
}
