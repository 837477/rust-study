fn sum_rust() -> i64 {
    let mut total = 0;
    for i in 1..=1_000_000_000 {
        total += i;
    }
    total
}

fn main() {
    let start = std::time::Instant::now();
    let result = sum_rust();
    let duration = start.elapsed();
    println!("Elapsed time: {:?}", duration);
}