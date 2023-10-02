#[cfg(not(target_os = "windows"))]
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
mod win {
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
        pub fn GetLocalTime(lpsystemtime: *mut SystemTime);
    }
}

#[cfg(target_os = "windows")]
#[inline(always)]
pub fn now() -> String {
    let mut time = win::SystemTime::default();
    unsafe { win::GetLocalTime(&mut time) };
    format!("{:02}:{:02}:{:02}", time.hour, time.minute, time.second)
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        #[cfg(not(feature = "strip"))]
        #[cfg(not(feature = "error"))]
        #[cfg(not(feature = "warn"))]
        {
            print!("\x1b[90m{} \x1b[92mINFO\x1b[0m  {}:\x1b[30m{}\x1b[0m - ", $crate::now(), file!(), line!());
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        #[cfg(not(feature = "strip"))]
        #[cfg(not(feature = "error"))]
        {
            print!("\x1b[90m{} \x1b[93mWARN\x1b[0m  {}:\x1b[30m{}\x1b[0m - ", $crate::now(), file!(), line!());
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(not(feature = "strip"))]
        {
            print!("\x1b[90m{} \x1b[91mERROR\x1b[0m {}:\x1b[30m{}\x1b[0m - ", $crate::now(), file!(), line!());
            println!($($arg)*);
        }
    };
}
