use std::ops::Range;

/// (i, j)のようにインデックスの組み合わせを提供するイテレーター
pub struct Combination2dIter {
    range: Range<usize>,
    i: usize,
    j: usize,
}

impl Iterator for Combination2dIter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= self.range.end {
            // 一つ段を下がる
            self.i += 1;
            self.j = self.i;

            let res = if self.i >= self.range.end {
                None
            } else {
                Some((self.i, self.j))
            };

            self.j += 1;
            res
        } else {
            let res = Some((self.i, self.j));

            self.j += 1;
            res
        }
    }
}

/// Rangeの2dの組み合わせを(usize, usize)を返すイテレータ―として取得
pub fn combination_2d(index_range: Range<usize>) -> Combination2dIter {
    let start = index_range.start;

    Combination2dIter {
        range: index_range,
        i: start,
        j: start,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_combination_2d() {
        assert_eq!(
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 2),
                (2, 3),
                (3, 3)
            ],
            super::combination_2d(0..4).collect::<Vec<(usize, usize)>>()
        )
    }
}
