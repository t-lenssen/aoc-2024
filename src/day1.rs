use std::mem::{self, MaybeUninit};

pub fn fast_inp(input: &str) -> (i32, i32) {
    let inp_bytes = input.as_bytes();
    let a = (inp_bytes[0] - b'0') as i32 * 10000 +
                (inp_bytes[1] - b'0') as i32 * 1000 +
                (inp_bytes[2] - b'0') as i32 * 100 +
                (inp_bytes[3] - b'0') as i32 * 10 +
                (inp_bytes[4] - b'0') as i32 * 1;
    let b = (inp_bytes[8 + 0] - b'0') as i32 * 10000 +
                (inp_bytes[8 + 1] - b'0') as i32 * 1000 +
                (inp_bytes[8 + 2] - b'0') as i32 * 100 +
                (inp_bytes[8 + 3] - b'0') as i32 * 10 +
                (inp_bytes[8 + 4] - b'0') as i32 * 1;
    (a, b)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut a: [MaybeUninit<i32>; 1000] = [const { MaybeUninit::uninit() }; 1000];
    let mut b: [MaybeUninit<i32>; 1000] = [const { MaybeUninit::uninit() }; 1000];

    for (i, l) in input.lines().enumerate() {
        let (a_i, b_i) = fast_inp(l);
        a[i].write(a_i);
        b[i].write(b_i);
    }

    
    let mut sorted_a = unsafe { mem::transmute::<_, [i32; 1000]>(a) }; 
    let mut sorted_b = unsafe { mem::transmute::<_, [i32; 1000]>(b) }; 

    sorted_a.sort_unstable();
    sorted_b.sort_unstable();

    sorted_a.iter().zip(sorted_b.iter()).map(|(a_i, b_i)| (a_i - b_i).abs()).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut a: [MaybeUninit<i32>; 1000] = [const { MaybeUninit::uninit() }; 1000];

    let mut b_map: [i32; 100000] = [0; 100000];
    for (i, l) in input.lines().enumerate() {
        let (a_i, b_i) = fast_inp(l);
        a[i].write(a_i);
        b_map[b_i as usize] += 1;
    }
    
    unsafe {
        a.iter().map(|a_i| a_i.assume_init() * b_map[a_i.assume_init() as usize]).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn sample1() {
        assert_eq!(part1("3   4
                        4   3
                        2   5
                        1   3
                        3   9
                        3   3"), 11);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2("3   4
                        4   3
                        2   5
                        1   3
                        3   9
                        3   3"), 31);
    }
}
