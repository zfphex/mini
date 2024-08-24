#[cfg(not(target_os = "windows"))]
#[doc(hidden)]
pub fn now() -> String {
    static mut ONCE: std::sync::Once = std::sync::Once::new();
    static mut TZ: i64 = 0;
    unsafe {
        ONCE.call_once(|| {
            TZ = match std::env::var("TZ") {
                Ok(offset) => offset.parse::<i64>().unwrap_or_default(),
                _ => 0,
            };
        });
    }
    let current_time = std::time::SystemTime::now();
    let since_epoch = current_time.duration_since(std::time::UNIX_EPOCH).unwrap();
    let seconds = (since_epoch.as_secs() as i64 + unsafe { TZ }) % 86400;
    let hour = seconds / 3600;
    let minute = (seconds % 3600) / 60;
    let second = seconds % 60;
    format!("{:02}:{:02}:{:02}", hour, minute, second)
}

#[cfg(target_os = "windows")]
#[doc(hidden)]
#[inline]
pub fn now() -> String {
    #[repr(C)]
    #[derive(Copy, Clone, Debug, Default)]
    pub struct SystemTime {
        pub year: u16,
        pub month: u16,
        pub day_of_week: u16,
        pub day: u16,
        pub hour: u16,
        pub minute: u16,
        pub second: u16,
        pub milliseconds: u16,
    }

    extern "system" {
        fn GetLocalTime(lpsystemtime: *mut SystemTime);
    }

    let mut time = SystemTime::default();
    unsafe { GetLocalTime(&mut time) };
    format!("{:02}:{:02}:{:02}", time.hour, time.minute, time.second)
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        #[cfg(feature = "info")]
        {
            eprint!("\x1b[90m{} \x1b[92mINFO\x1b[0m {}:\x1b[30m{}\x1b[0m - ", $crate::now(), file!(), line!());
            eprintln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! info_raw {
    ($($arg:tt)*) => {
        #[cfg(feature = "info")]
        {
            eprint!("\x1b[90m{} \x1b[92mINFO\x1b[0m ", $crate::now());
            eprintln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        #[cfg(any(feature = "warn", feature = "info"))]
        {
            eprint!("\x1b[90m{} \x1b[93mWARN\x1b[0m {}:\x1b[30m{}\x1b[0m - ", $crate::now(), file!(), line!());
            eprintln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! warn_raw {
    ($($arg:tt)*) => {
        #[cfg(any(feature = "warn", feature = "info"))]
        {
            eprint!("\x1b[90m{} \x1b[93mWARN\x1b[0m ", $crate::now());
            eprintln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(any(feature = "error", feature = "warn", feature = "info"))]
        {
            eprint!("\x1b[90m{} \x1b[91mERROR\x1b[0m {}:\x1b[30m{}\x1b[0m - ", $crate::now(), file!(), line!());
            eprintln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! error_raw {
    ($($arg:tt)*) => {
        #[cfg(any(feature = "error", feature = "warn", feature = "info"))]
        {
            eprint!("\x1b[90m{} \x1b[91mERROR\x1b[0m ", $crate::now());
            eprintln!($($arg)*);
        }
    };
}
