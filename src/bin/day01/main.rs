const INITIAL_POS: isize = 50;
const MAX_POS: isize = 100;

const INPUT_A: &'static str = include_str!("../../../resources/day01/input_a.txt");

use aoc_2025::harness::run_timed;
use aoc_2025::util;

fn main() {
    run_timed(solve_a);
    run_timed(solve_b);
}

fn solve_a() -> usize {
    get_intermediate_position(INPUT_A)
        .filter(|&pos| pos == 0)
        .count()
}

fn solve_b() -> usize {
    get_zero_passes(INPUT_A)
        .sum()
}

fn get_zero_passes(input: &str) -> impl Iterator<Item=usize> {
    input.lines()
        .map(line_to_int)
        .scan(INITIAL_POS, |pos, shift| {
            let new_value = *pos + shift;
            let mut passes = (new_value / MAX_POS).abs();
            if new_value <= 0 && *pos > 0 {
                passes += 1
            }
            
            *pos = util::posmod(new_value, MAX_POS);
            Some(passes as usize)
        })
}

fn get_intermediate_position(input: &str) -> impl Iterator<Item=isize> {
    input.lines()
        .map(line_to_int)
        .scan(INITIAL_POS, |pos, shift| {
            *pos = util::posmod(*pos + shift, MAX_POS);
            Some(*pos)
        })
}

fn line_to_int(line: &str) -> isize {
    let direction = if line.starts_with("L") { -1 } else { 1 };
    let distance: isize = line
        .strip_prefix(&['L', 'R']).expect("Failed to strip direction indicator")
        .parse().expect("Failed to parse distance to int");
    
    direction * distance
}

#[test]
fn test_line_to_int() {
    assert_eq!(line_to_int("R8"), 8);
    assert_eq!(line_to_int("L19"), -19);
}

#[test]
fn test_intermediate_positions() {
    let input = "R10\nR20\nR40\nL50\n";
    let mut iter = get_intermediate_position(input);
    assert_eq!(iter.next(), Some(60));
    assert_eq!(iter.next(), Some(80));
    assert_eq!(iter.next(), Some(20));
    assert_eq!(iter.next(), Some(70));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_zero_passes() {
    let input = "L50\nR1050\nR50\nL50\nL50\nL100";
    let mut iter = get_zero_passes(input);
    assert_eq!(iter.next(), Some(1)); // 50 + 50 = 100
    assert_eq!(iter.next(), Some(10)); // 0 + 1050 = 1050
    assert_eq!(iter.next(), Some(1)); // 50 - 50 = 0
    assert_eq!(iter.next(), Some(0)); // 0 - 50 = -50
    assert_eq!(iter.next(), Some(1)); // 50 - 50 = 0
    assert_eq!(iter.next(), Some(1)); // 0 - 100 = -100
    assert_eq!(iter.next(), None)
}