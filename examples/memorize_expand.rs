use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static FIBO_CACHE: OnceLock<Mutex<HashMap<usize, u32>>> = OnceLock::new();

fn fibo(n: usize) -> u32 {
    // キャッシュの中を探す
    {
        if let Some(value) = FIBO_CACHE
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap()
            .get(&n)
        {
            return value.clone();
        }
    }

    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let answer = fibo(n - 1).saturating_add(fibo(n - 2));

    // キャッシュに追加
    {
        FIBO_CACHE
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap()
            .insert(n, answer.clone());
    }

    answer
}

fn main() {
    use std::time::Instant;

    let start_time = Instant::now();
    let ret = fibo(50);

    println!("{ret:?}");
    println!("cached: {:?}", Instant::now() - start_time);
}
