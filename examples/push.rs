use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();
    let mut dur = Vec::with_capacity(300000);
    loop {
        let start = Instant::now();
        if now.elapsed() > Duration::from_millis(10) {
            break;
        }
        dur.push((start, Instant::now()));
        let end = Instant::now();
        dbg!(end.duration_since(start));
    }
}
