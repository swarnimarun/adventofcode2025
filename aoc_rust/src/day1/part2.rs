pub fn run(input: String) {
    let res = input
        .lines()
        .map(|s| (i64::from(&s[0..1] != "L") * 2 - 1) * s[1..].parse::<i64>().unwrap())
        .scan(50, |acc, x| {
            let sum = *acc + x;
            let rotations = sum.abs() / 100 + i64::from(*acc != 0 && sum <= 0);
            *acc = (sum % 100 + 100) % 100;
            Some(rotations)
        })
        .sum::<i64>();
    println!("Result for the day is: {}", res);
}
