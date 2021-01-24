extern crate walkdir;

use config;
use std::env;
use std::path;
use std::ffi::OsStr;
use walkdir::WalkDir;

/// Defines a command for this application
///
/// Commands have an identifier, which is the first argument given to
/// this application, and arguments (args) which are all the other commandline
/// arguments given to this application.
pub struct Command<'a> {
    pub identifier: String,
    pub args: &'a [String],
}

impl<'a> Command<'a> {
    pub fn new (args: &[String])
		-> Result<Command, &'static str> {

	if args.len() < 2 {
	    return Err("Not enough arguments");
	    // TODO: Provide detailed information how the program can be run
	}

	let identifier = args[1].clone();
	let args = &args[2..];

	Ok(Command { identifier, args })
    }

    pub fn clone (&self)
		  -> Command {
	Command {
	    identifier: self.identifier.clone(),
	    args: self.args.clone()
	}
    }
}

/// Returns shell command that should be run
///
/// Searches for the commands subsection in the provided config and checks
/// wether the provide command is part of the commands in the config or not.
/// If the command exists in the config then the shell command, that is specified
/// at the command is returned.
/// If the search for the subsection or the command fails, respective error messages
/// are printed.
pub fn verify_command (command: Command, config: config::Config)
		       -> Result<String, String> {
    let commands =
	config.get_table("commands")
	.unwrap();
    let exec_path = commands.get (&command.identifier);
    let exec_path = match exec_path {
	Some(exec_path) => exec_path.clone().into_str(),
	None => return Err(
	    format!("'{}' is not a valid command.",
		    &command.identifier)),
    };

    Ok(exec_path.unwrap())
}

/// Returns Path to config file
///
/// Searches for possible config files in the directory tree upwards, beginning from the directory
/// the application is called from. The first config file that is found is used, but if more than 
/// one config file exists per directory the application terminates and prints an error message.
pub fn find_and_verify_config_files ()
			    -> Result<path::PathBuf, &'static str> {
    let mut no_conf_files = 0;

    let formats = [".yml", ".json", ".toml"];

    let mut current_dir = env::current_dir().unwrap();

    while no_conf_files == 0
	&& current_dir != path::PathBuf::from("/") {
	for entry in WalkDir::new(&(current_dir.to_string_lossy().to_string()))
	    .max_depth(1)
	    {
		let entry = entry.unwrap();
		for format in &formats {
		    if entry.file_name()
			== OsStr::new(&(".jarvis".to_owned() + format)) {
			    no_conf_files += 1;
			    current_dir.push(".jarvis".to_owned() + format);
			}
		}
	    }

	    if no_conf_files == 0 {
		current_dir.pop();
	    }
    }
    if no_conf_files == 0 {
	return Err("No config file found in current project.
Please provide a '.jarvis.{yml|json|toml}' file in your project root.");

    } else if no_conf_files > 1 {
	return Err("Too many config files found in current project.
Please remove all '.jarvis.{yml|json|toml}' files but one.");
    }

    Ok(current_dir)
}
