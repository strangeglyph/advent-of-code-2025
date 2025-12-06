pub mod algebra {
    use std::fmt::{Display, Formatter};
    use std::ops::*;


    pub trait One {
        fn one() -> Self;
    }
    pub trait Zero {
        fn zero() -> Self;
    }

    macro_rules! zero_one {
        ($ty:ty) => {
            impl Zero for $ty {
                fn zero() -> Self {
                    0 as $ty
                }
            }
            impl One for $ty {
                fn one() -> Self {
                    1 as $ty
                }
            }
        };
    }

    zero_one!(u8);
    zero_one!(u16);
    zero_one!(u32);
    zero_one!(u64);
    zero_one!(u128);
    zero_one!(usize);

    zero_one!(i8);
    zero_one!(i16);
    zero_one!(i32);
    zero_one!(i64);
    zero_one!(i128);
    zero_one!(isize);

    zero_one!(f32);
    zero_one!(f64);

    pub struct Matrix<const WIDTH: usize, const HEIGHT: usize, T> {
        pub rows: [[T; WIDTH]; HEIGHT],
    }

    impl<const WIDTH: usize, const HEIGHT: usize, T> Matrix<WIDTH, HEIGHT, T> {
        pub fn get(&self, x: usize, y: usize) -> &T {
            &self.rows[y][x]
        }

        pub fn map<R: Zero + Copy>(&self, f: fn(&T) -> R) -> Matrix<WIDTH, HEIGHT, R> {
            let mut result = [[R::zero(); WIDTH]; HEIGHT];
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    result[y][x] = f(self.get(x, y));
                }
            }

            Matrix { rows: result }
        }
    }

    impl<const WIDTH: usize, const HEIGHT: usize, T: Copy> Matrix<WIDTH, HEIGHT, T> {
        pub fn from_vec(data: Vec<Vec<T>>) -> Matrix<WIDTH, HEIGHT, T> {
            if data.len() != HEIGHT {
                panic!("Bad input height");
            }

            let mut result = [[data[0][0]; WIDTH]; HEIGHT];
            for y in 0..HEIGHT {
                if data[y].len() != WIDTH {
                    panic!("Bad input length");
                }
                for x in 0..WIDTH {
                    result[y][x] = data[y][x];
                }
            }

            Matrix { rows: result }
        }

        pub const fn from_arr(arr: [[T; WIDTH]; HEIGHT]) -> Matrix<WIDTH, HEIGHT, T> {
            Matrix { rows: arr }
        }

        pub fn transpose(&self) -> Matrix<HEIGHT, WIDTH, T> {
            let dummy = *self.get(0, 0);
            let mut rows = [[dummy; HEIGHT]; WIDTH];
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    rows[x][y] = *self.get(x, y)
                }
            }

            Matrix { rows }
        }
    }

    impl<const WIDTH: usize, T: Zero + Copy> Matrix<WIDTH, 1, T> {
        // row vector of 0s
        pub fn zero() -> Matrix<WIDTH, 1, T> {
            Matrix {
                rows: [[T::zero(); WIDTH]],
            }
        }
    }

    impl<const WIDTH: usize, T: One + Copy> Matrix<WIDTH, 1, T> {
        // row vector of all 1s
        pub fn all_ones() -> Matrix<WIDTH, 1, T> {
            Matrix {
                rows: [[T::one(); WIDTH]],
            }
        }
    }

    impl<const WIDTH: usize, T: Zero + One + Copy> Matrix<WIDTH, 1, T> {
        // row unit vector
        pub fn unit(dim: usize) -> Matrix<WIDTH, 1, T> {
            let mut result = Matrix::zero();
            result.rows[0][dim] = T::one();
            result
        }
    }

    impl<
        const WIDTH: usize,
        const HEIGHT: usize,
        T: Add<Output = T> + Mul<Output = T> + Copy + Zero,
    > Matrix<WIDTH, HEIGHT, T>
    {
        pub fn convolve_0<const KW: usize, const KH: usize>(
            &self,
            kernel: &Matrix<KW, KH, T>,
        ) -> Matrix<WIDTH, HEIGHT, T> {
            if KW % 2 == 0 {
                panic!("Kernel must have odd width");
            }
            if KH % 2 == 0 {
                panic!("Kernel must have odd height");
            }

            let kcenter_x = KW / 2;
            let kcenter_y = KH / 2;

            let mut mat_result = [[T::zero(); WIDTH]; HEIGHT];
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let mut result = T::zero();
                    for ky in 0..KH {
                        for kx in 0..KW {
                            if x + kx < kcenter_x || y + ky < kcenter_y {
                                continue;
                            }
                            let mx = x + kx - kcenter_x;
                            let my = y + ky - kcenter_y;
                            if mx >= WIDTH || my >= HEIGHT {
                                continue;
                            }

                            let kval = *kernel.get(kx, ky);
                            let mval = *self.get(mx, my);
                            result = result + (kval * mval);
                        }
                    }
                    mat_result[y][x] = result;
                }
            }

            Matrix { rows: mat_result }
        }
    }

    impl<const WIDTH: usize, const HEIGHT: usize, T: Add<Output = T> + Copy> Add
    for Matrix<WIDTH, HEIGHT, T>
    {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            let mut result = self.rows.clone();
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    result[y][x] = result[y][x] + *other.get(x, y);
                }
            }
            Matrix { rows: result }
        }
    }

    impl<const WIDTH: usize, const HEIGHT: usize, T: Sub<Output = T> + Copy> Sub
    for Matrix<WIDTH, HEIGHT, T>
    {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            let mut result = self.rows.clone();
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    result[y][x] = result[y][x] - *other.get(x, y);
                }
            }
            Matrix { rows: result }
        }
    }

    impl<
        const WIDTH: usize,
        const HEIGHT: usize,
        const OTHER_WIDTH: usize,
        T: Add<Output = T> + Mul<Output = T> + Copy + Zero,
    > Mul<Matrix<OTHER_WIDTH, WIDTH, T>> for Matrix<WIDTH, HEIGHT, T>
    {
        type Output = Matrix<OTHER_WIDTH, HEIGHT, T>;

        fn mul(self, rhs: Matrix<OTHER_WIDTH, WIDTH, T>) -> Self::Output {
            let mut result = [[T::zero(); OTHER_WIDTH]; HEIGHT];

            for ry in 0..HEIGHT {
                for rx in 0..OTHER_WIDTH {
                    for x in 0..WIDTH {
                        result[ry][rx] = result[ry][rx] + (*self.get(x, ry) * *rhs.get(rx, x))
                    }
                }
            }

            Matrix { rows: result }
        }
    }

    impl<
        const WIDTH: usize,
        const HEIGHT: usize,
        const OTHER_WIDTH: usize,
        T: Add<Output = T> + Mul<Output = T> + Copy + Zero,
    > Mul<&Matrix<OTHER_WIDTH, WIDTH, T>> for &Matrix<WIDTH, HEIGHT, T>
    {
        type Output = Matrix<OTHER_WIDTH, HEIGHT, T>;

        fn mul(self, rhs: &Matrix<OTHER_WIDTH, WIDTH, T>) -> Self::Output {
            let mut result = [[T::zero(); OTHER_WIDTH]; HEIGHT];

            for ry in 0..HEIGHT {
                for rx in 0..OTHER_WIDTH {
                    for x in 0..WIDTH {
                        result[ry][rx] = result[ry][rx] + (*self.get(x, ry) * *rhs.get(rx, x))
                    }
                }
            }

            Matrix { rows: result }
        }
    }

    impl <const WIDTH: usize, const HEIGHT: usize, T: Display> Display for Matrix<WIDTH, HEIGHT, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for y in 0..HEIGHT {
                for x in 0..HEIGHT {
                    write!(f, "{:>2}", self.get(x, y))?;
                }
                writeln!(f)?;
            }

            Ok(())
        }
    }

    #[test]
    fn test_transpose() {
        assert_eq!(
            Matrix::<3, 1, u8>::all_ones().transpose().rows,
            [[1], [1], [1]]
        )
    }

    #[test]
    fn test_convolve0() {
        let M = Matrix {
            rows: [[1, 0, 1], [1, 1, 0], [1, 1, 1]],
        };
        let F = Matrix { rows: [[1, 0, 1]] };

        let expected = [[0, 2, 0], [1, 1, 1], [1, 2, 1]];
        assert_eq!(M.convolve_0(&F).rows, expected)
    }

    #[test]
    fn test_mul() {
        let M = Matrix {
            rows: [[1, 0, 1], [0, 1, 0], [1, 0, 1]]
        };

        let M2 = M * Matrix::all_ones().transpose();
        assert_eq!(M2.rows, [[2], [1], [2]]);
        assert_eq!((Matrix::all_ones() * M2).rows, [[5]]);
    }
}
