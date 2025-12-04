mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let mut args = std::env::args().skip(1);
    let day = args.next().unwrap();
    let part = args.next().unwrap();
    let input = std::fs::read_to_string(args.next().unwrap()).unwrap();
    let days = [
        [day1::part1, day1::part2],
        [day2::part1, day2::part2],
        [day3::part1, day3::part2],
        [day4::part1, day4::part2],
        // [day5::part1, day5::part2],
    ];
    let day = match day
        .trim()
        .replace(" ", "")
        .replace("_", "")
        .to_lowercase()
        .as_str()
        .trim_start_matches("day")
        .parse::<usize>()
    {
        Ok(i) if i != 0 => {
            let d = days.get(i - 1).expect(&format!(
                "failed to find the given {i} day's implementation"
            ));
            print!("Rust -<>- Day {i} ");
            d
        }
        _ => {
            panic!("only [day1, day_1, 'day 1', Day1, etc..] formats supported for now!")
        }
    };
    let f = match part
        .trim()
        .replace(" ", "")
        .replace("_", "")
        .to_lowercase()
        .as_str()
    {
        "part1" => {
            println!("Part 1 called!");
            day[0]
        }
        "part2" => {
            println!("Part 2 called!");
            day[1]
        }
        _ => {
            panic!("[part1, part2] are the only valid options for selecting which part to run")
        }
    };
    // calling the day's implementation
    f(input)
}
