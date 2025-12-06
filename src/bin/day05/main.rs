use std::ops::Range;
use aoc_2025::combinatorics::combinatorics::RangeSet;
use aoc_2025::harness::run_timed;

const INPUT_A: &'static str = include_str!("../../../resources/day05/input_a.txt");

fn main() {
    run_timed(solve_a);
    run_timed(solve_b);
}

fn solve_b() -> u64 {
    let (ranges, _) = parse_input(INPUT_A);
    let set = RangeSet::from_vec(ranges);
    set.len()
}

fn solve_a() -> usize {
    let (ranges, ids) = parse_input(INPUT_A);
    let set = RangeSet::from_vec(ranges);
    ids.into_iter().filter(|id| set.contains(*id)).count()
}

fn parse_range(line: &str) -> Range<u64> {
    let (start, end) = line.split_once("-").expect("Failed to split range");
    let start = start.parse().expect("Failed to parse start");
    let end: u64 = end.parse().expect("Failed to parse end");
    start..(end+1)
}

fn parse_input(input: &str) -> (Vec<Range<u64>>, Vec<u64>) {
    let (ranges, ids) = input.split_once("\n\n").expect("Failed to split input in half");
    let ranges = ranges.lines().map(parse_range).collect();
    let ids = ids.lines().map(|s| s.parse().expect("Failed to parse id")).collect();

    (ranges, ids)
}

#[test]
fn test_parse_input() {
    let input = "1-2
4-10
10000-10000000

1
70
981
";
    let (ranges, ids) = parse_input(input);
    assert_eq!(ranges, vec![1..3, 4..11, 10000..10000001]);
    assert_eq!(ids, vec![1, 70, 981]);
}

#[test]
fn test_part_2() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
    let (ranges, _) = parse_input(input);
    let set = RangeSet::from_vec(ranges);
    assert_eq!(set.len(), 14);

    let input = "263168346238540-263700364947153
14374432572227-15942816853763
444767118084396-445413354731974

1";
    let (ranges, _) = parse_input(input);
    let set = RangeSet::from_vec(ranges);
    assert_eq!(set.len(), 2746639637730);


    let input = include_str!("../../../resources/day05/test_input_b.txt");
    let (ranges, _) = parse_input(input);
    let set = RangeSet::from_vec(ranges);
    assert_eq!(set.len(), 2891545127672);

    let input = include_str!("../../../resources/day05/test_input_c.txt");
    let (ranges, _) = parse_input(input);
    let set = RangeSet::from_vec(ranges);
    assert_eq!(set.len(), 1845645509158);


    let input = include_str!("../../../resources/day05/test_input_a.txt");
    let (ranges, _) = parse_input(input);
    let set = RangeSet::from_vec(ranges);
    assert_eq!(set.len(), 359913027576322);


}

#[test]
fn test_ranges() {
    let (ranges, _) = parse_input(INPUT_A);
    let set = RangeSet::from_vec(ranges);
    let mut last = &set.data[0];
    println!("{last:?}");
    for range in set.data[1..].iter() {
        println!("{range:?}");
        assert!(range.start > last.end);
        last = range
    }
}

