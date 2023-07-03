/// maxの引数を拡張するマクロ
#[macro_export]
macro_rules! max {
    // 引数が二つだけの場合
    ($x:expr, $y:expr) => {
        $crate::utils::max($x, $y)
    };
    // 要素が二つより多い場合(再帰)
    ($x:expr, $y:expr, $($rest:expr),+) => {
        $crate::utils::max(
            $x,
            $crate::utils::max(
                $y,
                $($rest),+
            )
        )
    };
}

/// minの引数を拡張するマクロ
#[macro_export]
macro_rules! min {
    // 引数が二つだけの場合
    ($x:expr, $y:expr) => {
        $crate::utils::min($x, $y)
    };
    // 要素が二つより多い場合(再帰)
    ($x:expr, $y:expr, $($rest:expr),+) => {
        $crate::utils::min(
            $x,
            $crate::utils::min(
                $y,
                $($rest),+
            )
        )
    };
}

#[cfg(test)]
mod test {
    use float_cmp::approx_eq;

    #[test]
    fn test_max() {
        assert!(
            approx_eq!(f32, crate::max!(1.0, 2.0, 3.0), 3.0),
            "max!の基本的な使い方"
        );
        assert!(
            approx_eq!(f32, crate::max!(3.0, 2.0, 1.0), 3.0),
            "max!の基本的な使い方"
        );
    }

    #[test]
    fn test_min() {
        assert!(
            approx_eq!(f32, crate::min!(1.0, 2.0, 3.0), 1.0),
            "min!の基本的な使い方"
        );
        assert!(
            approx_eq!(f32, crate::min!(3.0, 2.0, 1.0), 1.0),
            "min!の基本的な使い方"
        );
    }
}
