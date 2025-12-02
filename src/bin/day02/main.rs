use aoc_2025::harness::run_timed;
use aoc_2025::util::{gcd, triangle};
use std::cmp::min;

const INPUT_A: &'static str = include_str!("../../../resources/day02/input_a.txt");

fn main() {
    run_timed(solve_a);
    run_timed(solve_b);
}

fn solve_b() -> u64 {
    parse_range_list(INPUT_A)
        .map(|(lo, hi)| sum_iterwords_in_range(lo, hi))
        .sum()
}

fn solve_a() -> u64 {
    parse_range_list(INPUT_A)
        .map(|(lo, hi)| sum_copywords_in_range(lo, hi))
        .sum()
}

fn sum_iterwords_in_range(low: u64, high: u64) -> u64 {
    let mut current_low = low;
    let mut current_high = min(high, next_power_of_ten(current_low) - 1);
    let mut sum = 0;

    loop {
        let num_digits = current_low.ilog10() + 1;
        let mut divisors = vec![];
        for block_size in (1..=(num_digits / 2)).rev() {
            if num_digits % block_size != 0 {
                continue;
            }
            if divisors.iter().any(|other_blk| other_blk % block_size == 0) {
                continue;
            }

            sum += sum_bs_iterwords_in_range_fixed(current_low, current_high, block_size);

            // quick and dirty hack for input sizes
            for &other in &divisors {
                sum -= sum_bs_iterwords_in_range_fixed(
                    current_low,
                    current_high,
                    gcd(block_size, other),
                )
            }
            divisors.push(block_size);
        }

        if current_high == high {
            break;
        }
        current_low = current_high + 1;
        current_high = min(high, next_power_of_ten(current_low) - 1);
    }

    sum
}

fn sum_bs_iterwords_in_range_fixed(low: u64, high: u64, block_size: u32) -> u64 {
    let num_digits = low.ilog10() + 1;
    if num_digits % block_size != 0 {
        return 0;
    }

    let coeff = get_coeff(num_digits, block_size);

    let mut start_prefix = get_nth_infix(low, block_size, 0);
    if start_prefix * coeff < low {
        start_prefix += 1;
    }
    let mut end_prefix = get_nth_infix(high, block_size, 0);
    if end_prefix * coeff > high {
        end_prefix -= 1;
    }

    coeff * (triangle(end_prefix) - triangle(start_prefix - 1))
}

fn get_nth_infix(val: u64, block_size: u32, n: u32) -> u64 {
    let digits = val.ilog10() + 1;
    let blocks = digits / block_size;

    let cut_front = val % (10u64.pow(block_size * (blocks - n)));
    let cut_back = cut_front / (10u64.pow(block_size * (blocks - n - 1)));

    cut_back
}

fn get_coeff(digits: u32, block_size: u32) -> u64 {
    let mut sum = 0;
    let mut ix = 0;
    while ix < digits {
        sum += 10u64.pow(ix);
        ix += block_size;
    }

    sum
}

fn sum_copywords_in_range(low: u64, high: u64) -> u64 {
    let mut current_low = low;
    let mut current_high = min(high, next_power_of_ten(current_low) - 1);
    let mut sum = 0;

    loop {
        if current_low.ilog10() % 2 != 0 {
            let mut start_prefix = get_prefix(current_low);
            let start_suffix = get_suffix(current_low);
            if start_prefix < start_suffix {
                start_prefix += 1;
            }

            let mut end_prefix = get_prefix(current_high);
            let end_suffix = get_suffix(current_high);
            if end_prefix > end_suffix {
                end_prefix -= 1;
            }

            let coeff = 10u64.pow((current_low.ilog10() + 1) / 2) + 1;
            sum += coeff as u64 * (triangle(end_prefix) - triangle(start_prefix - 1));
        }

        if current_high == high {
            break;
        }
        current_low = current_high + 1;
        current_high = min(high, next_power_of_ten(current_low) - 1);
    }

    sum
}

fn get_prefix(val: u64) -> u64 {
    let digits = (val.ilog10() + 1) / 2;
    val / 10u64.pow(digits)
}

