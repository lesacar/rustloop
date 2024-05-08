use std::env;
use std::process;
use std::time::Duration;
use std::thread::sleep;

mod help;

fn main() {
    // Calculate the base name
    let binname = env::args()
        .nth(0)
        .expect("Failed to get base name")
        .split('/')
        .last()
        .map(|s| s.to_owned())
        .expect("Failed to get base name");

    let argv: Vec<String> = env::args().skip(1).collect(); // Collect all arguments except the program name

    // Check if argc < 2
    if argv.is_empty() {
        help::help(binname.clone());
        process::exit(1);
    }

    let mut sleepdur = 250;
    let bintorun;

    // Check if the first argument is an integer or a command name
    let first_arg_is_integer = argv.get(0).map_or(false, |arg| arg.parse::<i64>().is_ok());
    if first_arg_is_integer {
        // If the first argument is an integer, skip it
        sleepdur = argv.get(0).expect("err").parse::<u64>().unwrap();
        if argv.len() < 2 {
            // If there's no argument after skipping the integer, exit
            help::help(binname.clone());
            process::exit(1);
        }
        // Use the second argument as the command name
        bintorun = &argv[1];
    } else {
        // If the first argument is not an integer, assume it's a command name
        bintorun = &argv[0];
    }

    if argv.get(1) == Some(&String::from("-h")) || argv.get(1) == Some(&String::from("--help")) {
        help::help(binname.clone());
        process::exit(1);
    }

    // Construct arguments to pass to the command
    let cmd_args: Vec<&str> = argv.iter().skip(if first_arg_is_integer { 2 } else { 1 }).map(|s| s.as_str()).collect();

    loop {
        // Construct the command
        let mut cmd = if bintorun.is_empty() {
            process::Command::new("echo") // Default to echo if no command name provided
        } else {
            process::Command::new(bintorun)
        };
        cmd.args(&cmd_args);

        // Execute the command
        if let Err(err) = cmd.spawn() {
            eprintln!("Failed to execute command: {}", err);
        }
        sleep(Duration::from_millis(sleepdur));
    }
}

