use std::arch::x86_64::_popcnt64;

const GRID_LEN: usize = 50;
const ANTENNAS_N: usize = (b'z' + 1) as usize;

fn init_antennas(input: &[u8], antennas: &mut [Vec<(i32, i32)>; ANTENNAS_N]) {
    for i in 0..GRID_LEN {
        for j in 0..GRID_LEN {
            let c = input[(GRID_LEN + 1) * i + j];
            if c != b'.' {
                antennas[c as usize].push((i as i32, j as i32));
            }
        }
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
    let mut antennas: [Vec<(i32, i32)>; ANTENNAS_N] = [const { Vec::new() }; ANTENNAS_N];
    init_antennas(input.as_bytes(), &mut antennas);

    let mut grid: [u64; GRID_LEN] = [0; GRID_LEN];
    for ant in antennas {
        if ant.len() <= 1 {
            continue;
        }
        for i in 0..ant.len() {
            for j in (i + 1)..ant.len() {
                let x_0 = ant[i].0 - (ant[j].0 - ant[i].0);
                let y_0 = ant[i].1 - (ant[j].1 - ant[i].1);
                if 0 <= x_0  && x_0 < GRID_LEN as i32 && 
                   0 <= y_0  && y_0 < GRID_LEN as i32 {
                    grid[y_0 as usize] |= 1 << x_0;
                }

                let x_1 = ant[j].0 + (ant[j].0 - ant[i].0);
                let y_1 = ant[j].1 + (ant[j].1 - ant[i].1);
                if 0 <= x_1  && x_1 < GRID_LEN as i32 && 
                   0 <= y_1  && y_1 < GRID_LEN as i32 {
                    grid[y_1 as usize] |= 1 << x_1;
                }
            }
        }
    }

    let mut s = 0;
    unsafe {
        for row in grid {
            s += _popcnt64(row as i64);
        }
    }
    s
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
    let mut antennas: [Vec<(i32, i32)>; ANTENNAS_N] = [const { Vec::new() }; ANTENNAS_N];
    init_antennas(input.as_bytes(), &mut antennas);

    let mut grid: [u64; GRID_LEN] = [0; GRID_LEN];
    for ant in antennas {
        if ant.len() <= 1 {
            continue;
        }
        for i in 0..ant.len() {
            for j in (i + 1)..ant.len() {
                grid[ant[i].1 as usize] |= 1 << ant[i].0;
                grid[ant[j].1 as usize] |= 1 << ant[j].0;

                let mut k: i32 = 1;
                loop {
                    let x = ant[i].0 - k * (ant[j].0 - ant[i].0);
                    let y = ant[i].1 - k * (ant[j].1 - ant[i].1);
                    if x < 0 || x >= GRID_LEN as i32 || 
                    y < 0 || y >= GRID_LEN as i32 {
                        break;
                    }
                    grid[y as usize] |= 1 << x;
                    k += 1;
                }

                k = 1;
                loop {
                    let x = ant[j].0 + k * (ant[j].0 - ant[i].0);
                    let y = ant[j].1 + k * (ant[j].1 - ant[i].1);
                    if x < 0 || x >= GRID_LEN as i32 || 
                    y < 0 || y >= GRID_LEN as i32 {
                        break;
                    }
                    grid[y as usize] |= 1 << x;
                    k += 1;
                }
            }
        }
    }

    let mut s = 0;
    unsafe {
        for row in grid {
            s += _popcnt64(row as i64);
        }
    }
    s
}
