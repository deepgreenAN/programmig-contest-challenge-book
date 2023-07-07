/// enumをそのバリアントをイテレーションするイテレータ―に変換するトレイト
/// 条件として全てのバリアントがフィールドを持たない必要がある．
pub trait IntoEnumIterator: Sized {
    type Iterator: Iterator<Item = Self>;
    /// スタティクメソッドとして定義する
    fn iter() -> <Self as IntoEnumIterator>::Iterator;
}
