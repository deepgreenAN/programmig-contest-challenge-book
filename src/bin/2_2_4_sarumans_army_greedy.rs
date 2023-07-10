fn solve(x: &mut [u32], r: u32) -> Vec<u32> {
    x.sort();
    let mut x_iter = x.iter().peekable();
    let mut ans = Vec::new();
    let mut most_left_point = x_iter.next();
    let mut last_marker_point = Option::<&u32>::None;

    || -> Option<()> {
        loop {
            let most_left_point_value = most_left_point?;
            let x_point = x_iter.next()?;

            // 最後に打った点+rの距離以内だった場合は次の点へ
            if let Some(last_marker_point) = last_marker_point {
                if &(*last_marker_point + r) >= x_point {
                    continue;
                }
            }

            // 一つ後の点をピークしながら
            match x_iter.peek() {
                Some(next_x_point) => {
                    if &(*most_left_point_value + r) < next_x_point {
                        // 左の点+rの距離よりnext_x_pointが大きい場合
                        ans.push(*x_point);
                        last_marker_point = Some(x_point);
                        most_left_point = Some(*next_x_point);
                    } else {
                        continue;
                    }
                }
                None => {
                    ans.push(*x_point);
                    break;
                }
            }
        }
        Some(())
    }();

    ans
}

fn main() {
    let mut x = vec![1_u32, 7, 15, 20, 30, 50];
    let r = 10_u32;

    println!("x:{x:?}, r:{r}, ans: {:?}", solve(&mut x, r));
}
