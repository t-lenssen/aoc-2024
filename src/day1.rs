use std::mem::{self, MaybeUninit};

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut a: [MaybeUninit<i32>; 1000] = [const { MaybeUninit::uninit() }; 1000];
    let mut b: [MaybeUninit<i32>; 1000] = [const { MaybeUninit::uninit() }; 1000];

    for (i, l) in input.lines().enumerate() {
        a[i].write(l[0..5].parse::<i32>().unwrap());
        b[i].write(l[8..13].parse::<i32>().unwrap());
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
    let mut b: [MaybeUninit<i32>; 1000] = [const { MaybeUninit::uninit() }; 1000];

    for (i, l) in input.lines().enumerate() {
        a[i].write(l[0..5].parse::<i32>().unwrap());
        b[i].write(l[8..13].parse::<i32>().unwrap());
    }

    
    let mut sorted_a = unsafe { mem::transmute::<_, [i32; 1000]>(a) }; 
    let mut sorted_b = unsafe { mem::transmute::<_, [i32; 1000]>(b) }; 

    sorted_a.sort_unstable();
    sorted_b.sort_unstable();

    let mut score = 0;
    let mut b_idx = 0;

    let mut b_i_cnt = 0;
    let mut prev_a_i = -1;
    let mut a_i_cnt = 1;

    'outer: for a_i in sorted_a {
        if a_i == prev_a_i {
            a_i_cnt += 1;
        } else {
            score += prev_a_i * a_i_cnt * b_i_cnt;
            b_i_cnt = 0;
            a_i_cnt = 1;
        }
        prev_a_i = a_i;
        while a_i > sorted_b[b_idx] {
            b_idx += 1;
            
            if b_idx == 1000 {
                break 'outer;
            }
        }
        while a_i == sorted_b[b_idx] {
            b_i_cnt += 1;
            b_idx += 1;
            if b_idx == 1000 {
                break 'outer;
            }
        }
    }

    score += prev_a_i * a_i_cnt * b_i_cnt;

    score
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
}
