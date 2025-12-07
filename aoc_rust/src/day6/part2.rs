fn digit_iter(s: &str) -> impl Iterator<Item = i64> {
    s.bytes()
        .rev() // right to left
        .map(|c| u8::from(c >= b'0' && c <= b'9') * (c - b'0'))
        .map(i64::from)
}
#[derive(Debug)]
enum Op {
    ADD,
    MUL,
}
pub fn run(input: String) {
    let mut iter = input.trim().lines().rev();
    let ops = iter
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
    let init = iter.next().map(digit_iter).unwrap().collect::<Vec<_>>();
    let values = iter.fold(init, |acc, s| {
        acc.into_iter()
            .zip(digit_iter(s))
            .map(|(a, b)| if b == 0 { a } else { a * 10 + b })
            .collect()
    });
    let results = values
        .split(|s| *s == 0)
        .filter(|s| !s.is_empty())
        .zip(ops)
        .map(|(values, op)| match op {
            Op::ADD => values.iter().fold(0, |acc, i| acc + *i),
            Op::MUL => values.iter().fold(1, |acc, i| acc * *i),
        });
    println!("Final result: {}", results.sum::<i64>());
}
