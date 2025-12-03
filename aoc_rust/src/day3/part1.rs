pub fn run(input: String) {
    let res = input
        .trim()
        .lines()
        .map(|line| {
            // find the largest number in the line except the last element
            if line.len() <= 1 {
                panic!("empty of single item line not supported");
            }
            let (mi, max) = line
                .as_bytes()
                .iter()
                .enumerate()
                .rev()
                .skip(1)
                .max_by(|(_, b), (_, d)| b.cmp(d))
                .unwrap();
            let m2 = line.as_bytes().iter().skip(mi + 1).max().unwrap();
            format!("{}{}", *max as char, *m2 as char)
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>();
    println!("Result for the part of the day is: {res}");
}
