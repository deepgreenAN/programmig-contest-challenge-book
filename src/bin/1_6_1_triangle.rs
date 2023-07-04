use challenge_book::max;

fn max_circumference(a: &[i32]) -> Option<i32> {
    let mut ans = Option::<i32>::None;

    // 重複を避けるため i < j < k
    for i in 0..a.len() {
        for j in (i + 1)..a.len() {
            for k in (j + 1)..a.len() {
                let circumference = a[i] + a[j] + a[k];
                let max_edge = max!(a[i], a[j], a[k]);

                let rest = circumference - max_edge;

                // 最も長い辺よりそれを含めない二つの辺の長さの合計の方が長い場合
                if max_edge < rest {
                    ans = Some(match ans {
                        Some(ans_value) => {
                            max!(ans_value, circumference)
                        }
                        None => circumference,
                    });
                }
            }
        }
    }

    ans
}

fn main() {
    {
        let a = vec![2, 3, 4, 5, 10];
        println!("a: {a:?}, ans: {:?}", max_circumference(&a));
    }
    {
        let a = vec![4, 5, 10, 20];
        println!("a: {a:?}, ans: {:?}", max_circumference(&a));
    }
}
