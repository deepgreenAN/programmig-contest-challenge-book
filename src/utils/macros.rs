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

/// HashMapを作成するマクロ
#[macro_export]
macro_rules! hash_map {
    // 最後が,で終る場合
    ($($key:expr => $value:expr,)+) => { crate::hash_map!{$($key => $value),+}};
    // 間にカンマを使う場合
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                let _ = map.insert($key, $value);
            )*
            map
        }
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

    #[test]
    fn test_hash_map() {
        let map = crate::hash_map! {
            "a" => 0,
            "b" => 1,
            "c" => 2,
        };
        assert_eq!(map.get("a").unwrap(), &0);
        assert_eq!(map.get("b").unwrap(), &1);
        assert_eq!(map.get("c").unwrap(), &2);
    }
}
