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
}