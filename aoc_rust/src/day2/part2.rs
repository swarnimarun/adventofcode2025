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
    // is repeating substring
    // try to window sizes b/w 1 .. src.len() / 2
    'outer: for window_size in 1..=(src.len() / 2) {
        if src.len() % window_size != 0 {
            continue;
        }
        let slice = &src[..window_size];
        let mut i = window_size;
        while i <= (src.len() - window_size) {
            if !slice.eq(&src[i..(i + window_size)]) {
                continue 'outer;
            }
            i += window_size;
        }
        return true;
    }
    false
}
