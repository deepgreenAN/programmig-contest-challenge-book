use std::collections::VecDeque;

/// 複数ピークができるイテレータ―．itertoolsのものとは異なり，常に固定サイズのバッファに値を入れ，ピークではインデックスで取得する．
/// 詳しくはIteratorExt::multi_peekを確認．
#[derive(Clone, Debug)]
pub struct MultiPeekable<I>
where
    I: Iterator,
{
    iter: I,
    buf: VecDeque<I::Item>,
}

impl<I: Iterator> MultiPeekable<I> {
    /// 必ずこのコンストラクタを通して生成する．
    pub(super) fn new(mut iter: I, buf_size: usize) -> Self {
        let mut buf = VecDeque::with_capacity(buf_size);

        for _ in 0..buf_size {
            if let Some(item) = iter.next() {
                buf.push_back(item);
            }
        }

        Self { iter, buf }
    }
}

impl<I: Iterator> Iterator for MultiPeekable<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(add_item) = self.iter.next() {
            self.buf.push_back(add_item);
        }

        self.buf.pop_front()
    }
}

impl<I: Iterator> MultiPeekable<I> {
    /// インデックスで指定してピーク．0を与えるとPeekable::peakと同じ．
    pub fn multi_peak(&self, index: usize) -> Option<&I::Item> {
        self.buf.get(index)
    }
    /// インデックスで指定して可変参照をピーク．0を与えるとPeekable::peak_mutと同じ．
    pub fn multi_peak_mut(&mut self, index: usize) -> Option<&mut I::Item> {
        self.buf.get_mut(index)
    }
}

#[cfg(test)]
mod test {
    use float_cmp::approx_eq;

    #[test]
    fn test_multi_peak() {
        // 移動平均を求めてみる
        let x = vec![1, 2, 3, 4, 5, 6, 7];

        let mut multi_peak = super::MultiPeekable::new(x.iter(), 3);

        let mut moving_average = Vec::new();

        || -> Option<()> {
            loop {
                let item = multi_peak.next()?;

                moving_average.push(
                    (*item
                        + *multi_peak.multi_peak(0)?
                        + *multi_peak.multi_peak(1)?
                        + *multi_peak.multi_peak(2)?) as f32
                        / 4.0,
                );
            }
        }();

        assert!(approx_eq!(&[f32], &moving_average, &[2.5, 3.5, 4.5, 5.5]));
    }
}
