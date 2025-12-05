use std::ops::RangeInclusive;

pub fn run(input: String) {
    let (ranges, ingredients) = input.trim().split_once("\n\n").expect("no empty line");
    let ranges = ranges.lines().map(parse_range).collect::<Vec<_>>();
    let fresh_items = ingredients
        .lines()
        .map(str::parse::<i64>)
        .filter_map(Result::ok)
        .filter(fresh_items_using(&ranges))
        .count();
    println!("Day part answer: {fresh_items}");
}

fn fresh_items_using<'a>(ranges: &'a [RangeInclusive<i64>]) -> impl Fn(&i64) -> bool + 'a {
    |ingredient| ranges.iter().any(|range| range.contains(&ingredient))
}

fn parse_range(s: &str) -> RangeInclusive<i64> {
    let (lhs, rhs) = s.split_once('-').expect("Range not in correct format");
    lhs.parse().unwrap()..=rhs.parse().unwrap()
}