fn get_suffix(val: u64) -> u64 {
    let digits = (val.ilog10() + 1) / 2;
    val % 10u64.pow(digits)
}

fn next_power_of_ten(val: u64) -> u64 {
    let log = val.ilog10();

    10u64.pow(log + 1)
}

fn parse_range(range_str: &str) -> (u64, u64) {
    let (low, high) = range_str.split_once('-').expect("Unable to split range");

    (
        low.parse().expect("Unable to parse low"),
        high.parse().expect("Unable to parse high"),
    )
}

fn parse_range_list<'a>(line: &'a str) -> impl Iterator<Item = (u64, u64)> {
    line.split(',').map(parse_range)
}

#[test]
fn test_parse_range() {
    assert_eq!(parse_range("11-22"), (11, 22));
    assert_eq!(
        parse_range("1188511880-1188511890"),
        (1188511880, 1188511890)
    );
}

#[test]
fn test_parse_range_list() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890";
    let mut result = parse_range_list(input);
    assert_eq!(result.next(), Some((11, 22)));
    assert_eq!(result.next(), Some((95, 115)));
    assert_eq!(result.next(), Some((998, 1012)));
    assert_eq!(result.next(), Some((1188511880, 1188511890)));
}

#[test]
fn test_next_power_of_ten() {
    assert_eq!(next_power_of_ten(9), 10);
    assert_eq!(next_power_of_ten(10), 100);
}

#[test]
fn test_get_prefix_and_suffix() {
    assert_eq!(get_prefix(11), 1);
    assert_eq!(get_prefix(2345), 23);
    assert_eq!(get_prefix(91291_29123), 91291);

    assert_eq!(get_suffix(11), 1);
    assert_eq!(get_suffix(2345), 45);
    assert_eq!(get_suffix(91291_29123), 29123);
}

#[test]
fn test_sum_copywords_in_range() {
    assert_eq!(sum_copywords_in_range(11, 11), 11);
    assert_eq!(sum_copywords_in_range(11, 22), 33);
    assert_eq!(sum_copywords_in_range(1, 100), 495);
    assert_eq!(sum_copywords_in_range(80, 1211), 2308);
}

#[test]
fn test_get_nth_infix() {
    assert_eq!(get_nth_infix(101112, 2, 0), 10);
    assert_eq!(get_nth_infix(101112, 2, 1), 11);
    assert_eq!(get_nth_infix(101112, 2, 2), 12);

    assert_eq!(get_nth_infix(12345, 1, 2), 3);

    assert_eq!(get_nth_infix(1, 1, 0), 1);
}

#[test]
fn test_get_coeff() {
    assert_eq!(get_coeff(2, 1), 11);
    assert_eq!(get_coeff(4, 1), 1111);
    assert_eq!(get_coeff(4, 2), 101);
    assert_eq!(get_coeff(6, 3), 1001);
}

#[test]
fn test_sum_iterwords_in_range() {
    assert_eq!(sum_iterwords_in_range(11, 22), 33);
    assert_eq!(sum_iterwords_in_range(99, 115), 210);
    assert_eq!(sum_iterwords_in_range(998, 1012), 999 + 1010);
    assert_eq!(sum_iterwords_in_range(1188511880, 1188511890), 1188511885);
    assert_eq!(sum_iterwords_in_range(222220, 222224), 222222);
    assert_eq!(sum_iterwords_in_range(1698522, 1698528), 0);
    assert_eq!(sum_iterwords_in_range(446443, 446449), 446446);
    assert_eq!(sum_iterwords_in_range(38593856, 38593862), 38593859);
    assert_eq!(sum_iterwords_in_range(565653, 565659), 565656);
    assert_eq!(sum_iterwords_in_range(824824821, 824824827), 824824824);
    assert_eq!(sum_iterwords_in_range(2121212118,2121212124), 2121212121);
    assert_eq!(
        sum_iterwords_in_range(11111_11111_11, 11111_11111_11),
        11111_11111_11
    );
}

#[test]
fn test_solve_b_example() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let result: u64 = parse_range_list(input)
        .map(|(lo, hi)| sum_iterwords_in_range(lo, hi))
        .sum();
    assert_eq!(result, 4174379265);
}
