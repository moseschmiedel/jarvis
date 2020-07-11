use config;

pub struct Command<'a> {
    pub identifier: String,
    pub args: &'a [String],
}

impl<'a> Command<'a> {
    pub fn new (args: &[String]) -> Result<Command, &'static str> {
	if args.len() < 2 {
	    return Err("Not enough arguments");
	    // TODO: Provide detailed information how the program can be run
	}

	let identifier = args[1].clone();
	let args = &args[2..];

	Ok(Command { identifier, args })
    }
}

pub fn verify_command (command: Command, config: config::Config)
		       -> Result<String, &'static str> {
    Ok("Got here".to_string())
}
