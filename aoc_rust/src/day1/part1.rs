pub fn run(input: String) {
    let mut acc = 50;
    let res = input
        .lines()
        .map(|s| s[1..].parse::<i64>().unwrap() * if s.as_bytes()[0] == b'L' { -1 } else { 1 })
        .filter(|res| {
            acc = ((acc + res) % 100 + 100) % 100;
            acc == 0
        })
        .count();
    println!("Result for the day is: {}", res);
}
