use aoc_2025::harness::run_timed;
use aoc_2025::algebra::algebra::Matrix;

const INPUT_A: &'static str = include_str!("../../../resources/day04/input_a.txt");


fn main() {
    run_timed(solve_a);
    run_timed(solve_b);
}

fn solve_a() -> i16 {
    let removable_grid = get_removable(&matrix_from_input::<136,136>(INPUT_A));
    sum(&removable_grid)
}

fn solve_b() -> i32 {
    let mut grid = matrix_from_input::<136, 136>(INPUT_A);
    let mut tot_sum = 0;
    loop {
        let removable = get_removable(&grid);
        let new = sum(&removable);
        if new == 0 { break; }

        tot_sum += new as i32;
        grid = grid - removable;
    }

    tot_sum
}


const MOVABLE_FILTER: Matrix<3, 3, i16> = Matrix::from_arr([
    [-1, -1, -1],
    [-1,  4, -1],
    [-1, -1, -1]
]);

fn get_removable<const W: usize, const H: usize>(grid: &Matrix<W, H, i16>) -> Matrix<W, H, i16> {
    grid.convolve_0(&MOVABLE_FILTER)
        .map(|&val| if val <= 0 { 0 } else { 1 })
}

fn sum(m: &Matrix<136, 136, i16>) -> i16 {
    *(Matrix::all_ones() * (m * &Matrix::all_ones().transpose())).get(0, 0)
}


fn matrix_from_input<const W: usize, const H: usize>(input: &str) -> Matrix<W, H, i16> {
    let as_vec = input.lines()
        .map(|s| s.chars().map(|c| {
            match c {
                '.' => 0,
                '@' => 1,
                _ => panic!("Unexpected char")
            }
        }).collect::<Vec<i16>>())
        .collect();

    Matrix::from_vec(as_vec)
}

