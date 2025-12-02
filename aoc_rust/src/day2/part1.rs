pub fn run(input: String) {
    let res = input
        .trim()
        .split(',')
        .filter_map(|s| s.split_once('-'))
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .flat_map(|(start, end)| (start..=end).into_iter().filter(|r| is_invalid(*r)))
        .sum::<i64>();
    println!("Result for the day is: {}", res);
}

fn is_invalid(x: i64) -> bool {
    let src = format!("{x}");
    if src.len() % 2 == 0 {
        let r = src.len() / 2;
        for i in 0..r {
            if src.as_bytes()[i] != src.as_bytes()[r + i] {
                return false;
            }
        }
        true
    } else {
        false
    }
}
