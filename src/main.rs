extern crate config;
extern crate toml;

use std::env;
use std::process;
use std::path::Path;

use jarvis::*;
use jarvis::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Command::new (&args).unwrap_or_else (|err| {
	println!("Problem parsing arguments: {}", err);
	process::exit(1);
    });


    run (command);
}

fn run (command: Command) {
    verify_config_files()
	.unwrap_or_else(|err| {
	    println!("{}", err);
	    process::exit(1);
	});

    let mut config = config::Config::default();
    let config =
	config.merge (config::File::from(Path::new(".jarvis")))
	.unwrap_or_else(|err| {
	    println!("Error while reading config: {}", err);
	    process::exit(1);
	});

    let mut exec_path =
	verify_command (command.clone(), config.clone())
	.unwrap_or_else (|err| {
	    println!("{}", err);
	    process::exit(1);
	});

    for arg in command.args {
	exec_path = exec_path.to_owned() + format!(" \"{}\"", arg).as_str();
    }

    println!("Executing: {}", exec_path);

    let mut child = process::Command::new("sh");

    let child = child
	.arg("-c")
	.arg(&exec_path);

    let child = child
	.spawn()
	.expect(format!("failed to execute {}", exec_path).as_str());

    child
	.wait_with_output()
	.expect("failed to wait on child");
}
