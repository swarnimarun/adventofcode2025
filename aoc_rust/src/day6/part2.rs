#[derive(Debug)]
enum Op {
    ADD,
    MUL,
}
pub fn run(input: String) {
    let mut iter = input.trim().lines().rev();
    let mut ops = iter
        .next()
        .map(|s| {
            s.split(' ')
                .filter(|s| !s.is_empty())
                .rev()
                .map(|s| match s.trim() {
                    "+" => Op::ADD,
                    "*" => Op::MUL,
                    _ => panic!("last line isn't valid operators"),
                })
        })
        .unwrap();
    let mut iter = iter.rev();
    let init = iter
        .next()
        .unwrap()
        .bytes()
        .rev() // right to left
        .map(|c| u8::from(c >= b'0' && c <= b'9') * (c - b'0'))
        .map(i64::from)
        .collect::<Vec<_>>();
    let values = iter.fold(
        init,
        // right to left
        |acc, s| {
            acc.iter()
                .zip(
                    s.bytes()
                        .rev()
                        .map(|c| u8::from(c >= b'0' && c <= b'9') * (c - b'0'))
                        .map(i64::from),
                )
                .map(|(a, b)| if b == 0 { *a } else { *a * 10 + b })
                .collect()
        },
    );
    let mut stack = vec![];
    let mut results = vec![];
    for r in values {
        if r == 0 {
            if !stack.is_empty() {
                let op = ops.next().unwrap();
                let r = match op {
                    Op::ADD => stack.iter().fold(0i64, |acc, i| acc + *i),
                    Op::MUL => stack.iter().fold(1i64, |acc, i| acc * *i),
                };
                results.push(r);
                stack.clear();
            }
            continue;
        }
        stack.push(r);
    }
    if !stack.is_empty() {
        let op = ops.next().unwrap();
        let r = match op {
            Op::ADD => stack.iter().fold(0i64, |acc, i| acc + *i),
            Op::MUL => stack.iter().fold(1i64, |acc, i| acc * *i),
        };
        results.push(r);
        stack.clear();
    }
    println!("Final result: {}", results.into_iter().sum::<i64>());
}
