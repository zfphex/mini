/// 
/// Only supports boolean flags.
/// 
/// ```
/// use mini::args;
/// 
/// args![
///     Run, "Runs the program", release, quiet;
///     Build, "Builds the program", release;
///     Test, "Tests the program",;
///     Bench, "",;
/// ];
/// 
/// //args() & help() will be defined in same file as args![].
/// match args() {
///     Command::Run(run) => println!("Run without showing output? {}", run.quiet),
///     Command::Build(build) => println!("Build with release? {}", build.release),
///     Command::Test(_) => println!("Execute test function."),
///     Command::Empty => help(),
///     _ => {}
/// }
/// ```
#[macro_export]
macro_rules! args {
    //TODO: Any way to do optional struct?
    ($($arg:tt, $description:expr, $($element:ident),*);* $(;)?) => {
        $(
            #[derive(Debug, Default)]
            pub struct $arg {
                $(pub $element: bool),*
            }
        )*

        #[derive(Debug)]
        pub enum Command {
            $(
                $arg($arg),
            )*
            Empty,
        }

        pub fn args() -> Command {
            let args: Vec<String> = std::env::args().skip(1).collect();

            if args.is_empty() {
                return Command::Empty;
            }

            let flags = if args.len() > 1 {
                &args[1..]
            } else {
                &[]
            };

            $(
                if args[0].to_lowercase().contains(stringify!($arg).to_lowercase().as_str()) {
                    #[allow(unused)]
                    let mut s = $arg::default();
                    $(
                        if flags.contains(&format!("--{}", stringify!($element))) {
                            s.$element = true;
                        }
                    )*

                    return Command::$arg(s);
                }
            )*

            Command::Empty
        }

        pub fn help() {
            println!("Usage:");
            println!("\tcargo [<option> <args>]");
            println!("Commands:");
            $(
                //TODO: Tab padding.
                println!("\t{}\t{}", stringify!($arg).to_lowercase(), $description);
            )*
        }
    }
}