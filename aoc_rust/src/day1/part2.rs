pub fn run(input: String) {
    let mut previous = 50;
    let mut rotations = 0;
    input
        .lines()
        .map(|s| s[1..].parse::<i64>().unwrap() * if s.as_bytes()[0] == b'L' { -1 } else { 1 })
        .for_each(|next| {
            let res = previous + next;
            rotations += res.abs() / 100 + i64::from(previous != 0 && res <= 0);
            previous = (res % 100 + 100) % 100;
        });
    println!("Result for the day is: {}", rotations);
}
