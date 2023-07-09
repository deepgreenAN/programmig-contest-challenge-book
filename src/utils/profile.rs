use std::time::Duration;

/// 簡単な関数プロファイリング用の型
#[derive(Default, Debug, Clone)]
pub struct FuncProfile {
    pub call_n: u32,
    pub mean_t: Duration,
}
