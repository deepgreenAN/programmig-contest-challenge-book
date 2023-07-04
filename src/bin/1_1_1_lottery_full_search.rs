fn lottery(k: &[i32], m: i32) -> Option<Vec<i32>> {
    let mut ans = Option::<Vec<i32>>::None;

    // 4重ループにより全探索
    for a in 0..k.len() {
        for b in 0..k.len() {
            for c in 0..k.len() {
                for d in 0..k.len() {
                    if k[a] + k[b] + k[c] + k[d] == m {
                        ans = Some(vec![k[a], k[b], k[c], k[d]]);
                    }
                }
            }
        }
    }

    ans
}

fn main() {
    {
        let k = vec![1, 3, 5];
        let m = 10;
        println!("k: {k:?}, m: {m:?}, ans: {:?}", lottery(&k, m));
    }
    {
        let k = vec![1, 3, 5];
        let m = 9;
        println!("k: {k:?}, m: {m:?}, ans: {:?}", lottery(&k, m));
    }
}
