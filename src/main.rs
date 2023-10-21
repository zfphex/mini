use mini::*;

fn main() {
    info!("This is an info message");
    warn!("This is a warning!");
    error!("This is an error!!!");

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

    println!("{}", results!());
    println!("{}", results!("mini::test_fn", "Custom function name"));
}
