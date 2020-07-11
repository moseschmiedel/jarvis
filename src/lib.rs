use config;
use std::fs::File;

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

pub fn verify_config_files ()
			    -> Result<bool, &'static str> {
    let mut no_conf_files = 0;

    let formats = [".yml", ".json", ".toml"];

    for format in &formats {
	let _conf_file = File::open (".jarvis".to_owned() + format);
	let _conf_file = match _conf_file {
	    Ok(_file) => {
		no_conf_files += 1;
		true
	    },
	    Err(_error) => false,
	};
    }
    if no_conf_files == 0 {
	return Err("No config file found in current project.
Please provide a '.jarvis.{yml|json|toml}' file in your project root.");

    } else if no_conf_files > 1 {
	return Err("Too many config files found in current project.
Please remove all '.jarvis.{yml|json|toml}' files but one.");
    }

    Ok(true)
}
