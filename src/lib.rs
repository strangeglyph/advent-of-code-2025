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
        (k*(k+1)) / 2
    }
    
    pub fn gcd(a: u32, b: u32) -> u32 {
        if b == 0 { return a; }
        gcd(b, a % b)
    }
}