use std::ops::RangeInclusive;

pub fn run(input: String) {
    let (ranges, _) = input.trim().split_once("\n\n").expect("no empty line");
    let mut ranges = ranges.lines().map(parse_range).collect::<Vec<_>>();
    ranges.sort_by_key(|r| *r.start());
    let fresh_items = ranges
        .into_iter()
        .fold(vec![], |mut visited, range| {
            if let Some(visited_range) = visited.last_mut() {
                if merge_range(visited_range, &range) {
                    return visited;
                }
            }
            visited.push(range);
            visited
        })
        .into_iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<i64>();
    println!("Day part answer: {fresh_items}");
}

fn merge_range(a: &mut RangeInclusive<i64>, b: &RangeInclusive<i64>) -> bool {
    if a.start() <= b.end() && a.end() >= b.start() {
        // always make a bigger and b smaller
        if b.end() >= a.end() {
            if a.start() >= b.start() {
                *a = b.clone();
            } else {
                *a = *a.start()..=*b.end();
            }
        } else {
            if a.start() >= b.start() {
                *a = *b.start()..=*a.end();
            }
        }
        true
    } else {
        // no overlap
        false
    }
}

fn parse_range(s: &str) -> RangeInclusive<i64> {
    let (lhs, rhs) = s.split_once('-').expect("Range not in correct format");
    lhs.parse().unwrap()..=rhs.parse().unwrap()
}
