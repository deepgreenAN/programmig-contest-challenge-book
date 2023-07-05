use std::ops::Range;

/// 八近傍のインデックスを返すイテレータ―
pub struct EightNeighborIter {
    /// 横方向のレンジ
    i_range: Range<usize>,
    /// 縦方向のレンジ
    j_range: Range<usize>,
    /// 横の位置
    i: usize,
    /// 縦の位置
    j: usize,
    /// 1: 中上, 2: 右上, 3: 右中, 4: 右下, 5: 中下, 6: 左下, 7: 左中, 8: 左上
    counter: usize,
}

/// - 1, + 1 されるときにusizeのオーバーフローになるのを防ぐ．
impl Iterator for EightNeighborIter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1; // 1から始まる
        let (i, j) = (self.i, self.j);

        match self.counter {
            1 => {
                if j.checked_sub(1).is_some() && self.j_range.contains(&(j - 1)) {
                    Some((i, j - 1))
                } else {
                    self.next()
                }
            }
            2 => {
                if i.checked_add(1).is_some()
                    && j.checked_sub(1).is_some()
                    && self.i_range.contains(&(i + 1))
                    && self.j_range.contains(&(j - 1))
                {
                    Some((i + 1, j - 1))
                } else {
                    self.next()
                }
            }
            3 => {
                if i.checked_add(1).is_some() && self.i_range.contains(&(i + 1)) {
                    Some((i + 1, j))
                } else {
                    self.next()
                }
            }
            4 => {
                if i.checked_add(1).is_some()
                    && j.checked_add(1).is_some()
                    && self.i_range.contains(&(i + 1))
                    && self.j_range.contains(&(j + 1))
                {
                    Some((i + 1, j + 1))
                } else {
                    self.next()
                }
            }
            5 => {
                if j.checked_add(1).is_some() && self.j_range.contains(&(j + 1)) {
                    Some((i, j + 1))
                } else {
                    self.next()
                }
            }
            6 => {
                if i.checked_sub(1).is_some()
                    && j.checked_add(1).is_some()
                    && self.i_range.contains(&(i - 1))
                    && self.j_range.contains(&(j + 1))
                {
                    Some((i - 1, j + 1))
                } else {
                    self.next()
                }
            }
            7 => {
                if i.checked_sub(1).is_some() && self.i_range.contains(&(i - 1)) {
                    Some((i - 1, j))
                } else {
                    self.next()
                }
            }
            8 => {
                if i.checked_sub(1).is_some()
                    && j.checked_sub(1).is_some()
                    && self.i_range.contains(&(i - 1))
                    && self.j_range.contains(&(j - 1))
                {
                    Some((i - 1, j - 1))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// 八近傍のインデックスをイテレータ―として取得
pub fn eight_neighbors(
    i: usize,
    j: usize,
    i_range: Range<usize>,
    j_range: Range<usize>,
) -> EightNeighborIter {
    EightNeighborIter {
        i_range,
        j_range,
        i,
        j,
        counter: 0,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_eight_neibors() {
        use super::eight_neighbors;

        let i_range = 0_usize..3_usize;
        let j_range = 0_usize..3_usize;

        assert_eq!(
            eight_neighbors(0, 0, i_range.clone(), j_range.clone()).collect::<Vec<_>>(),
            vec![(1, 0), (1, 1), (0, 1)]
        );
        assert_eq!(
            eight_neighbors(1, 0, i_range.clone(), j_range.clone()).collect::<Vec<_>>(),
            vec![(2, 0), (2, 1), (1, 1), (0, 1), (0, 0)]
        );
        assert_eq!(
            eight_neighbors(2, 0, i_range.clone(), j_range.clone()).collect::<Vec<_>>(),
            vec![(2, 1), (1, 1), (1, 0)]
        );
        assert_eq!(
            eight_neighbors(0, 1, i_range.clone(), j_range.clone()).collect::<Vec<_>>(),
            vec![(0, 0), (1, 0), (1, 1), (1, 2), (0, 2)]
        );
        assert_eq!(
            eight_neighbors(1, 1, i_range.clone(), j_range.clone()).collect::<Vec<_>>(),
            vec![
                (1, 0),
                (2, 0),
                (2, 1),
                (2, 2),
                (1, 2),
                (0, 2),
                (0, 1),
                (0, 0)
            ]
        );
        assert_eq!(
            eight_neighbors(2, 1, i_range.clone(), j_range.clone()).collect::<Vec<_>>(),
            vec![(2, 0), (2, 2), (1, 2), (1, 1), (1, 0)]
        );
        assert_eq!(
            eight_neighbors(0, 2, i_range.clone(), j_range.clone()).collect::<Vec<_>>(),
            vec![(0, 1), (1, 1), (1, 2)]
        );
        assert_eq!(
            eight_neighbors(1, 2, i_range.clone(), j_range.clone()).collect::<Vec<_>>(),
            vec![(1, 1), (2, 1), (2, 2), (0, 2), (0, 1)]
        );
        assert_eq!(
            eight_neighbors(2, 2, i_range.clone(), j_range.clone()).collect::<Vec<_>>(),
            vec![(2, 1), (1, 2), (1, 1)]
        );
    }
}
