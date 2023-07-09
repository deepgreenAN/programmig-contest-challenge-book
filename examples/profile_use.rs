use challenge_book_macros::{get_profile, init_profiler_closure, profile, profile_closure};

#[profile]
fn fibo(n: usize) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibo(n - 1).saturating_add(fibo(n - 2))
    }
}

fn main() {
    println!("fibo(30): {}", fibo(30));
    let profile = get_profile!(fibo);
    println!("{profile:?}");

    init_profiler_closure!("x_closure");
    let x = profile_closure!("x_closure", |a| { a + 20 });
    x(10);
    println!("{:?}", get_profile!("x_closure"));
}
