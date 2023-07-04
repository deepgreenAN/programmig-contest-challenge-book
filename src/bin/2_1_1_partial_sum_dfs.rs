fn dfs_recur(i: usize, sum: i32, array: &[i32], solution: &mut Vec<i32>) -> bool {
    // ベースケース
    if i == 0 {
        return sum == 0;
    }

    // a[i-1]を選ばない場合
    if dfs_recur(i - 1, sum, array, solution) {
        return true;
    }

    // a[i-1]を選ぶ場合
    if dfs_recur(i - 1, sum - array[i - 1], array, solution) {
        solution.push(array[i - 1]);
        return true;
    }

    false // どの場合もtrueにならなかった場合
}

fn dfs_partial_sum(sum: i32, array: &[i32]) -> Option<Vec<i32>> {
    let mut solution = Vec::<i32>::new();

    if dfs_recur(array.len(), sum, array, &mut solution) {
        Some(solution)
    } else {
        None
    }
}

fn main() {
    {
        let a = vec![1, 2, 4, 7];
        let k = 13;
        println!("a: {a:?}, k: {k:?}, ans: {:?}", dfs_partial_sum(k, &a));
    }
    {
        let a = vec![1, 2, 4, 7];
        let k = 15;
        println!("a: {a:?}, k: {k:?}, ans: {:?}", dfs_partial_sum(k, &a));
    }
}
