use broken_app::{algo, sum_even};

fn main() {
    let data: Vec<i64> = (0..50_000).collect();
    let dedup_data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();

    for _ in 0..200 {
        let _ = sum_even(&data);
        let _ = algo::slow_fib(35);
        let _ = algo::slow_dedup(&dedup_data);
    }
    println!("done");
}
