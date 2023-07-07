use crate::enum_ext::IntoEnumIterator;
use crate::error::ConversionStructureError;

use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// EnumMapに対応した列挙体であることを示すマーカートレイト
pub trait EnumForMap: IntoEnumIterator + Hash + Eq {}

/// 列挙体をキーとするハッシュマップ．バリアントに対応したデータの取得を保証する．
pub struct EnumMap<E, V>
where
    E: EnumForMap,
{
    map: HashMap<E, V>,
}

impl<E: EnumForMap, V> EnumMap<E, V> {
    /// 列挙体のバリアントに対応した値を取得する．値の存在を保証する．
    pub fn get(&self, key: &E) -> &V {
        self.map.get(key).unwrap()
    }
    /// 列挙体のバリアントに対応した値を可変参照として取得する．値の存在を保証する．
    pub fn get_mut(&mut self, key: &E) -> &mut V {
        self.map.get_mut(key).unwrap()
    }
}

/// EnumMapを作成する唯一のインタフェース．enum_map!はこれを利用する．
impl<E: EnumForMap, V> TryFrom<HashMap<E, V>> for EnumMap<E, V> {
    type Error = ConversionStructureError;
    fn try_from(value: HashMap<E, V>) -> Result<Self, Self::Error> {
        for variant in E::iter() {
            value.get(&variant).ok_or(ConversionStructureError(
                "There is a missing key in the enum variant for EnumMap".to_string(),
            ))?;
        }
        if value.len() == E::iter().count() {
            Ok(EnumMap { map: value })
        } else {
            Err(ConversionStructureError(
                "There are duplicate variants.".to_string(),
            )) // マクロによってコンパイル時に補足されるはずのエラー
        }
    }
}

impl<E: EnumForMap + Debug, V: Debug> Debug for EnumMap<E, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.map)
    }
}
