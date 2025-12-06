enum Op {
    ADD,
    MUL,
}
pub fn run(input: String) {
    let mut iter = input
        .trim()
        .lines()
        .map(|s| s.split(' ').filter(|s| !s.is_empty()))
        .rev();
    let ops = iter
        .next()
        .map(|s| {
            s.map(|s| match s.trim() {
                "+" => Op::ADD,
                "*" => Op::MUL,
                _ => panic!("last line isn't valid operators"),
            })
        })
        .unwrap();
    let init = iter
        .next()
        .unwrap()
        .map(str::parse::<i64>)
        .filter_map(Result::ok)
        .collect::<Vec<i64>>();
    let results = iter.fold(init, |acc, rows| {
        rows.map(str::parse::<i64>)
            .filter_map(Result::ok)
            .zip(ops.clone())
            .zip(acc.iter())
            .map(|((i, op), p)| match op {
                Op::ADD => *p + i,
                Op::MUL => *p * i,
            })
            .collect()
    });
    println!("Final result: {}", results.into_iter().sum::<i64>());
}
