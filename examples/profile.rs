#![allow(static_mut_refs)]
use crossbeam_channel::*;
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Write,
    sync::LazyLock,
    time::{Duration, Instant},
};

static mut FINISHED: bool = false;

//TODO: Condvar...
//TODO: A lock free ringbuffer would probably be faster, although this thread would need to keep up with consumuption.
static mut SENDER: LazyLock<Sender<(ProfileLocation, ProfileEvent)>> = LazyLock::new(|| {
    let (s, r) = crossbeam_channel::unbounded::<(ProfileLocation, ProfileEvent)>();

    std::thread::spawn(move || {
        let mut map: HashMap<ProfileLocation, Vec<ProfileEvent>> = HashMap::new();
        loop {
            while let Ok((location, event)) = r.recv_timeout(std::time::Duration::from_millis(16)) {
                match map.entry(location) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(event);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![event]);
                    }
                }
            }

            if unsafe { FINISHED } {
                break;
            }
        }

        let results = calculate(map);
        println!("{}", results);
    });

    s
});

pub fn calculate(map: HashMap<ProfileLocation, Vec<ProfileEvent>>) -> String {
    let mut scores = Vec::new();

    for (location, events) in &map {
        let mut total = Duration::default();
        let mut min = events.get(0).unwrap().elapsed();
        let mut max = Duration::default();

        for event in events {
            let elapsed = event.elapsed();
            min = min.min(elapsed);
            max = max.max(elapsed);
            total += elapsed;
        }

        scores.push(ProfileScore {
            full_name: location.full_name,
            file: location.file,
            line: location.line,
            total,
            mean: Duration::from_secs_f64(total.as_secs_f64() / events.len() as f64),
            min,
            max,
            count: events.len(),
        });
    }

    let mut string = String::new();
    for score in scores {
        writeln!(
            &mut string,
            "{} ({} {}) {}:{}",
            score.full_name,
            score.count,
            if score.count == 1 { "run" } else { "runs" },
            score.file,
            score.line,
        )
        .unwrap();

        writeln!(&mut string, "  - total: {:.2?}", score.total).unwrap();
        writeln!(&mut string, "  - mean:  {:.2?}", score.mean).unwrap();
        writeln!(&mut string, "  - min:   {:.2?}", score.min).unwrap();
        writeln!(&mut string, "  - max:   {:.2?}\n", score.max).unwrap();
    }

    let mut string = string.trim_end().to_string();
    string.push('\n');
    string
}

#[derive(Debug, Default)]
pub struct ProfileScore {
    pub full_name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub total: Duration,
    pub mean: Duration,
    pub min: Duration,
    pub max: Duration,
    pub count: usize,
}

#[derive(Debug)]
pub struct ProfileEvent {
    pub start: Instant,
    pub end: Option<Instant>,
}

impl ProfileEvent {
    pub fn elapsed(&self) -> Duration {
        self.end.unwrap().duration_since(self.start)
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Default)]
pub struct ProfileLocation {
    pub full_name: &'static str,
    pub file: &'static str,
    pub line: u32,
}

pub struct Defer {
    pub location: ProfileLocation,
    pub start: Instant,
}

impl Drop for Defer {
    fn drop(&mut self) {
        unsafe {
            let _ = SENDER
                .send((
                    core::ptr::read(&self.location),
                    ProfileEvent {
                        start: self.start,
                        end: Some(std::time::Instant::now()),
                    },
                ));
        }
    }
}

fn main() {
    // defer_results!();

    let time = Instant::now();
    loop {
        let start = std::time::Instant::now();
        let location = ProfileLocation {
            full_name: mini::function!(),
            file: file!(),
            line: line!(),
        };

        let _defer = Defer { location, start };

        if time.elapsed() > Duration::from_millis(2) {
            unsafe { FINISHED = true };
            dbg!(time.elapsed());
            std::thread::sleep_ms(300);
            break;
        }
    }
}
