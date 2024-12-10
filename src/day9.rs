use std::u32;
use std::{cmp::Reverse, collections::BinaryHeap};

const LEN: usize = 19999;

fn get_checksum(len: i64, pos: i64, id: i64) -> i64 {
    id * (len * (len + 1) / 2 + len * (pos - 1))
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
    let inp = input.as_bytes();
    let mut pos: i64 = 0;
    let mut i: usize = 0;
    let mut j: usize = LEN - 1;
    let mut s: i64 = 0;

    let mut inp_j = inp[j] - b'0';

    loop {
        s += get_checksum((inp[i] - b'0') as i64, pos, (i as i64) >> 1);
        pos += (inp[i] - b'0') as i64;

        let mut gap = inp[i + 1] - b'0';
        while gap > 0 {
            if j == i + 2 {
                s += get_checksum(inp_j as i64, pos, (j as i64) >> 1);
                return s;
            }
            if inp_j <= gap {
                s += get_checksum(inp_j as i64, pos, (j as i64) >> 1);
                pos += inp_j as i64;
                gap -= inp_j;
                j -= 2;
                inp_j = inp[j]  - b'0';
            } else {
                s += get_checksum(gap as i64, pos, (j as i64) >> 1);
                pos += gap as i64;
                inp_j -= gap;
                break;
            }
        }
        i += 2;
    }
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
    let inp = input.as_bytes();
    let mut files: Vec<(u8, u32, u16)> = Vec::new();
    let mut gaps: [BinaryHeap<Reverse<u32>>; 10] = [const { BinaryHeap::new() }; 10];
    let mut gap_files: Vec<(u8, u32, u16)> = Vec::new();
    let mut pos: u32 = 0;

    let mut i = 0;
    loop {
        files.push((inp[i] - b'0', pos, (i >> 1) as u16));
        pos += (inp[i] - b'0') as u32;
        if i + 1 >= LEN { break; }
        gaps[(inp[i + 1] - b'0') as usize].push(Reverse(pos));
        pos += (inp[i + 1] - b'0') as u32;
        i += 2;
    }

    for (file_len, file_pos, file_id) in files.iter_mut().rev() {
        let mut earliest: u32 = u32::MAX;
        let mut earliest_len: u8 = 0xFF;
        for k in (*file_len as usize)..10 {
            if !gaps[k].is_empty() && gaps[k].peek().unwrap().0 < earliest {
                earliest = gaps[k].peek().unwrap().0;
                earliest_len = k as u8;
            }
        }
        
        if earliest > *file_pos {
            if *file_len == 1 {break}
            continue
        }

        let gap_pos = gaps[earliest_len as usize].pop().unwrap().0;
        gap_files.push((*file_len, gap_pos, *file_id));

        if earliest_len - *file_len > 0 {
            gaps[(earliest_len - *file_len) as usize].push(Reverse(gap_pos + *file_len as u32));
        }

        *file_id = 0;
    }

    let mut s: i64 = 0;

    for (file_len, file_pos, file_id) in files {
        s += get_checksum(file_len as i64, file_pos as i64, file_id as i64);
    }
    for (file_len, file_pos, file_id) in gap_files {
        s += get_checksum(file_len as i64, file_pos as i64, file_id as i64);
    }

    s
}