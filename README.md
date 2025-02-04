```rs
use mini::{info, warn, error};

fn main() {
    info!("This is an info message");
    warn!("This is a warning!");
    error!("This is an error!!!");
}
```

Enable features by adding this to your `Cargo.toml` file.

```
profile = ["mini/profile"]
info = ["mini/info"]
warn = ["mini/warn"]
error = ["mini/error"]
```

Then use them by adding the correct feature flag.

Use `--features "info"` to show info, warnings and errors.

Use `--features "warn"` to show warnings and errors.

Use `--features "error"` to show only errors.

Use `--features "profile"` to enable profiling.

```rs
use mini::profile;

fn test_fn() {
    profile!();
}

fn test_fn_2() {
    profile!("Custom function name");
}

fn main() {
    test_fn();
    test_fn_2();
    results!();
}
```

```
Custom function name (1 runs) src\main.rs:8
  - total: 0ns
  - mean:  0ns
  - min:   0ns
  - max:   0ns

miniprofile::test_fn (1 runs) src\main.rs:4
  - total: 400ns
  - mean:  400ns
  - min:   400ns
  - max:   400ns
```
