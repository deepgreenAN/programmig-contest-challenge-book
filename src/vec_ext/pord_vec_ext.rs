use std::cmp::Ordering;

/// T:PartialOrdに関するVecの拡張
pub trait POrdVecExt<T: PartialOrd> {
    /// Max
    fn max(&self) -> Option<&T>;
    /// Min
    fn min(&self) -> Option<&T>;
    /// sort
    fn sort_(&mut self);
}

impl<T: PartialOrd> POrdVecExt<T> for Vec<T> {
    fn max(&self) -> Option<&T> {
        self.iter().max_by(|x, y| {
            x.partial_cmp(y).unwrap_or_else(|| {
                if x.partial_cmp(x).is_none() {
                    // 一番目がNaNの場合
                    Ordering::Less // 一番目が小さいとする(左に寄せる)
                } else {
                    // 二番目がNaNの場合
                    Ordering::Greater // 二番目が小さいとする(左に寄せる)
                }
            })
        })
    }
    fn min(&self) -> Option<&T> {
        self.iter().min_by(|x, y| {
            x.partial_cmp(y).unwrap_or_else(|| {
                if x.partial_cmp(x).is_none() {
                    // 一番目がNaNの場合
                    Ordering::Greater // 一番目が大きいとする(右に寄せる)
                } else {
                    // 二番目がNaNの場合
                    Ordering::Less // 二番目が大きいとする(右に寄せる)
                }
            })
        })
    }
    fn sort_(&mut self) {
        self.sort_by(|x, y| {
            x.partial_cmp(y).unwrap_or_else(|| {
                if x.partial_cmp(x).is_none() {
                    // 1番目がNaNの場合
                    Ordering::Less // Lessにして1番目を左に
                } else {
                    // 2番目がNaNの場合
                    Ordering::Greater // Greaterにして1番目を右に
                }
            })
        })
    }
}

#[cfg(test)]
mod test {
    use super::POrdVecExt;
    use float_cmp::approx_eq;

    #[test]
    fn test_max() {
        assert!(approx_eq!(
            f32,
            *(vec![1.0, 2.0, 3.0, 10.0, 5.0, 6.0, 7.0].max().unwrap()),
            10.0
        ))
    }

    #[test]
    fn test_min() {
        assert!(approx_eq!(
            f32,
            *(vec![1.0, 2.0, 3.0, -10.0, 5.0, 6.0, 7.0].min().unwrap()),
            -10.0
        ))
    }

    #[test]
    fn test_sort() {
        let mut x = vec![1.0, f32::NAN, 0.5, 3.0, 0.2, f32::INFINITY, 2.0];
        x.sort_();

        assert!(approx_eq!(
            &[f32],
            &x,
            &[f32::NAN, 0.2, 0.5, 1.0, 2.0, 3.0, f32::INFINITY]
        ));
    }
}
