use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Write,
    sync::Mutex,
    time::{Duration, Instant},
};

#[doc(hidden)]
pub static mut EVENTS: Mutex<Vec<Event>> = Mutex::new(Vec::new());

#[macro_export]
/// Must go before any `profile!()` macros.
/// Otherwise it won't be dropped last and can't print all the results.
macro_rules! defer_print {
    () => {
        let _d = $crate::Defer(Some(|| {
            $crate::results(None);
        }));
    };
}

#[macro_export]
/// Print the profiling results.
///
/// Will not do anything is profiling is disabled.
macro_rules! print_profile {
    () => {
        // #[cfg(not(feature = "profile"))]
        // panic!("Called print without profile being enabled");

        // #[cfg(feature = "profile")]
        $crate::results(None);
    };
    ($($name:expr),*) => {
        // #[cfg(not(feature = "profile"))]
        // panic!("Called print without profile being enabled");

        // #[cfg(feature = "profile")]
        {
            let names = &[$(
                $name
            ),*];
            $crate::results(Some(names));

        }
    };
}

/// Creates a string with the results of every profile.
#[doc(hidden)]
pub fn results(names: Option<&[&str]>) {
    let lock = unsafe { EVENTS.lock().unwrap() };
    let events = lock.as_slice();

    if events.is_empty() {
        return;
    }

    let mut map: HashMap<Location, Vec<Event>> = HashMap::new();

    for event in events {
        if let Some(names) = names {
            if !names.contains(&event.location.full_name) && !names.contains(&event.location.name) {
                continue;
            }
        }

        match map.entry(event.location.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(event.clone());
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![event.clone()]);
            }
        }
    }

    let results = calculate(map);

    println!("{}", results);
}

fn calculate(map: HashMap<Location, Vec<Event>>) -> String {
    let mut scores = Vec::new();

    for (k, v) in map.iter() {
        let mut mean = Duration::default();
        let mut min = v.get(0).unwrap_or(&Event::default()).elapsed();
        let mut max = Duration::default();

        for event in v {
            let elapsed = event.elapsed();

            if elapsed < min {
                min = elapsed;
            }

            if elapsed > max {
                max = elapsed;
            }

            mean += elapsed;
        }

        scores.push(Score {
            full_name: k.full_name,
            file: k.file,
            line: k.line,
            total: mean,
            mean: Duration::from_secs_f64(mean.as_secs_f64() / v.len() as f64),
            min,
            max,
            count: v.len(),
        });
    }

    let mut string = String::new();
    for score in scores {
        writeln!(
            &mut string,
            "{} ({} runs) {}:{}",
            score.full_name, score.count, score.file, score.line,
        )
        .unwrap();
        writeln!(&mut string, "  - total: {:?}", score.total).unwrap();
        writeln!(&mut string, "  - mean:  {:?}", score.mean).unwrap();
        writeln!(&mut string, "  - min:   {:?}", score.min).unwrap();
        writeln!(&mut string, "  - max:   {:?}\n", score.max).unwrap();
    }

    string
}

#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct Event {
    pub location: Location,
    pub start: Option<Instant>,
    pub end: Option<Instant>,
}

impl Event {
    pub fn elapsed(&self) -> Duration {
        self.end.unwrap().duration_since(self.start.unwrap())
    }
}

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct Score {
    pub full_name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub total: Duration,
    pub mean: Duration,
    pub min: Duration,
    pub max: Duration,
    pub count: usize,
}

#[doc(hidden)]
#[derive(Hash, Eq, PartialEq, Clone, Debug, Default)]
pub struct Location {
    pub full_name: &'static str,
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
}

#[doc(hidden)]
#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name(f);
        &name[..name.len() - 3]
    }};
}

/// Defer execution of a closure until the return value is dropped.
#[doc(hidden)]
pub struct Defer<F: FnOnce()>(pub Option<F>);

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        self.0.take().map(|f| f());
    }
}

//We can't defer functions with a deeper scope.
//Features must be handled this way.
#[macro_export]
macro_rules! profile {
    () => {
        // #[cfg(feature = "profile")]
        let full_name = $crate::function!();
        // #[cfg(feature = "profile")]
        let name = &full_name[full_name.find("::").unwrap() + 2..];
        // #[cfg(feature = "profile")]
        let mut event = $crate::Event {
            location: $crate::Location {
                full_name,
                name,
                file: file!(),
                line: line!(),
            },
            start: Some(std::time::Instant::now()),
            end: None,
        };
        // #[cfg(feature = "profile")]
        let _d = $crate::Defer(Some(|| {
            event.end = Some(std::time::Instant::now());
            unsafe { $crate::EVENTS.lock().unwrap().push(event) };
        }));
    };
    ($name:expr) => {
        // #[cfg(feature = "profile")]
        let mut event = $crate::Event {
            location: $crate::Location {
                full_name: $name,
                name: $name,
                file: file!(),
                line: line!(),
            },
            start: Some(std::time::Instant::now()),
            end: None,
        };
        // #[cfg(feature = "profile")]
        let _d = $crate::Defer(Some(|| {
            event.end = Some(std::time::Instant::now());
            unsafe { $crate::EVENTS.lock().unwrap().push(event) };
        }));
    };
}

#[macro_export]
macro_rules! step {
    ($name:expr) => {
        /*
        pub fn test() {
            profile!();

            //something slow
            step!("Applying parameters")

            //something slow
            //something slow
            step!("Shaving the yak")
        }
         */
    };
}

//'cargo test -- --show-output'
#[cfg(test)]
mod tests {
    use crate::profile;

    #[test]
    fn test() {
        fn hi() {
            profile!();
        }
        fn hi2() {
            profile!();
        }

        hi();
        hi2();

        crate::print_profile!("hi");
    }
}
