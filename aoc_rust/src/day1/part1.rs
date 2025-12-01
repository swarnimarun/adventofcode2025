pub fn run(input: String) {
    let mut result = 0;
    _ = input
        .lines()
        .map(|s| {
            if s.as_bytes()[0] == b'L' {
                s[1..].parse::<i64>().unwrap() * -1
            } else {
                s[1..].parse::<i64>().unwrap()
            }
        })
        .fold(50i64, |acc, res| {
            let r = ((acc + res) % 100 + 100) % 100;
            if r == 0 {
                result += 1;
            }
            r
        });
    println!("Result for the day is: {}", result);
}
