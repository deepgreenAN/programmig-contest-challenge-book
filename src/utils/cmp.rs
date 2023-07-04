use std::cmp::Ordering;

/// PartialOrdに拡張したMax関数
pub fn max<T: PartialOrd>(x: T, y: T) -> T {
    match x.partial_cmp(&y) {
        Some(ordering) => match ordering {
            Ordering::Greater | Ordering::Equal => x,
            Ordering::Less => y,
        },
        None => {
            if x.partial_cmp(&x).is_none() {
                // xがNaNだった場合
                y
            } else {
                // yがNaNだった場合
                x
            }
        }
    }
}

/// PartialOrdに拡張したMin関数
pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    match x.partial_cmp(&y) {
        Some(ordering) => match ordering {
            Ordering::Less | Ordering::Equal => x,
            Ordering::Greater => y,
        },
        None => {
            if x.partial_cmp(&x).is_none() {
                // xがNaNだった場合
                y
            } else {
                // yがNaNだった場合
                x
            }
        }
    }
}

/// 二分探索
pub fn binary_search<T: Ord>(array: &[T], key: &T) -> Option<usize> {
    let mut left = 0_usize;
    let mut right = array.len().saturating_sub(1);

    while right > left {
        let mid = left + (right - left) / 2_usize;

        match array.get(mid).unwrap().cmp(key) {
            // 見つかった場合
            Ordering::Equal => return Some(mid),
            // Keyに比べてmidの値が小さい場合
            Ordering::Less => {
                left = mid + 1; // 探索範囲の左端をmidにする
            }
            // Keyに比べてmidの値が大きい場合
            Ordering::Greater => {
                right = mid.saturating_sub(1); // 探索範囲の右端をmidにする
            }
        }
    }
    None // 見つからなかった場合
}

#[cfg(test)]
mod test {
    use float_cmp::approx_eq;

    #[test]
    fn test_max() {
        assert!(
            approx_eq!(f32, super::max(1.0, 2.0), 2.0),
            "maxの基本的な使い方"
        );
        assert!(!super::max(2.0, f64::NAN).is_nan(), "NANじゃない方が返る");
    }

    #[test]
    fn test_min() {
        assert!(
            approx_eq!(f32, super::min(1.0, 2.0), 1.0),
            "minの基本的な使い方"
        );
        assert!(!super::min(1.0, f64::NAN).is_nan(), "NANじゃない方が返る");
    }

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
        {
            let array = vec![1, 1, 1, 1, 1, 1, 1, 1];
            assert_eq!(super::binary_search(&array, &2), None);
        }
    }
}
