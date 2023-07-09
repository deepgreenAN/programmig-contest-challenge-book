#[derive(Clone, Debug)]
struct WorkTime {
    id: usize,
    start: usize,
    end: usize,
}

fn work_schedule(works: &mut Vec<WorkTime>) -> Vec<usize> {
    works.sort_by_key(|work| work.end);

    let mut t = 0_usize; // 現在時刻
    let mut works_iter = works.iter();
    let mut ans = Vec::<usize>::new();

    while let Some(work) = works_iter.find(|work| t < work.start) {
        ans.push(work.id);
        t = work.end;
    }

    ans
}

fn main() {
    let mut works = vec![
        WorkTime {
            id: 0,
            start: 1,
            end: 3,
        },
        WorkTime {
            id: 1,
            start: 2,
            end: 5,
        },
        WorkTime {
            id: 2,
            start: 4,
            end: 7,
        },
        WorkTime {
            id: 3,
            start: 6,
            end: 9,
        },
        WorkTime {
            id: 4,
            start: 8,
            end: 10,
        },
    ];
    println!("works: {works:?}");

    let ans = work_schedule(&mut works);
    println!("ans: {ans:?}");
}
