use challenge_book_macros::{memorize, memorize_closure};

#[memorize]
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
    let ans = fibo(50);
    println!("{ans}");

    let x = memorize_closure!("a", |a: usize| -> u32 { a as u32 });
    x(10);
}
