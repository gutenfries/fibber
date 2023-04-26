/// external crate imports
extern crate clap;
extern crate once_cell;

/// internal module declarations
pub mod args;
pub mod constants;
pub mod fib;
pub mod options;

use clap::Parser;
pub use options::Verbosity;

use crate::args::Cli;

/// Global configuration object
#[derive(Debug, Default)]
pub struct Config {
	/// log verbosity level
	pub verbosity: options::Verbosity,

	/// the length of the sequence to output, i.e.:
	/// 0, 1, 1, 2, 3, 5 would be the result of -c 6
	pub count: u32,

	/// Print on one line
	pub one_line: bool,

	/// Preface each number in the sequence with it's position within the sequence, i.e:
	/// `1:0, 2:1, 3:1, 4:2, 5:3, 6:5`
	pub numbering: bool,

	/// Print only the last number of the sequence for the given count
	pub last_only: bool,
}

/// program entrypoint
fn main() {
	// parse the command line arguments
	let args = Cli::parse();

	// create the configuration object with the values from the parsed and cleansed cli args
	// data validation matters!!! :)
	let config = Config {
		verbosity: match (args.quiet, args.verbose) {
			(true, _) => Verbosity::Quiet,
			(_, true) => Verbosity::Verbose,
			(false, false) => Verbosity::Normal,
		},
		count: args.count,
		one_line: args.one_line,
		numbering: args.numbering,
		last_only: args.last_only,
	};

	if config.verbosity == Verbosity::Verbose {
		println!("Config: {:?}", config);
	}

	let mom = fib::fibbinacci(5);
	println!("mom: {:?}", mom);
}
