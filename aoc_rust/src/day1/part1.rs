pub fn run(input: String) {
    let res = input
        .lines()
        .map(|s| (i64::from(&s[0..1] != "L") * 2 - 1) * s[1..].parse::<i64>().unwrap())
        .scan(50, |acc, x| {
            *acc = (*acc + x) % 100 + 100 % 100;
            Some(i64::from(0.eq(acc)))
        })
        .sum::<i64>();
    println!("Result for the day is: {}", res);
}
