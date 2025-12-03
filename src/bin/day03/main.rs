use std::cmp::max;
use std::thread::current;
use aoc_2025::harness::run_timed;

const INPUT_A: &'static str = include_str!("../../../resources/day03/input_a.txt");

fn main() {
    run_timed(solve_a);
    run_timed(solve_b);
}

fn solve_b() -> u64 {
    INPUT_A.lines()
        .map(parse_bank)
        .map(Iterator::collect::<Vec<usize>>)
        .map(|v| maximize_multi_bank(&v, 12))
        .sum()
}

fn solve_a() -> u64 {
    INPUT_A.lines()
        .map(parse_bank)
        .map(Iterator::collect::<Vec<usize>>)
        .map(|v| maximize_bank(&v))
        .sum()
}

fn maximize_multi_bank(bank: &[usize], digits: usize) -> u64 {
    let mut acc: Vec<usize> = vec![0; digits];
    let mut digit_map = [0usize; 10];

    for &digit in &bank[..=(bank.len() - digits)] {
        let acc_ix = digit_map[digit];
        if acc_ix == acc.len() { continue; }

        acc[acc_ix] = digit;
        flush_left(&mut digit_map, digit + 1, acc_ix + 1);
    }

    for offset in 1..digits {
        let bank_ix = bank.len() - digits + offset;
        let digit = bank[bank_ix];

        let acc_ix = max(offset, digit_map[digit]);
        if acc_ix == acc.len() { continue; }

        acc[acc_ix] = digit;
        flush_left(&mut digit_map, digit + 1, acc_ix + 1);
    }

    buf_to_int(&acc)
}

fn flush_left(buf: &mut [usize], end: usize, value: usize) {
    for i in 0..end { buf[i] = value; }
}

fn buf_to_int(buf: &Vec<usize>) -> u64 {
    buf.iter().fold(0, |acc, digit| acc * 10 + *digit as u64)
}

fn maximize_bank(bank: &[usize]) -> u64 {
    let mut tens = 0;
    let mut ones = 0;

    for i in 0..bank.len()-1 {
        if bank[i] > tens {
            tens = bank[i];
            ones = 0;
        } else if bank[i] > ones {
            ones = bank[i];
        }
    }

    if bank[bank.len() - 1] > ones {
        ones = bank[bank.len() - 1];
    }

    (tens * 10 + ones) as u64
}

fn parse_bank(line: &str) -> impl Iterator<Item=usize> {
    line.chars()
        .map(|c| c.to_digit(10).expect("Not a digit") as usize)
}

#[test]
fn test_parse_bank() {
    let input = "123";
    let mut iter = parse_bank(input);
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
}

#[test]
fn test_maximize_bank() {
    assert_eq!(maximize_bank(&[9,8,7,6,5,4,3,2,1,1,1,1,1,1,1]), 98);
    assert_eq!(maximize_bank(&[8,1,1,1,1,1,1,1,1,1,1,1,1,1,9]), 89);
    assert_eq!(maximize_bank(&[2,3,4,2,3,4,2,3,4,2,3,4,2,7,8]), 78);
    assert_eq!(maximize_bank(&[8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]), 92);
}

#[test]
fn test_maximize_multi_bank() {
    assert_eq!(maximize_multi_bank(&[9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 12), 987654321111);
    assert_eq!(maximize_multi_bank(&[8,1,1,1,1,1,1,1,1,1,1,1,1,1,9], 12), 811111111119);
    assert_eq!(maximize_multi_bank(&[2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], 12), 434234234278);
    assert_eq!(maximize_multi_bank(&[8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 12), 888911112111);
}