# JARVIS

`jarvis` is a command line tool that allows you to define aliases that are
local to your current project. The name is borrowed from Tony Stark's
Personal AI Assistant, because this tool allows you to talk to your
computer via the command line much like Tony Stark talks to JARVIS.

## Inspiration
When I saw [npm](https://npmjs.com) and its feature to define local
aliases, so called tasks, in your `package.json` I got the idea to write
this little tool. It very much does the same as npm in terms of the
task runner thing. But `jarvis` also does more...

## Features
* multiple supported config file formats: `JSON`, `TOML`
* integrates with every project
* simple to setup
* runs blazingly fast and secure because it's written in
[rust](https://rust-lang.org)

## Installation
With Cargo package manager:
```sh
cargo install jarvis
```
or by cloning the git repository:
```sh
git clone https://github.com/moseschmiedel/jarvis.git
cd jarvis
cargo install .
```

## Usage
Just create a `.jarvis.{your-preferred-config-type}` in your project's
root. In there create a subsection called `commands` and then specify
your commands as key-value-pairs where the key is your wished identifier
used to tell `jarvis` which command to run and the value is a string
containing the command that should be called when the identifier is
called with `jarvis`.

You can run `jarvis` just by executing:
```sh
jarvis <command> <args-for-command>
```
For example with the following configuration files, this:
```sh
jarvis hello-world
```
would print
```
Hello, world!
```
in your terminal.

### Example configuration files

`.jarvis.toml`
```toml
[commands]
hello-world = "echo \"Hello, world!\"
```

`.jarvis.json`
```json
{
	"commands": {
		"hello-world": "echo \"Hello, world!\"
	}
}
```

## Contact
Maintainer: Mose Schmiedel <mose.schmiedel@web.de>

Please post Issues in the GitHub Repo:
https://github.com/moseschmiedel/jarvis.git
