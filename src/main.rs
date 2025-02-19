use mini::*;

//Only supports boolean flags.
args![
    Run, "Runs the program", release, quiet;
    Build, "Builds the program", release;
    Test, "Tests the program",;
    Bench, "",;
];

fn main() {
    defer_results!();
    profile!();

    info!("info");
    warn!("warn");
    error!("error");

    match args() {
        Command::Run(run) => println!("Run without showing output? {}", run.quiet),
        Command::Build(build) => println!("Build with release? {}.", build.release),
        Command::Test(_) => println!("Execute test function."),
        Command::Empty => help(),
        _ => {}
    }
}
