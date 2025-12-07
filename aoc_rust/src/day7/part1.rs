use std::collections::HashSet;

pub fn run(input: String) {
    let grid: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|s| s.bytes().collect::<Vec<_>>())
        .collect();
    // pick S
    let s = grid[0]
        .iter()
        .position(|s| *s == b'S')
        .expect("S not found in the first line");
    // start beam, then follow beam & handle splits
    let mut split_count = 0;
    (1..grid.len())
        .scan(HashSet::from([s]), |acc, i| {
            *acc = acc
                .iter()
                .flat_map(|&j| {
                    if grid[i][j] == b'^' {
                        split_count += 1;
                        vec![j - 1, j + 1]
                    } else {
                        vec![j]
                    }
                    .into_iter()
                })
                .collect();
            Some(())
        })
        .last();
    println!("Result: {split_count}");
}
