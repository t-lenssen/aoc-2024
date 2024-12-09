const N: usize = 1000;

//Checks order of first 2 numbers, returns 0 if invalid, else last number with 0x80 set if accending
fn check_first_order(inp: &mut &[u8]) -> u8 {
    let a: u8;
    let b: u8;

    if inp[3] == b' ' {
        (a, b) = (inp[0] - b'0',
        inp[2] - b'0');
        *inp = &inp[4..];
    } else if inp[5] == b' ' {
        (a, b) = ((inp[0] - b'0') * 10 + inp[1] - b'0',
        (inp[3] - b'0') * 10 + inp[4] -  b'0');
        *inp = &inp[6..];
    } else  if inp[2] == b' ' {
        (a, b) = ((inp[0] - b'0') * 10 + inp[1] - b'0',
        inp[3] - b'0');
        *inp = &inp[5..];
    } else {
        (a, b) = (inp[0] - b'0',
        (inp[2] - b'0') * 10 + inp[3] - b'0');
        *inp = &inp[5..];
    }
    
    if b > a { 
        if b - a == 0 || b - a > 3 { 0 }
        else { b | 0x80 }
    } else {
        if a - b == 0 || a - b > 3 { 0 }
        else { b }
    }
}

//Checks order of middle 2 numbers, returns 0 if invalid, else last number with 0x80 set if accending
fn check_middle_order(inp: &mut &[u8], prev: u8) -> u8 {
    let a: u8;
    let b: u8;

    if inp[3] == b' ' {
        (a, b) = (inp[0] - b'0',
        inp[2] - b'0');
        *inp = &inp[4..];
    } else if inp[5] == b' ' {
        (a, b) = ((inp[0] - b'0') * 10 + inp[1] - b'0',
        (inp[3] - b'0') * 10 + inp[4] -  b'0');
        *inp = &inp[6..];
    } else  if inp[2] == b' ' {
        (a, b) = ((inp[0] - b'0') * 10 + inp[1] - b'0',
        inp[3] - b'0');
        *inp = &inp[5..];
    } else {
        (a, b) = (inp[0] - b'0',
        (inp[2] - b'0') * 10 + inp[3] - b'0');
        *inp = &inp[5..];
    }
    
    if prev & 0x80 != 0 { 
        let prev = prev & 0x7f;
        if a - prev == 0 || a - prev > 3 || b - a == 0 || b - a > 3 { 0 }
        else { b | 0x80 }
    } else {
        if prev - a == 0 || prev - a > 3 || a - b == 0 || a - b > 3 { 0 }
        else { b }
    }
}

//Checks order of potential last number, returns 0 if invalid, else last number with 0x80 set if accending
//If end and valid returns 0xff and invalid return 0xfe
fn check_end(inp: &mut &[u8], prev: u8) -> u8 {
    let a: u8;
    let is_end: bool;
    
    if inp[2] < b'0' {
        (a, is_end) = ((inp[0] - b'0') * 10 + inp[1] - b'0', 
        inp[2] == b'\n');
        *inp = &inp[3..];
    } else {
        (a, is_end) =  (inp[0] - b'0',
        inp[1] == b'\n');
        *inp = &inp[2..];
    };
    
    if prev & 0x80 != 0 {
        let prev = prev & 0x7f;
        if a - prev == 0 || a - prev > 3 { 
            if is_end { 0xfe }
            else { 0 }
        }
        else if is_end { 0xff }
        else { a | 0x80 }
    } else {
        if prev - a == 0 || prev - a > 3 { 
            if is_end { 0xfe }
            else { 0 }
        }
        else if is_end { 0xff }
        else { a }
    }
}

fn to_next_line(inp: &mut &[u8]) {
    while inp[0] != b'\n' {
        *inp = &inp[1..]
    }
    *inp = &inp[1..]
}

fn check_line(inp: &mut &[u8]) -> i32 {
    let mut out = check_first_order(inp);
        if out == 0 {
            to_next_line(inp);
            return 0;
        }
        out = check_middle_order(inp, out);
        if out == 0 {
            to_next_line(inp);
            return 0;
        }
        loop {
            out = check_end(inp, out);
            if out == 0 {
                to_next_line(inp);
                return 0;
            }
            if out == 0xff {
                return 1;
            }
            if out == 0xfe {
                return 0;
            }
        }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut inp: &[u8] = input.as_bytes();

    let mut cnt: i32 = 0;

    for i in 0..N {
        if i == N - 1 {
            let mut last_line: &[u8] = &[inp, "\n0".as_bytes()].concat();
            cnt += check_line(&mut last_line);
            break;
        }
        cnt += check_line(&mut inp);
    }

    cnt
}

#[aoc(day2, part2)]
pub fn part2(_input: &str) -> i32 {
    -1
}