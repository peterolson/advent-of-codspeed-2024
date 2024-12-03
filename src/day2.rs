#[aoc(day2, part1, Chars)]
pub fn part1(input: &str) -> i16 {
    let bytes = input.as_bytes();
    let mut sum = 0;

    let mut current_number = 0;
    let mut prev_number = 0;
    let mut is_safe = true;
    let mut increasing = 0;

    for &byte in bytes {
        match byte {
            b'0'..=b'9' => {
                current_number = current_number * 10 + (byte - b'0') as i16;
            }
            b' ' | b'\n' => {
                if is_safe && prev_number != 0 {
                    if current_number == prev_number || current_number.abs_diff(prev_number) > 3 {
                        is_safe = false;
                    }
                    if increasing == 0 {
                        increasing = current_number - prev_number;
                    } else {
                        if increasing > 0 && current_number < prev_number {
                            is_safe = false;
                        } else if increasing < 0 && current_number > prev_number {
                            is_safe = false;
                        }
                    }
                }
                prev_number = current_number;
                current_number = 0;
                if byte == b'\n' {
                    if is_safe {
                        sum += 1;
                    }
                    is_safe = true;
                    prev_number = 0;
                    increasing = 0;
                }
            }
            _ => {}
        }
    }
    if current_number == prev_number || current_number.abs_diff(prev_number) > 3 {
        is_safe = false;
    } else if increasing > 0 && current_number < prev_number {
        is_safe = false;
    } else if increasing < 0 && current_number > prev_number {
        is_safe = false;
    }
    if is_safe {
        sum += 1;
    }

    sum
}
