use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();

    let mut dur = Vec::with_capacity(300000);
    loop {
        let start = Instant::now();

        if now.elapsed() > Duration::from_millis(10) {
            break;
        }

        let end = Instant::now();
        dur.push((start, end));
    }

    let total = now.elapsed();
    let sum: Duration = dur
        .iter()
        .map(|(start, end)| end.duration_since(*start))
        .sum();
    println!("total time: {:?}, len: {}", total, dur.len());
    println!("unaccounted time: {:?}", total - sum);
    println!("profiled time: {:?}", sum);
}
