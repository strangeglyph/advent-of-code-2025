use std::ops::{Add, Mul};
use aoc_2025::algebra::algebra::Matrix;
use aoc_2025::harness::run_timed;
use aoc_2025::util::transpose;

const INPUT_A: &'static str = include_str!("../../../resources/day06/input_a.txt");

fn main() {
    run_timed(solve_a);
    run_timed(solve_b)
}

fn solve_a() -> i64 {
    solve_problems_from_inp::<4, 1000>(INPUT_A).iter().sum()
}

fn solve_b() -> i64 {
    let (matrix, ops) = parse_cephalopod_style(INPUT_A, 4);
    solve_problems(matrix.into_iter(), ops).iter().sum()
}

fn solve_problems_from_inp<const PROB_L: usize, const PROB_N: usize>(input: &str) -> Vec<i64> {
    let (matrix, ops) = sheet_problems_and_ops::<PROB_L, PROB_N>(input);
    solve_problems(matrix.rows.into_iter().map(Vec::from), ops)
}

fn solve_problems(problems: impl Iterator<Item=Vec<i64>>, ops: Vec<char>) -> Vec<i64> {
    problems.zip(ops).
        map(|(nums, op)| match op {
            '+' => nums.iter().fold(0, i64::add),
            '*' => nums.iter().fold(1, i64::mul),
            _ => panic!("Invalid op")
        })
        .collect()
}

fn parse_cephalopod_style(input: &str, PROB_L: usize) -> (Vec<Vec<i64>>, Vec<char>) {
    let bytes = input.lines()
        .take(PROB_L)
        .map(|l| l.bytes().collect())
        .collect();

    let mut result_nums = Vec::new();
    let mut current_active = Vec::new();
    for line in transpose(&bytes).iter()
        .map(|row| str::from_utf8(row).expect("Failed to decode from bytes").trim()) {

        if line.is_empty() {
            result_nums.push(current_active);
            current_active = Vec::new();
            continue
        }
        current_active.push(line.parse().expect("Failed to parse num"));
    }
    result_nums.push(current_active);

    let ops = input.lines().nth(PROB_L).expect("No ops found")
        .split(' ')
        .filter(|n| n.contains(&['*', '+']))
        .map(|o| o.chars().next().unwrap())
        .collect();

    (result_nums, ops)
}

fn sheet_problems_and_ops<const PROB_L: usize, const PROB_NUM: usize>(input: &str) -> (Matrix<PROB_L, PROB_NUM, i64>, Vec<char>) {
    let mut numbers = Vec::new();
    let mut iter = input.lines();
    for i in 0..PROB_L {
        let line = iter.next().expect("Not enough lines")
            .split(' ')
            .filter(|n| n.contains(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
            .map(|n| n.parse().expect("Failed to parse number"))
            .collect();
        numbers.push(line);
    }
    let ops = iter.next().expect("No op line")
        .split(' ')
        .filter(|n| n.contains(&['*', '+']))
        .map(|o| o.chars().next().unwrap())
        .collect();

    (Matrix::from_vec(numbers).transpose(), ops)
}

#[test]
fn test_parsing() {
    let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
    let (matrix, ops) = sheet_problems_and_ops::<3, 4>(input);
    assert_eq!(matrix.rows[0], [123, 45, 6]);
    assert_eq!(matrix.rows[1], [328, 64, 98]);
    assert_eq!(matrix.rows[2], [51, 387, 215]);
    assert_eq!(matrix.rows[3], [64, 23, 314]);
    assert_eq!(ops, ['*', '+', '*', '+']);
}

#[test]
fn test_solve() {
    let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
    assert_eq!(solve_problems_from_inp::<3,4>(input), vec![33210, 490, 4243455, 401])
}

#[test]
fn test_parse_cephalopod_style() {
    let input = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
    let (matrix, ops) = parse_cephalopod_style(input, 3);
    assert_eq!(matrix[0], [1, 24, 356]);
    assert_eq!(matrix[1], [369, 248, 8]);
    assert_eq!(matrix[2], [32, 581, 175]);
    assert_eq!(matrix[3], [623, 431, 4]);
    assert_eq!(ops, ['*', '+', '*', '+']);

    let input = "\
966 185
513 247
72  656
1   914
*   *  ";
    let (matrix, ops) = parse_cephalopod_style(input, 4);
    assert_eq!(matrix[0], [9571, 612, 63]);
    assert_eq!(matrix[1], [1269, 8451, 5764]);
}

#[test]
fn test_solve_cephalopod_style() {
    let input = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
    let (matrix, ops) = parse_cephalopod_style(input, 3);
    assert_eq!(solve_problems(matrix.into_iter(), ops), vec![8544, 625, 3253600, 1058])
}