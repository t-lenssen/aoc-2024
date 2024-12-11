const LEN: usize = 59;
const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn search(grid: &[u8], seen: &mut [bool], h: u8, i: usize, j: usize) -> i32 {
    if seen[j + LEN * i] { return 0; }
    seen[j + LEN * i] = true;
    if h == b'9' {return 1; }

    let mut s = 0;
    for dir in DIRS {
        let x = i as isize + dir.0;
        let y = j as isize + dir.1;
        if x < 0 || x >= LEN as isize || y < 0 || y >= LEN as isize { continue; }
        if grid[y as usize + (LEN + 1) * x as usize] == h + 1 {
            s += search(grid, seen, h + 1, x as usize, y as usize);
        }
    }
    s
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let mut s = 0;
    let grid = input.as_bytes();

    for i in 0..LEN {
        for j in 0..LEN {
            if grid[j + (LEN + 1) * i] == b'0' {
                let mut seen: [bool; LEN * LEN] = [false; LEN * LEN];
                s += search(input.as_bytes(), &mut seen, b'0', i, j);
            }
        }
    }
    
    s
}

fn search_p2(grid: &[u8], seen: &mut [i32], h: u8, i: usize, j: usize) -> i32 {
    if seen[j + LEN * i] != -1 { return seen[j + LEN * i]; }
    if h == b'9' {return 1;}

    let mut s = 0;
    for dir in DIRS {
        let x = i as isize + dir.0;
        let y = j as isize + dir.1;
        if x < 0 || x >= LEN as isize || y < 0 || y >= LEN as isize { continue; }
        if grid[y as usize + (LEN + 1) * x as usize] == h + 1 {
            s += search_p2(grid, seen, h + 1, x as usize, y as usize);
        }
    }
    seen[j + LEN * i] = s;
    s
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> i32 {
    let mut s = 0;
    let grid = input.as_bytes();

    for i in 0..LEN {
        for j in 0..LEN {
            if grid[j + (LEN + 1) * i] == b'0' {
                let mut seen: [i32; LEN * LEN] = [-1; LEN * LEN];
                s += search_p2(input.as_bytes(), &mut seen, b'0', i, j);
            }
        }
    }
    
    s
}