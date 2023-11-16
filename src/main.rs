use mini::*;

fn main() {
    info!("This is an info message");
    info_raw!("This is an raw info message");
    warn!("This is a warning!");
    warn_raw!("This is a raw warning!");
    error!("This is an error!");
    error_raw!("This is a raw error!");

    profile();
}

fn test_fn() {
    profile!();
    std::thread::sleep(std::time::Duration::from_millis(2));
}

fn test_fn_2() {
    profile!("Custom function name");
}

fn test_fn_3() {
    profile!();
}

fn profile() {
    test_fn();
    test_fn_2();
    test_fn_3();

    results!();
}
