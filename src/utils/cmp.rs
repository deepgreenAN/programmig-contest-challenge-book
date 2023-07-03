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
}
