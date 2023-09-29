```rs
use log::*;

fn main() {
    info!("This is an info message");
    warn!("This is a warning!");
    error!("This is an error!!!");
}
```

Use `--features "strip"` to remove logging from your build.

Use `--features "warn"` to hide info.

Use `--features "error"` to hide info and warnings.