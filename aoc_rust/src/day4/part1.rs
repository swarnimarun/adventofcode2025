pub fn run(input: String) {
    let grid = input
        .trim()
        .lines()
        .map(|s| {
            s.as_bytes()
                .iter()
                .map(|u| match *u {
                    b'@' => 1,
                    b'.' => 0,
                    _ => {
                        panic!("unsupported value in the input")
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();
    // walk grid, check neighbours
    let count = (0..grid.len())
        .flat_map(|y| (0..grid[y].len()).zip(std::iter::repeat(y)))
        .filter(|(x, y)| count_neighbours(&grid, *x as i32, *y as i32) < 4)
        .count();
    println!("Result of day and part: {count}");
}

fn grid_value(grid: &[Vec<u8>], y: i32, x: i32) -> u8 {
    if x <= -1 || grid[0].len() <= x as _ {
        return 0;
    }
    if y <= -1 || grid.len() <= y as _ {
        return 0;
    }
    grid[y as usize][x as usize]
}

fn count_neighbours(grid: &[Vec<u8>], x: i32, y: i32) -> u8 {
    if grid[y as usize][x as usize] != 1 {
        return 8;
    }
    (-1..=1)
        .flat_map(|i| (-1..=1).zip(std::iter::repeat(i)))
        .filter(|(i, j)| !(*i == 0 && *j == 0))
        .fold(0, |acc, (i, j)| acc + grid_value(&grid, y + j, x + i))
}
