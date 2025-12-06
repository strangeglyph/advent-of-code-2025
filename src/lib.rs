pub mod algebra;
pub mod combinatorics;

pub mod harness {
    use std::fmt::Display;

    pub fn run_timed<T: Display>(f: fn() -> T) {
        use std::time::Instant;
        let now = Instant::now();

        let result = f();
        let elapsed = now.elapsed();

        println!("Result:  {}", result);
        println!("Elapsed: {:.2?}", elapsed);
    }
}

pub mod util {
    pub fn posmod(val: isize, k: isize) -> isize {
        ((val % k) + k) % k
    }

    /// triangle number, sum of [1,k]
    pub fn triangle(k: u64) -> u64 {
        (k * (k + 1)) / 2
    }

    pub fn gcd(a: u32, b: u32) -> u32 {
        if b == 0 { return a; }
        gcd(b, a % b)
    }

    pub fn transpose<T: Copy>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
        if v.len() == 0 { return Vec::new(); }
        let mut r = Vec::with_capacity(v[0].len());

        for y in 0..v.len() {
            if r.len() <= y { r.push(Vec::with_capacity(v.len())) }
            for x in 0..v[y].len() {
                if r.len() <= x { r.push(Vec::with_capacity(v.len())) }
                r[x].push(v[y][x])
            }
        }

        r
    }

    #[test]
    pub fn test_transpose() {
        let v = vec![vec![1], vec![2, 3], vec![1,2,3]];
        assert_eq!(transpose(&v), vec![vec![1,2,1], vec![3, 2], vec![3]])
    }
}

