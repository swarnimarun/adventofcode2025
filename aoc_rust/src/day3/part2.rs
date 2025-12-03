pub fn run(input: String) {
    let res = input
        .trim()
        .lines()
        .map(|line| {
            // find the largest number in the line except the last element
            if line.len() <= 13 {
                panic!("empty of single item line not supported");
            }
            let mut start_index = 0;
            let mut values = vec![];
            for i in 0..12 {
                let (mi, max) = line
                    .as_bytes()
                    .iter()
                    .enumerate()
                    .skip(start_index)
                    .rev()
                    .skip(11 - i)
                    .max_by(|(_, b), (_, d)| b.cmp(d))
                    .unwrap();
                values.push(max);
                start_index = mi + 1;
            }
            values
                .into_iter()
                .fold(String::new(), |mut acc, b| {
                    acc.push(*b as char);
                    acc
                })
                .parse::<usize>()
                .unwrap()
        })
        .sum::<usize>();
    println!("Result for the part of the day is: {res}");
}
