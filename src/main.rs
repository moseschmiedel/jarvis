extern crate config;
extern crate toml;

use std::env;
use std::fs::File;
use std::process;
use std::path::Path;
use std::collections::HashMap;

use jarvis::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Command::new (&args).unwrap_or_else (|err| {
	println!("Problem parsing arguments: {}", err);
	process::exit(1);
    });

    let mut no_conf_files = 0;

    let formats = [".yml", ".json", ".toml"];

    for format in &formats {
	let conf_file = File::open (".jarvis".to_owned() + format);
	let conf_file = match conf_file {
	    Ok(_file) => {
		no_conf_files += 1;
		true
	    },
	    Err(_error) => false,
	};
    }

    if no_conf_files == 0 {
	println!("No config file found in current project.
Please provide a '.jarvis.{{yml|json|toml}}' file in your project root.");
	process::exit(1);

    } else if no_conf_files > 1 {
	println!("Too many config files found in current project.
Please remove all '.jarvis.{{yml|json|toml}}' files but one.");
	process::exit(1);
    }


    let mut config = config::Config::default();
    config
	.merge (config::File::from(Path::new(".jarvis")))
	.unwrap_or_else(|err| {
	    println!("Error while reading config: {}", err);
	    process::exit(1);
	});
    // println!("{:?}", config.cache);

    run (command, config);
}

fn run (command: Command, config: config::Config) {
    println!("{}", command.identifier);
    println!("{:?}", command.args);

    println!("{:?}", config.try_into::<HashMap<String, toml::Value>>().unwrap());
}
