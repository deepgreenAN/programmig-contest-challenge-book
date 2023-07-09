use challenge_book::utils::FuncProfile;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

static FIBO_PROFILER: OnceLock<Mutex<FuncProfile>> = OnceLock::new();

fn fibo(n: usize) -> u32 {
    // 開始時間を計測
    let profile_start_xxx = Instant::now(); // 名前が被っていないかチェックする必要がある．

    let inner_func = |n| {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }

        fibo(n - 1).saturating_add(fibo(n - 2))
    };

    let ans = inner_func(n);

    // Durationを取得、プロファイラを更新
    {
        let duration = Instant::now() - profile_start_xxx;
        let mut profiler_guard = FIBO_PROFILER
            .get_or_init(|| Mutex::new(FuncProfile::default()))
            .lock()
            .unwrap();

        profiler_guard.mean_t = ((profiler_guard.mean_t * profiler_guard.call_n) + duration)
            / (profiler_guard.call_n + 1);
        profiler_guard.call_n += 1;
    }

    ans
}

fn profile_fibo() -> FuncProfile {
    match FIBO_PROFILER.get() {
        Some(profiler) => profiler.lock().unwrap().clone(),
        None => Default::default(),
    }
}

fn main() {
    let ans = fibo(1);
    println!("ans: {ans}, profile: {:?}", profile_fibo());

    static PROFILER_X_CLOSURE: ::std::sync::OnceLock<
        ::std::sync::Mutex<::challenge_book::utils::FuncProfile>,
    > = ::std::sync::OnceLock::new();

    let x = |a| {
        let profile_start_xxx = ::std::time::Instant::now();
        let block_fn = |a| a + 20;
        let res = block_fn(a);
        {
            let duration = profile_start_xxx.elapsed();
            let mut profiler_guard = PROFILER_X_CLOSURE
                .get_or_init(|| ::std::sync::Mutex::new(Default::default()))
                .lock()
                .unwrap();
            profiler_guard.mean_t = ((profiler_guard.mean_t * profiler_guard.call_n) + duration)
                / (profiler_guard.call_n + 1);
            profiler_guard.call_n += 1;
        }
        res
    };

    x(10);

    let profiler = match PROFILER_X_CLOSURE.get() {
        Some(profiler) => profiler.lock().unwrap().clone(),
        None => Default::default(),
    };
    println!("{:?}", profiler);
}
