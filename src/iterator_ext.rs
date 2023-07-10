mod multi_peakable;

use multi_peakable::MultiPeekable;

pub trait IteratorExt: Iterator {
    /// 複数ピークができるイテレータ―を返す．
    /// ```rust
    /// use challenge_book::IteratorExt;
    ///
    /// let x = vec![1, 2, 3, 4, 5, 6, 7];
    /// let multi_peek = x.iter().multi_peek(3);
    ///
    /// let mut moving_average = Vec::new();
    ///
    /// || -> Option<()> {
    ///     loop {
    ///         let item = multi_peak.next()?;
    ///
    ///         moving_average.push(
    ///             (*item
    ///                 + *multi_peak.multi_peak(0)?
    ///                 + *multi_peak.multi_peak(1)?
    ///                 + *multi_peak.multi_peak(2)?) as f32
    ///                 / 4.0,
    ///         );
    ///     }
    /// }();
    ///
    /// assert!(approx_eq!(&[f32], &moving_average, &[2.5, 3.5, 4.5, 5.5]));
    /// ```
    fn multi_peek(self, buf_size: usize) -> MultiPeekable<Self>
    where
        Self: Sized,
    {
        MultiPeekable::new(self, buf_size)
    }
}

impl<I: Iterator + ?Sized> IteratorExt for I {}
