use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();

    loop {
        let start = Instant::now();
        let new = Instant::now();
        println!("{:?}", start.elapsed());
        assert!(new.elapsed().as_micros() > 100);

        if now.elapsed() > Duration::from_millis(10) {
            break;
        }
    }
}
